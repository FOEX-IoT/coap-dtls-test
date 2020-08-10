use crate::udp::UDPWrapper;
use openssl::ssl::{SslConnector, SslMethod};
use std::io::{Read, Write};
use std::net::UdpSocket;

const KEY: &'static str = "1234";
const ID: &'static str = "CoaP";

mod udp;

fn main() {
  let mut builder = SslConnector::builder(SslMethod::dtls()).unwrap();

  builder.set_psk_client_callback(move |_ssl, _hint, mut identity_buffer, mut psk_buffer| {
    identity_buffer.write_all(ID.as_bytes()).unwrap();
    psk_buffer.write_all(KEY.as_bytes()).unwrap();

    Ok(KEY.len())
  });

  let connector = builder.build();
  let socket = UdpSocket::bind("127.0.0.1:34254").unwrap();
  let wrapper = UDPWrapper::new(socket);

  if let Err(e) = connector.connect("127.0.0.1:5684", wrapper) {
    println!("{:}", e);
  }
  // let stream = TcpStream::connect("google.com:443").unwrap();
  // let mut stream = connector.connect("google.com", stream).unwrap();
}
