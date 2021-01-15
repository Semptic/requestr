#[macro_use]
extern crate lazy_static;
extern crate serde_json;
extern crate clap_verbosity_flag;
use log::{debug};
use requestr_core::{load_request_definition, make_request};
use serde_json::Value;
use simplelog::{ConfigBuilder, TermLogger, TerminalMode};
use structopt::StructOpt;
lazy_static! {
    static ref OPT: Opt = Opt::from_args();
}
fn main() {
    let config = ConfigBuilder::new()
      .set_time_to_local(true)
      .build();

    TermLogger::init(OPT.verbose.log_level().unwrap().to_level_filter(), config, TerminalMode::Mixed).unwrap();

    let request_config = load_request_definition(OPT.request_config.as_str()).unwrap();
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
