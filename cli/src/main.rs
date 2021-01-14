#[macro_use]
extern crate lazy_static;
extern crate clap_verbosity_flag;
use log::{debug, error, info, trace, warn};
use requestr_core::hello_world;
use simplelog::{Config, ConfigBuilder, TermLogger, TerminalMode};
use structopt::StructOpt;
lazy_static! {
    static ref OPT: Opt = Opt::from_args();
}
fn main() {
    let config = ConfigBuilder::new()
      .set_time_to_local(true)
      .build();

    TermLogger::init(OPT.verbose.log_level().unwrap().to_level_filter(), config, TerminalMode::Mixed).unwrap();

    error!("Oh nooo!");
    warn!("warn");
    info!("info");
    debug!("debug");
    trace!("Trace");

    hello_world();
}
#[derive(Debug, StructOpt)]
struct Opt {
    #[structopt(flatten)]
    verbose: clap_verbosity_flag::Verbosity,
}
