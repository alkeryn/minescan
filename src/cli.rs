use crate::scan;
use futures::{stream, StreamExt};
use async_trait::async_trait;

#[derive(clap::Args, Debug, Clone)]
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

#[async_trait]
pub trait Writer {
    async fn handle(&self,result: std::io::Result<String>, ip: String, port: u16);
}

pub type ReaderRet = std::pin::Pin<Box<dyn futures::Stream<Item = String>>>;

#[async_trait]
pub trait Reader {
    type ErrorType;
    async fn get_stream(&mut self) -> std::result::Result<ReaderRet, Self::ErrorType>;
}

#[derive(Clone)]
pub struct ReadWriter<'a> {
    pub args: DefaultArgs,
    pub cmd: clap::App<'a>
}

impl<'a> ReadWriter<'a> {
    pub fn new(args: DefaultArgs, cmd: clap::App<'a>) -> Self {
        Self { args: args, cmd: cmd }
    }
}

#[async_trait]
impl Writer for ReadWriter<'_> {
    async fn handle(&self,result: std::io::Result<String>, ip: String, _port: u16) {
        let args = &self.args;
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

#[async_trait]
impl Reader for ReadWriter<'_> {
    type ErrorType = std::io::Error;
    async fn get_stream(&mut self) -> std::result::Result<ReaderRet, Self::ErrorType> {
        let args = &self.args;
        let cmd = &mut self.cmd;
        if let Some(address) = args.address.clone() {
            return Ok(stream::iter([address]).boxed())
        }
        else {
            if atty::is(atty::Stream::Stdin) {
                cmd.print_help()?;
                return Err(std::io::Error::new(std::io::ErrorKind::Other, "No Stdin"))
            }

            let lines = std::io::stdin().lines();
            let iter = stream::iter(lines);
            let iter = iter.filter_map( |x| async {
                match x {
                    Ok(x) => Some(x),
                    Err(_) => None
                }
            });
            return Ok(iter.boxed_local())
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

pub async fn run<R: Reader>(writer: impl Writer, mut reader: R, args: &DefaultArgs) -> Result<(), R::ErrorType> {
    let iter = reader.get_stream().await?;
    run_stream(writer, iter, args).await;
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
        writer.handle(result,ip,port).await;
}
