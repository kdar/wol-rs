use std::net::{UdpSocket, ToSocketAddrs};
use std::io::Error;

// fn to_hex_string(bytes: Vec<u8>) -> String {
//   let strs: Vec<String> = bytes.iter()
//     .map(|b| format!("0x{:02x}", b))
//     .collect();
//   strs.join(" ")
// }

pub fn send<A: ToSocketAddrs>(mac: Vec<u8>, bcast_addr: A, bind_addr: A) -> Result<(), Error> {
  let mut packet = vec![0u8; 102];

  // The header is 6 0xFFs
  for i in 0..6 {
    packet[i] = 0xFF;
  }

  // We copy the mac address 16 times.
  for i in 0..16 {
    for j in 0..6 {
      packet[6 + (i * 6) + j] = mac[j];
    }
  }

  let socket = try!(UdpSocket::bind(bind_addr));
  try!(socket.set_broadcast(true));
  try!(socket.send_to(&packet, bcast_addr));

  Ok(())
}
