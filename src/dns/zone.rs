use crate::dns::protocol::{DnsQuestion, DnsRecord, QueryType, ResultCode, TransientTtl};
use std::net::Ipv4Addr;

pub struct Zone<'a> {
    domain: &'a str,
    mname: &'a str,
    rname: &'a str,
}

impl<'a> Zone<'a> {
    pub fn new(domain: &'a str, mname: &'a str, rname: &'a str) -> Zone<'a> {
        Zone {
            domain: domain,
            mname: mname,
            rname: rname,
        }
    }

    pub fn in_zone(&self, domain: &str) -> bool {
        domain.ends_with(&self.domain)
    }

    pub fn get_soa_record(&self) -> DnsRecord {
        DnsRecord::SOA {
            domain: self.domain.to_owned(),
            m_name: self.mname.to_owned(),
            r_name: self.rname.to_owned(),
            serial: 0,
            refresh: 3600,
            retry: 3600,
            expire: 3600,
            minimum: 3600,
            ttl: TransientTtl(3600),
        }
    }

    pub fn answer(&self, question: &DnsQuestion) -> Result<Option<DnsRecord>, ResultCode> {
        assert!(self.in_zone(&question.name));

        Ok(match question.qtype {
            QueryType::SOA => Some(self.get_soa_record()),
            QueryType::A => {
                let parts: Vec<&str> = question.name.splitn(2, '.').collect();
                if parts.len() == 2 && parts[1] == self.domain {
                    match parts[0].replace('-', ".").parse::<Ipv4Addr>() {
                        Err(_) => None,
                        Ok(ip) => Some(DnsRecord::A {
                            domain: question.name.clone(),
                            addr: ip,
                            ttl: TransientTtl(3600),
                        }),
                    }
                } else {
                    None
                }
            }
            _ => None,
        })
    }
}
