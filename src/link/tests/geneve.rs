// SPDX-License-Identifier: MIT

use std::net::Ipv6Addr;
use std::str::FromStr;

use netlink_packet_utils::{Emitable, Parseable};

use crate::link::link_flag::LinkFlags;
use crate::link::{
    GeneveDf, InfoData, InfoGeneve, InfoKind, LinkAttribute, LinkHeader,
    LinkInfo, LinkLayerType, LinkMessage, LinkMessageBuffer,
};
use crate::AddressFamily;

#[test]
fn test_geneve_link_info() {
    let raw: Vec<u8> = vec![
        0x00, 0x00, 0xfe, 0xff, 0xc0, 0x69, 0x00, 0x00, 0x90, 0x00, 0x00, 0x00,
        0x00, 0x00, 0x00, 0x00, 0x74, 0x00, 0x12, 0x00, 0x0b, 0x00, 0x01, 0x00,
        0x67, 0x65, 0x6e, 0x65, 0x76, 0x65, 0x00, 0x00, 0x64, 0x00, 0x02, 0x00,
        0x08, 0x00, 0x01, 0x00, 0x2a, 0x00, 0x00, 0x00, 0x14, 0x00, 0x07, 0x00,
        0x20, 0x01, 0x0d, 0xb8, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
        0x00, 0x00, 0x00, 0x01, 0x05, 0x00, 0x09, 0x00, 0x00, 0x00, 0x00, 0x00,
        0x05, 0x00, 0x03, 0x00, 0x0a, 0x00, 0x00, 0x00, 0x05, 0x00, 0x04, 0x00,
        0x12, 0x00, 0x00, 0x00, 0x08, 0x00, 0x0b, 0x00, 0x00, 0x01, 0xe2, 0x40,
        0x05, 0x00, 0x0d, 0x00, 0x01, 0x00, 0x00, 0x00, 0x06, 0x00, 0x05, 0x00,
        0x11, 0x5c, 0x00, 0x00, 0x05, 0x00, 0x0a, 0x00, 0x00, 0x00, 0x00, 0x00,
        0x05, 0x00, 0x0c, 0x00, 0x00, 0x00, 0x00, 0x00, 0x04, 0x00, 0x0e, 0x00,
    ];

    let expected = LinkMessage {
        header: LinkHeader {
            interface_family: AddressFamily::Unspec,
            index: 27072,
            link_layer_type: LinkLayerType::None,
            flags: LinkFlags::Pointopoint | LinkFlags::Noarp,
            change_mask: LinkFlags::empty(),
        },
        attributes: vec![LinkAttribute::LinkInfo(vec![
            LinkInfo::Kind(InfoKind::Geneve),
            LinkInfo::Data(InfoData::Geneve(vec![
                InfoGeneve::Id(42),
                InfoGeneve::Remote6(Ipv6Addr::from_str("2001:db8::1").unwrap()),
                InfoGeneve::UdpZeroCsum6Tx(false),
                InfoGeneve::Ttl(10),
                InfoGeneve::Tos(18),
                InfoGeneve::Label(123456),
                InfoGeneve::Df(GeneveDf::Set),
                InfoGeneve::Port(4444),
                InfoGeneve::UdpZeroCsum6Rx(false),
                InfoGeneve::TtlInherit(false),
                InfoGeneve::InnerProtoInherit,
            ])),
        ])],
    };

    assert_eq!(
        expected,
        LinkMessage::parse(&LinkMessageBuffer::new(&raw)).unwrap()
    );

    let mut buf = vec![0; expected.buffer_len()];

    expected.emit(&mut buf);

    assert_eq!(buf, raw);
}
