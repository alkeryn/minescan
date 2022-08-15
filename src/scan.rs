use std::io::prelude::*;
use std::net::{TcpStream,ToSocketAddrs};
use std::time::Duration;

trait ToString {
    fn to_string(&self) -> String;
}

impl ToString for [u8] {
    fn to_string(&self) -> String {
        unsafe { String::from_utf8_unchecked(self.to_vec()) }
    }
}

// fn bytes_to_string(data: &[u8]) -> String{
//     unsafe { String::from_utf8_unchecked(data.to_vec()) }
// }

fn pack_data(data: String) -> String {

    let len = data.len() as u8; // should be varint
    let nstring = len.to_be_bytes().to_string();
    nstring + &data
}

#[allow(dead_code)]
fn read_n_bytes(stream: &mut TcpStream, n: usize) -> std::io::Result<String> {

    let mut buffer = vec![0u8;n];
    let mut string = String::new();

    stream.read_exact(&mut buffer)?; // need exact to not go over
    string += &buffer.to_string();

    Ok(string)
}

fn read_varint(stream: &mut TcpStream) -> std::io::Result<usize> {
    let mut d : usize = 0;
    let mut buffer = [0u8;1];
    for i in 0..5 as usize {
        stream.read_exact(&mut buffer)?;
        let b = buffer[0] as usize;
        d |= (b & 0x7F) << 7*i;
        if (b & 0x80) == 0 {
            return Ok(d)
        }
    }
    return Err(std::io::Error::new(std::io::ErrorKind::Other, "VarInt too big!"))

}

pub fn scanip(ip: String, port: Option<u16>) -> std::io::Result<String>{ // 1.7+

    let port : u16 = port.unwrap_or(25565);
    let address = [ip.clone(),port.to_string()].join(":");
    let timeout = 500;
    const MAXLENGHT : usize = usize::pow(2,13);

    let sockaddr : Vec<_> = address.to_socket_addrs().unwrap().collect();
    let mut stream = TcpStream::connect_timeout(&sockaddr[sockaddr.len() - 1], Duration::from_millis(timeout))?;
    stream.set_read_timeout(Some(Duration::from_millis(timeout)))?;
    stream.set_write_timeout(Some(Duration::from_millis(timeout)))?;
    // println!("address: {}", sockaddr[sockaddr.len() - 1]);

    let port = port.to_be_bytes().to_string();

    let payload = pack_data(String::from("\x00\x00".to_owned() + &pack_data(ip.to_owned()) + &port + "\x01"));
    stream.write_all(payload.as_bytes())?;
    stream.write_all(pack_data("\x00".to_owned()).as_bytes())?;

    let _packet_lenght = read_varint(&mut stream)?;
    let _packet_id = read_varint(&mut stream)?;
    let lenght = read_varint(&mut stream)?;
    // println!("{} {} {}", _packet_lenght, _packet_id, lenght);

    let mut firstchar = [0u8;1];
    stream.read_exact(&mut firstchar)?;
    let mut reply = String::new();

    if firstchar[0] == b'{' && lenght < MAXLENGHT {
        reply.push(firstchar[0] as char);
        let mut buffer = vec![0u8;lenght - 1];
        stream.read_exact(&mut buffer)?;
        reply += &buffer.to_string();
    }

    Ok(reply)
}

#[allow(dead_code)]
pub fn scanip_old(address: String) -> std::io::Result<String>{ // legacy up to 1.6
    // let sockaddr : std::net::SocketAddr = address.parse().expect("cannot parse address");
    let sockaddr : Vec<_> = address.to_socket_addrs().unwrap().collect();
    let mut stream = TcpStream::connect_timeout(&sockaddr[sockaddr.len() - 1], Duration::from_secs(1) )?;
    println!("address: {}", sockaddr[0]);

    stream.write(b"\xfe\x01\x1a")?;
    // let mut reply  = String::new();
    // stream.read_to_string(&mut reply)?;

    let mut reply2  = [0u8;4096];
    stream.read(&mut reply2)?;
    println!("{}",String::from_utf8_lossy(&reply2));

    Ok(String::from(""))
}
