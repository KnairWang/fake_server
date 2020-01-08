mod app;
mod configuration;
mod handlers;

use crate::configuration::CmdArgs;

use structopt::StructOpt;

fn main() {
    let cmd_args = CmdArgs::from_args();
    app::start_server(cmd_args.port).unwrap();
}
