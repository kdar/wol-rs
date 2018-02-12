wol-rs
======

Send a Wake-on-lan magic packet.

## Usage

```
extern crate wol;

wol::send(
  vec![0x01, 0x20, 0x44, 0x40, 0xf2, 0xff], // The MAC address you're targeting
  "255.255.255.255:9", // The UDP broadcast address
  "0.0.0.0:0" // The address to listen on
);
```
