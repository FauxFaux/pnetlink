use packet::netlink::{MutableNetlinkPacket, NetlinkPacket};
use packet::netlink::NetlinkMsgFlags;
use packet::netlink::{NetlinkBufIterator, NetlinkReader, NetlinkRequestBuilder};
use socket::{NetlinkProtocol, NetlinkSocket};
use packet::netlink::NetlinkConnection;
use pnet::packet::MutablePacket;
use pnet::packet::Packet;
use pnet::packet::PacketSize;
use libc;
use std::io::{Read, Write};

include!(concat!(env!("OUT_DIR"), "/audit/audit.rs"));

pub trait Audit
where
    Self: Read + Write,
{
    fn audit_enable<'a>(&'a mut self) -> ::std::io::Result<()>;
}

impl Audit for NetlinkConnection {
    fn audit_enable<'a>(&'a mut self) -> ::std::io::Result<()> {
        let mut buf = vec![0; MutableAuditStatusPacket::minimum_packet_size()];
        let req = NetlinkRequestBuilder::new(
            1001,
            NetlinkMsgFlags::NLM_F_REQUEST | NetlinkMsgFlags::NLM_F_ACK,
        ).append({
            let mut status = MutableAuditStatusPacket::new(&mut buf).unwrap();
            status.set_mask(1);
            status.set_enabled(1);
            status
        })
            .build();
        self.write(req.packet())?;
        let reader = NetlinkReader::new(self);
        reader.read_to_end()
    }
}
