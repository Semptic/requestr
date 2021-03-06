#[macro_use]
extern crate lazy_static;
extern crate ansi_term;
extern crate clap_verbosity_flag;
extern crate loggerv;
extern crate serde_json;
use anyhow::{Context, Result};
use std::path::PathBuf;
use std::{
    collections::{BTreeMap, HashMap},
    fs,
};

use log::debug;
use requestr_core::{
    load_request_definition, load_request_template, make_request, validate_parameter,
};
use serde_json::Value;
use structopt::StructOpt;

fn main() -> Result<()> {
    lazy_static! {
        static ref OPT: Opt = Opt::from_args();
    }

    // Add the following line near the beginning of the main function for an application to enable
    // colorized output on Windows 10.
    //
    // Based on documentation for the ansi_term crate, Windows 10 supports ANSI escape characters,
    // but it must be enabled first using the `ansi_term::enable_ansi_support()` function. It is
    // conditionally compiled and only exists for Windows builds. To avoid build errors on
    // non-windows platforms, a cfg guard should be put in place.
    #[cfg(windows)]
    ansi_term::enable_ansi_support().unwrap();

    loggerv::Logger::new()
        .max_level(
            OPT.verbosity
                .log_level()
                .context("Failed to get log level")?,
        )
        .level(OPT.debug)
        .module_path(OPT.debug)
        .line_numbers(OPT.debug)
        .init()
        .unwrap();

    debug!("{:#?}", *OPT);

    let mut parameter = params_to_map(&OPT.parameter);

    if let Some(env) = &OPT.env {
        let contents = fs::read_to_string(env).with_context(|| {
            format!(
                "Failed loading environment config from {}",
                env.to_string_lossy()
            )
        })?;
        let deserialized_map: BTreeMap<String, String> =
            serde_yaml::from_str(&contents).context("Failed to parse environment config")?;

        for (key, value) in deserialized_map {
            parameter.insert(key, value);
        }
    }

    let template = load_request_template(&OPT.request_config)
        .context("Failed loading request config template")?;

    validate_parameter(&template, &parameter).context("Failed to validate Parameter")?;

    let request_config = load_request_definition(&template, &parameter)
        .context("Failed to create request definition")?;

    let response = make_request(
        request_config.url.as_str(),
        request_config.body,
        request_config.method,
        request_config.header,
    )
    .context("Request failed")?;

    let obj: Result<Value> =
        serde_json::from_str(response.as_str()).context("Failed to parse response into json");

    match obj {
        Ok(obj) => println!(
            "{}",
            serde_json::to_string_pretty(&obj).context("Failed to prettify response")?
        ),
        Err(_) => println!("{}", response.as_str()),
    };

    Ok(())
}

fn params_to_map(args: &[String]) -> HashMap<String, String> {
    args.iter()
        .filter_map(|item| {
            let mut parts = item.splitn(2, '='); // Split into 2 parts.
            let key = parts.next()?;
            let value = parts.next()?;

            Some((key.to_string(), value.to_string()))
        })
        .collect()
}
#[derive(Debug, StructOpt)]
#[structopt(
    name = "requestr",
    about = "Store, share and run http request templates easily."
)]
struct Opt {
    #[structopt(flatten)]
    verbosity: clap_verbosity_flag::Verbosity,

    /// Path to the request definition.
    #[structopt(parse(from_os_str))]
    request_config: PathBuf,

    /// Parameters you want to pass to your request definition. In the form of `key=value` (e.g title='My title').
    #[structopt(short, long)]
    parameter: Vec<String>,

    /// Enables debug mode
    #[structopt(short, long)]
    debug: bool,

    /// Path to environment configuration to set multiple variables at once. This can be used to quickly switch
    /// between test and production environments.
    #[structopt(parse(from_os_str), short, long)]
    env: Option<PathBuf>,
}
