#[macro_use]
extern crate log;
extern crate parsetree;

use std::iter;
use log::LogLevelFilter;

use parsetree::Tree;
use parsetree::structures::NodeId;
use parsetree::packet::prelude::*;

static PACKET_ETH_IPV4_TCP: &'static [u8] =
    &[0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x08, 0x00, 0x45, 0x00, 0x00, 0x34,
      0x73, 0x22, 0x40, 0x00, 0x3f, 0x06, 0x3a, 0x09, 0x0a, 0x00, 0x00, 0x65, 0x42, 0xc4, 0x41, 0x70, 0xca, 0x45,
      0x01, 0xbb, 0x98, 0x66, 0x5f, 0x0a, 0x44, 0x9d, 0x7f, 0x05, 0x80, 0x10, 0x20, 0x00, 0xbf, 0xf2, 0x00, 0x00,
      0x01, 0x01, 0x08, 0x0a, 0x00, 0x02, 0x2c, 0x2c, 0x63, 0x93, 0xf1, 0x5b];

static PACKET_ETH_IPV6_UDP: &'static [u8] =
    &[0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x86, 0xdd, 0x60, 0x00, 0x00, 0x00,
      0x00, 0x24, 0x11, 0x40, 0x3f, 0xfe, 0x05, 0x07, 0x00, 0x00, 0x00, 0x01, 0x02, 0x00, 0x86, 0xff, 0xfe, 0x05,
      0x80, 0xda, 0x3f, 0xfe, 0x05, 0x01, 0x48, 0x19, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x42,
      0x09, 0x5c, 0x00, 0x35, 0x00, 0x24, 0xf0, 0x09];

static TLS_HEADER: &'static [u8] =
    &[0x16, 0x03, 0x01, 0x00, 0xf4, 0x01, 0x00, 0x00, 0xf0, 0x03, 0x03, 0x14, 0x5b, 0x92, 0xc3, 0xcd, 0x27, 0xe0,
      0xa7, 0x09, 0x1d, 0x3a, 0x14, 0xda, 0x13, 0x8f, 0x19, 0x92, 0x9b, 0x5f, 0xd9, 0x75, 0x34, 0xe7, 0x45, 0xd8,
      0x2d, 0x1c, 0xa9, 0xb0, 0x89, 0x3c, 0xac, 0x20, 0x58, 0x44, 0x00, 0x00, 0x68, 0x46, 0xcb, 0x02, 0xee, 0xfd,
      0x82, 0x22, 0x32, 0x12, 0x89, 0x20, 0x73, 0xbe, 0x5d, 0x4b, 0xdb, 0x0b, 0xe5, 0x2f, 0x2c, 0xf6, 0x41, 0x1f,
      0x27, 0xcb, 0xf1, 0x21, 0x00, 0x20, 0xc0, 0x2b, 0xc0, 0x2f, 0x00, 0x9e, 0xcc, 0x14, 0xcc, 0x13, 0xcc, 0x15,
      0xc0, 0x0a, 0xc0, 0x14, 0x00, 0x39, 0xc0, 0x09, 0xc0, 0x13, 0x00, 0x33, 0x00, 0x9c, 0x00, 0x35, 0x00, 0x2f,
      0x00, 0x0a, 0x01, 0x00, 0x00, 0x87, 0xff, 0x01, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x16, 0x00, 0x14, 0x00,
      0x00, 0x11, 0x61, 0x73, 0x65, 0x63, 0x75, 0x72, 0x69, 0x74, 0x79, 0x73, 0x69, 0x74, 0x65, 0x2e, 0x63, 0x6f,
      0x6d, 0x00, 0x17, 0x00, 0x00, 0x00, 0x23, 0x00, 0x00, 0x00, 0x0d, 0x00, 0x16, 0x00, 0x14, 0x06, 0x01, 0x06,
      0x03, 0x05, 0x01, 0x05, 0x03, 0x04, 0x01, 0x04, 0x03, 0x03, 0x01, 0x03, 0x03, 0x02, 0x01, 0x02, 0x03, 0x00,
      0x05, 0x00, 0x05, 0x01, 0x00, 0x00, 0x00, 0x00, 0x33, 0x74, 0x00, 0x00, 0x00, 0x12, 0x00, 0x00, 0x00, 0x10,
      0x00, 0x1d, 0x00, 0x1b, 0x08, 0x68, 0x74, 0x74, 0x70, 0x2f, 0x31, 0x2e, 0x31, 0x08, 0x73, 0x70, 0x64, 0x79,
      0x2f, 0x33, 0x2e, 0x31, 0x05, 0x68, 0x32, 0x2d, 0x31, 0x34, 0x02, 0x68, 0x32, 0x75, 0x50, 0x00, 0x00, 0x00,
      0x0b, 0x00, 0x02, 0x01, 0x00, 0x00, 0x0a, 0x00, 0x06, 0x00, 0x04, 0x00, 0x17, 0x00, 0x18];

fn get_default_tree() -> Tree<Layer, ParserVariant> {
    // Create a tree
    let mut tree = Tree::new();
    tree.set_log_level(LogLevelFilter::Trace);

    // Create some parsers
    let eth = tree.new_parser(EthernetParser);
    let ipv4 = tree.new_parser(Ipv4Parser);
    let ipv6 = tree.new_parser(Ipv6Parser);

    let tcp_ipv4 = tree.new_parser(TcpParser);
    let tcp_ipv6 = tree.new_parser(TcpParser);

    let udp_ipv4 = tree.new_parser(UdpParser);
    let udp_ipv6 = tree.new_parser(UdpParser);

    let tls_ipv4 = tree.new_parser(TlsParser);
    let tls_ipv6 = tree.new_parser(TlsParser);

    // Connect the parsers
    tree.link(eth, ipv4);
    tree.link(eth, ipv6);

    tree.link(ipv4, tcp_ipv4);
    tree.link(ipv4, udp_ipv4);

    tree.link(tcp_ipv4, tls_ipv4);
    tree.link(tcp_ipv6, tls_ipv6);

    tree.link(ipv6, tcp_ipv6);
    tree.link(ipv6, udp_ipv6);

    info!("The tree looks like:");
    info!("- Eth");
    log_children(&tree, eth, 0);

    tree
}

fn log_children(tree: &Tree<Layer, ParserVariant>, node: NodeId, mut level: usize) {
    level += 2;
    for child in node.children(&tree.arena) {
        let indent = iter::repeat(' ').take(level).collect::<String>();
        let ref parser = tree.arena[child].data;
        info!("{}- {}", indent, parser.variant());
        log_children(tree, child, level);
    }
}

#[test]
fn tcp() {
    let tree = get_default_tree();
    let result = tree.traverse(PACKET_ETH_IPV4_TCP, vec![]);
    info!("Result [ETH, IPV4, TCP]: {:?}", result);
}

#[test]
fn tls() {
    let tree = get_default_tree();
    let mut packet = Vec::from(PACKET_ETH_IPV4_TCP);
    packet.extend_from_slice(TLS_HEADER);
    let result = tree.traverse(&packet, vec![]);
    info!("Result [ETH, IPV4, TCP, TLS]: {:?}", result);
}

#[test]
fn udp() {
    let tree = get_default_tree();
    let result = tree.traverse(PACKET_ETH_IPV6_UDP, vec![]);
    info!("Result [ETH, IPV6, UDP]: {:?}", result);
}

