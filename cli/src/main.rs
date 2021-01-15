#[macro_use]
extern crate lazy_static;
extern crate serde_json;
extern crate clap_verbosity_flag;
use std::collections::HashMap;

use log::{debug};
use requestr_core::{load_request_definition, load_request_template, make_request, validate_parameter};
use serde_json::Value;
use simplelog::{ConfigBuilder, TermLogger, TerminalMode};
use structopt::StructOpt;

fn main() {
    lazy_static! {
        static ref OPT: Opt = Opt::from_args();
    }

    let config = ConfigBuilder::new()
      .set_time_to_local(true)
      .build();

    TermLogger::init(OPT.verbose.log_level().unwrap().to_level_filter(), config, TerminalMode::Mixed).unwrap();

    let mut parameter: HashMap<String, String> = HashMap::new();
    parameter.insert("title".to_string(), "title".to_string());
    parameter.insert("body".to_string(), "body".to_string());
    parameter.insert("id".to_string(), "1".to_string());

    let template = load_request_template(OPT.request_config.as_str()).unwrap();

    validate_parameter(&template, &parameter).unwrap();

    let request_config = load_request_definition(&template, &parameter).unwrap();
    debug!("{:?}", request_config);

    let response = make_request(request_config.url.as_str(), request_config.body, request_config.method).unwrap();

    let obj: Value = serde_json::from_str(response.as_str()).unwrap();

    println!("{}", serde_json::to_string_pretty(&obj).unwrap());
}
#[derive(Debug, StructOpt)]
struct Opt {

    #[structopt(flatten)]
    verbose: clap_verbosity_flag::Verbosity,

    request_config: String
}
