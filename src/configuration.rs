use structopt::StructOpt;

#[derive(Debug, StructOpt)]
#[structopt(about = "listen a port and print request")]
pub struct CmdArgs {
    #[structopt(short = "p", long, help = "resource full url to fetch")]
    pub port: u16,
}
