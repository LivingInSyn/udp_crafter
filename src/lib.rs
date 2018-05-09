extern crate pnet;

use std::io::{Error, ErrorKind};
use std::net::{Ipv4Addr};
use pnet::datalink::{NetworkInterface};
use pnet::packet::ipv4::{MutableIpv4Packet};
use pnet::packet::udp::MutableUdpPacket;
use pnet::packet::ip::{IpNextHeaderProtocols};
use pnet::packet::{MutablePacket};
//transport use
// use pnet::transport::{transport_channel, TransportSender};
// use pnet::transport::TransportProtocol::Ipv4;
// use pnet::transport::TransportChannelType::Layer4;
//use pnet_datalink::{NetworkInterface};


//testing
use pnet::transport::transport_channel;
use pnet::transport::TransportChannelType::Layer3;
use std::net::IpAddr;

pub struct CraftedUdp {
    pub interface: NetworkInterface,
    pub source_ip: Ipv4Addr,
    pub dest_ip: Ipv4Addr,
    pub source_port: u16,
    pub dest_port: u16,
    pub data: Vec<u8>,
}
impl CraftedUdp {
    pub fn send_packet(&self) -> Result<(), std::io::Error> {
        let protocol = Layer3(IpNextHeaderProtocols::Udp);
        //create a channel
        let (mut tx, _) = transport_channel(4096, protocol)?;
        //create UDP datagram
        let mut udpbuffer = [0u8;1024];
        let mut udppacket = match MutableUdpPacket::new(&mut udpbuffer) {
            Some(p) => p,
            None => {
                return Err(Error::new(ErrorKind::Other, "Couldn't create UDP packet"));
            }
        };
        //setup UDP packet
        udppacket.set_source(self.source_port);
        udppacket.set_destination(self.dest_port);
        udppacket.set_length((8 + self.data.len()) as u16);
        udppacket.set_checksum(0);
        udppacket.set_payload(&self.data);
        //create IP packet
        let mut ipv4buffer = [0u8; 1024];
        let mut ipv4packet = MutableIpv4Packet::new(&mut ipv4buffer).unwrap();
        //set source and dest
        ipv4packet.set_source(self.source_ip);
        ipv4packet.set_destination(self.dest_ip);
        //set len
        let totallen = (20 + 8 + self.data.len()) as u16;
        ipv4packet.set_total_length(totallen);
        //other fields
        ipv4packet.set_version(4);
        ipv4packet.set_header_length(5);
        ipv4packet.set_dscp(0);
        ipv4packet.set_ecn(0);
        ipv4packet.set_identification(0x1234);
        ipv4packet.set_flags(0);
        ipv4packet.set_fragment_offset(0);
        ipv4packet.set_ttl(64);
        ipv4packet.set_next_level_protocol(IpNextHeaderProtocols::Udp);
        //set udp packet to payload of ipv4 packet
        let udplen = udppacket.get_length() as usize;
        let udppacketmut = udppacket.packet_mut();
        ipv4packet.set_payload(&udppacketmut[0..udplen]);
        //send the packet
        match tx.send_to(ipv4packet, IpAddr::V4(self.dest_ip)) {
            Err(e) => Err(e),
            _ => Ok(())
        }
    }
}
