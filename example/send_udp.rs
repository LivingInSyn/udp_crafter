extern crate udp_crafter;

use std::net::Ipv4Addr;
use udp_crafter::CraftedUdp;

fn main() {
    let sampledata = vec![1,2,3,4,5];
    let udp_to_send = CraftedUdp {
        source_ip: Ipv4Addr::new(192,168,1,101),
        dest_ip: Ipv4Addr::new(127,0,0,1),
        source_port: 12345,
        dest_port: 55555,
        data: sampledata
    };
    udp_to_send.send_packet().unwrap();
}