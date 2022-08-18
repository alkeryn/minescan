use clap::Parser;
use clap::CommandFactory;
mod scan;
mod cli;

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
pub struct Args {
    #[clap(flatten)]
    default: cli::DefaultArgs,
}

#[tokio::main]
async fn main() -> std::io::Result<()> {
    let args = Args::parse();

    let writer = cli::DefaultWriter {};
    cli::run(&args.default, writer, &mut Args::command()).await
}
