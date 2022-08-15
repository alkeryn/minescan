use std::env;
mod scan;

// #![feature(concat_bytes)]

fn main() {
    let args: Vec<String> = env::args().collect();

    // let narg = &args[1..];
    // let out = scanip(narg.join(":")).expect("failed");

    let split : Vec<&str> = args[1].split(":").collect();
    let ip = split[0];
    let port : u16;
    if split.len() > 1 {
        port  = split[1].parse().unwrap_or(25565);
    }
    else {
        port = 25565;
    }

    match scan::scanip(ip.to_owned(), Some(port)) {
        Ok(d) => {
            if d.len() > 0 {
                println!("IP {}:\n{}",ip,d)
            }
        },
        Err(_) => return
    };
    // println!("Hello, world! {:?}", out);
}
