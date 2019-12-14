use crate::dns::buffer::{BytePacketBuffer, PacketBuffer, VectorPacketBuffer};
use crate::dns::protocol::{DnsPacket, ResultCode};
use crate::dns::zone::Zone;
use std::io;
use std::net::SocketAddrV4;
use tokio::net::UdpSocket;

pub struct Server<'a> {
    socket: UdpSocket,
    zones: Vec<Zone<'a>>,
}

impl<'a> Server<'a> {
    pub async fn new(addr: &SocketAddrV4, zones: Vec<Zone<'a>>) -> Result<Server<'a>, io::Error> {
        Ok(Server {
            socket: UdpSocket::bind(addr).await?,
            zones,
        })
    }

    pub async fn run(mut self) -> Result<(), io::Error> {
        loop {
            // Read
            let mut req_buffer = BytePacketBuffer::new();
            let (_, src) = self.socket.recv_from(&mut req_buffer.buf).await?;

            // Parse
            let req = match DnsPacket::from_buffer(&mut req_buffer) {
                Ok(v) => v,
                Err(e) => {
                    println!("Failed to parse UDP query packet: {:?}", e);
                    continue;
                }
            };

            // Query
            let mut res = self.query(&req);

            // Encode
            let mut res_buffer = VectorPacketBuffer::new();
            let _ = res.write(&mut res_buffer, 512);
            let data = res_buffer.get_range(0, res_buffer.pos()).unwrap(); // get_range doesn't fail for VectorPacketBuffer

            // Send
            self.socket.send_to(&data, src).await?;
        }
    }

    pub fn query(&self, req: &DnsPacket) -> DnsPacket {
        let mut packet = DnsPacket::new();
        packet.header.id = req.header.id;
        packet.header.authoritative_answer = true;
        packet.header.recursion_available = false;
        packet.header.response = true;

        if req.header.recursion_desired {
            packet.header.rescode = ResultCode::REFUSED;
        } else if req.questions.is_empty() {
            packet.header.rescode = ResultCode::FORMERR;
        } else {
            // By convention there's only ever one question
            let question = &req.questions[0];
            packet.questions.push(question.clone());

            // Answer with first matching zone
            if let Some(zone) = self.zones.iter().find(|z| z.in_zone(&question.name)) {
                match zone.answer(&question) {
                    Ok(Some(rec)) => {
                        packet.header.rescode = ResultCode::NOERROR;
                        packet.answers.push(rec);
                    }
                    Ok(None) => {
                        packet.header.rescode = ResultCode::NXDOMAIN;
                        packet.authorities.push(zone.get_soa_record());
                    }
                    Err(code) => {
                        packet.header.rescode = code;
                    }
                }
            } else {
                packet.header.rescode = ResultCode::REFUSED;
            }
        }

        packet
    }
}
