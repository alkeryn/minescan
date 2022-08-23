use crate::scan;
use futures::{stream, StreamExt};

#[derive(clap::Args, Debug)]
pub struct DefaultArgs {
    /// Address
    pub address: Option<String>,

    /// timeout in ms
    #[clap(short, long, value_parser, default_value_t = 500)]
    pub timeout: u64,

    /// concurency
    #[clap(short, long, value_parser, default_value_t = 100)]
    pub concurency: usize,

    /// verbose
    #[clap(short, long, value_parser, default_value_t = false)]
    pub verbose: bool,
}

pub trait Writer {
    fn handle(&self,result: std::io::Result<String>, ip: String, port: u16, args: &DefaultArgs);
}

pub struct DefaultWriter {}

impl Writer for DefaultWriter {
    fn handle(&self,result: std::io::Result<String>, ip: String, _port: u16, args: &DefaultArgs) {
        match result {
            Ok(d) => {
                println!("IP {}:\n{}",ip,d.replace('\n',"")) // i don't want newlines for parsing
            },
            Err(e) => {
                if args.verbose {
                    eprintln!("IP: {}\n{}",ip, e)
                }
            }
        }
    }
}

fn toaddr(address: String) -> (String,u16) {
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

pub async fn run(writer: impl Writer, args: &DefaultArgs, cmd: &mut clap::App<'_>) -> std::io::Result<()> {

    if let Some(address) = args.address.clone() {
        run_block(&writer, address, args).await;
    }
    else {
        if atty::is(atty::Stream::Stdin) {
            cmd.print_help()?;
            return Ok(());
        }

        let lines = std::io::stdin().lines();
        let iter = stream::iter(lines);
        let iter = iter.filter_map( |x| async {
            match x {
                Ok(x) => Some(x),
                Err(_) => None
            }
        });

        run_stream(writer, iter, args).await;
    }
    Ok(())
}

#[inline(always)]
pub async fn run_stream(writer: impl Writer, iter: impl futures::Stream<Item = String>, args: &DefaultArgs) {
    iter.for_each_concurrent(args.concurency, | address | async {
        run_block(&writer, address, args).await;
    }).await;
}

#[inline(always)]
pub async fn run_block(writer: &impl Writer, address: String, args: &DefaultArgs) {
        let (ip,port) = toaddr(address);
        let result = scan::scanip_timeout(ip.clone(), Some(port), Some(args.timeout)).await;
        writer.handle(result,ip,port,args);
}
