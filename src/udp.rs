use std::io::{Read, Result, Write};
use std::net::UdpSocket;

#[derive(Debug)]
pub struct UDPWrapper(UdpSocket);

impl UDPWrapper {
  pub fn new(udp: UdpSocket) -> Self {
    UDPWrapper(udp)
  }
}

impl Read for UDPWrapper {
  fn read(&mut self, buf: &mut [u8]) -> Result<usize> {
    self.0.recv(buf)
  }
}

impl Write for UDPWrapper {
  fn write(&mut self, buf: &[u8]) -> Result<usize> {
    self.0.send(buf)
  }
  fn flush(&mut self) -> Result<()> {
    Ok(())
  }
}
