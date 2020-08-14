extern crate coap;
use coap::dtls_client::DTLSCoAPClient;
use coap::message::header::RequestType as Method;
use coap::message::packet::Packet;
use coap::message::IsMessage;
use coap::CoAPRequest;
use std::net::{Ipv4Addr, SocketAddr, SocketAddrV4};

fn auth() {
  let peer = "coaps://192.168.1.11:5684/15011/9063";

  // let (domain, port, path) = DTLSCoAPClient::parse_coap_url(peer).unwrap();
  let path = "/15011/9063";

  let payload = r#"{"9090":"asdasd"}"#;

  let mut request = CoAPRequest::new();

  request.set_method(Method::Post);
  request.set_path(path);
  request.set_payload(Vec::from(payload.as_bytes()));

  let mut client = DTLSCoAPClient::new(("192.168.1.11", 5674)).unwrap();
  client.send(&request).unwrap();

  match client.receive() {
    Ok(receive_packet) => {
      println!(
        "Server reply: {}",
        String::from_utf8(receive_packet.message.payload).unwrap()
      );
    }
    _ => (),
  };
}

fn main() {
  let peer = "coaps://192.168.1.11:5684/15001/65537";

  // let (domain, port, path) = DTLSCoAPClient::parse_coap_url(peer).unwrap();
  let response = DTLSCoAPClient::get(peer).unwrap();
  println!(
    "Server reply: {:}",
    String::from_utf8(response.message.payload).unwrap()
  );
}
