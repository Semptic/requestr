#[macro_use]
extern crate lazy_static;
extern crate clap_verbosity_flag;
extern crate serde_json;
use std::{
    collections::{BTreeMap, HashMap},
    fs,
};

use log::debug;
use requestr_core::{
    load_request_definition, load_request_template, make_request, validate_parameter,
};
use serde_json::Value;
use serde_yaml;
use simplelog::{ConfigBuilder, TermLogger, TerminalMode};
use structopt::StructOpt;

fn main() {
    lazy_static! {
        static ref OPT: Opt = Opt::from_args();
    }

    let config = ConfigBuilder::new().set_time_to_local(true).build();

    TermLogger::init(
        OPT.verbose.log_level().unwrap().to_level_filter(),
        config,
        TerminalMode::Mixed,
    )
    .unwrap();

    debug!("{:#?}", *OPT);

    let mut parameter = params_to_map(&OPT.parameter);

    if let Some(env) = &OPT.env {
        let contents = fs::read_to_string(env).unwrap();
        let deserialized_map: BTreeMap<String, String> = serde_yaml::from_str(&contents).unwrap();

        for (key, value) in deserialized_map {
            parameter.insert(key, value);
        }
    }

    let template = load_request_template(OPT.request_config.as_str()).unwrap();

    validate_parameter(&template, &parameter).unwrap();

    let request_config = load_request_definition(&template, &parameter).unwrap();

    let response = make_request(
        request_config.url.as_str(),
        request_config.body,
        request_config.method,
        request_config.header,
    )
    .unwrap();

    let obj: Value = serde_json::from_str(response.as_str()).unwrap();

    println!("{}", serde_json::to_string_pretty(&obj).unwrap());
}

fn params_to_map(args: &Vec<String>) -> HashMap<String, String> {
    args.into_iter()
        .filter_map(|item| {
            let mut parts = item.splitn(2, '='); // Split into 2 parts.
            let key = parts.next()?;
            let value = parts.next()?;

            Some((key.to_string(), value.to_string()))
        })
        .collect()
}
#[derive(Debug, StructOpt)]
struct Opt {
    #[structopt(flatten)]
    verbose: clap_verbosity_flag::Verbosity,

    request_config: String,

    #[structopt(short, long)]
    parameter: Vec<String>,

    #[structopt(short, long)]
    env: Option<String>,
}
