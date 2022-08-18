use std::env;
use clap::{Parser, CommandFactory};
use futures::{stream, StreamExt};
mod scan;

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
    /// Address
    address: Option<String>,

    /// timeout in ms
    #[clap(short, long, value_parser, default_value_t = 500)]
    timeout: u64,

    /// concurency
    #[clap(short, long, value_parser, default_value_t = 100)]
    concurency: usize,

    /// verbose
    #[clap(short, long, value_parser, default_value_t = false)]
    verbose: bool,
}

// #![feature(concat_bytes)]

fn toaddr(address: String) -> (String, u16) {
    let split : Vec<&str> = address.split(":").collect();
    let ip = split[0];
    let port : u16;
    if split.len() > 1 {
        port  = split[1].parse().unwrap_or(25565);
    }
    else {
        port = 25565;
    }
    (ip.to_owned(),port)
}

fn handle(result: std::io::Result<String>, addr: String, args: &Args) {
    match result {
        Ok(d) => {
            println!("IP {}:\n{}",addr,d)
        },
        Err(e) => {
            if args.verbose {
                eprintln!("IP: {}\n{}",addr, e)
            }
        }
    }
}

#[tokio::main]
async fn main() -> std::io::Result<()> {
    let args = Args::parse();

    if let Some(address) = args.address.clone() {
        let addr = toaddr(address);
        let result = scan::scanip_timeout(addr.0.clone(), Some(addr.1), Some(args.timeout)).await;
        handle(result,addr.0,&args);
    }
    else {
        if atty::is(atty::Stream::Stdin) {
            let mut cmd = Args::command();
            cmd.print_help()?;
            return Ok(());
        }

        let lines = std::io::stdin().lines();
        let iter = stream::iter(lines);

        iter.for_each_concurrent(args.concurency, | address | async {
            if let Ok(address) = address {
                let addr = toaddr(address);
                let result = scan::scanip_timeout(addr.0.clone(), Some(addr.1), Some(args.timeout)).await;
                handle(result,addr.0,&args);
            }
        }).await;
    }

    // println!("Hello, world! {:?}", out);
    Ok(())
}
