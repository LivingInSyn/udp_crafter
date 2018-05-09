# udp_crafter

udp_crafter is a small crate for crafting UDP packets in Rust. It is build on [pnet](https://github.com/libpnet/libpnet). It's sole purpose it to make sending crafted UDP packets more simple.

## Sample
    let sampledata = vec![1,2,3,4,5];
    let udp_to_send = CraftedUdp {
        source_ip: Ipv4Addr::new(192,168,1,101),
        dest_ip: Ipv4Addr::new(127,0,0,1),
        source_port: 12345,
        dest_port: 55555,
        data: sampledata
    };
    udp_to_send.send_packet().unwrap();