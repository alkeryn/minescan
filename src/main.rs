use clap::{Parser,CommandFactory};
mod scan;
mod cli;

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
    #[clap(flatten)]
    default: cli::DefaultArgs,
}

#[tokio::main]
async fn main() -> std::io::Result<()> {
    let args = Args::parse();

    let rw = cli::ReadWriter::new(args.default.clone(),Args::command());
    let _ = cli::run(rw.clone(), rw, &args.default).await;
    Ok(())
}
