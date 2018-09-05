// Copyright (c) 2018, Michael J. Beer
// All rights reserved.

// Redistribution and use in source and binary forms, with or without
// modification, are permitted provided that the following conditions are met:

// * Redistributions of source code must retain the above copyright notice, this
//   list of conditions and the following disclaimer.

// * Redistributions in binary form must reproduce the above copyright notice,
//   this list of conditions and the following disclaimer in the documentation
//   and/or other materials provided with the distribution.

// * Neither the name of the copyright holder nor the names of its
//   contributors may be used to endorse or promote products derived from
//   this software without specific prior written permission.

// THIS SOFTWARE IS PROVIDED BY THE COPYRIGHT HOLDERS AND CONTRIBUTORS "AS IS"
// AND ANY EXPRESS OR IMPLIED WARRANTIES, INCLUDING, BUT NOT LIMITED TO, THE
// IMPLIED WARRANTIES OF MERCHANTABILITY AND FITNESS FOR A PARTICULAR PURPOSE ARE
// DISCLAIMED. IN NO EVENT SHALL THE COPYRIGHT HOLDER OR CONTRIBUTORS BE LIABLE
// FOR ANY DIRECT, INDIRECT, INCIDENTAL, SPECIAL, EXEMPLARY, OR CONSEQUENTIAL
// DAMAGES (INCLUDING, BUT NOT LIMITED TO, PROCUREMENT OF SUBSTITUTE GOODS OR
// SERVICES; LOSS OF USE, DATA, OR PROFITS; OR BUSINESS INTERRUPTION) HOWEVER
// CAUSED AND ON ANY THEORY OF LIABILITY, WHETHER IN CONTRACT, STRICT LIABILITY,
// OR TORT (INCLUDING NEGLIGENCE OR OTHERWISE) ARISING IN ANY WAY OUT OF THE USE
// OF THIS SOFTWARE, EVEN IF ADVISED OF THE POSSIBILITY OF SUCH DAMAGE.
//
use dnstraits::{AsBytes, DnsEntity};
use ::std::str::FromStr;
use ::std::net::Ipv4Addr;
use dnsname::Name;

/******************************************************************************
 *                                             TYPE
 ******************************************************************************/

pub enum ResourceType {

    A = 1,
    NS = 2,
    CNAME = 5,
    PTR = 12,
    HINFO = 13,
    MX = 15,
    AXFR = 252,
    ANY = 255

}

/*----------------------------------------------------------------------------*/

/// Representation of a DNS Resource Record
pub enum Record {

    A(Ipv4Addr),
    PTR(Name)

}



impl FromStr for Record {

    type Err = &'static str;

    fn from_str(s : &str) -> Result<Record, &'static str> {

        let s = s.to_string();
        let mut splitter = s.splitn(2, " *");
        let kind = splitter.next();
        if kind.is_none() {
            return Err("Malformed record: Missing whitespace?");
        }

        let kind = kind.unwrap();

        let remainder = splitter.next();

        if remainder.is_none() {
            return Err("Malformed record: Missing whitespace?");
        }

        match kind {
            "A" => match Ipv4Addr::from_str(remainder.unwrap()) {
                Ok(addr) => Ok(Record::A(addr)),
                Err(_) => Err("Could not parse IPv4 address")
            },
            &_ => Err("Unknown DNS type")
        }

    }

}
/*----------------------------------------------------------------------------*/

impl AsBytes for Ipv4Addr {

    fn to_bytes(&self, target: &mut Vec<u8>) -> Result<(), &'static str> {

        target.extend_from_slice(&self.octets());
        Ok(())

    }

    fn from_bytes(bytes: &[u8]) -> Result<Ipv4Addr, &'static str> {

        if bytes.len() != 4 {
            Err("Require exactly 4 octets")
        } else {
            Ok(Ipv4Addr::new(bytes[0], bytes[1], bytes[2], bytes[3]))
        }

    }

}

// /*----------------------------------------------------------------------------*/
// 
// impl AsStr for ResourceRecord {
// 
//     /// Parse a string into a DNS Name
//     fn from_str(string : &str) -> Result<Self, ()> {
// 
//         let mut v = Vec::<Label>::new();
// 
//         for label_str in string.split(".") {
// 
//             match Label::from_str(label_str) {
//                 Ok(label) => {
//                     v.insert(0, label);
//                 },
//                 Err(_) => return Err(())
//             };
//         }
// 
//         let empty = Label::from_str("").unwrap();
// 
//         // Ok, that's really ugly, perhaps we can simplify this
//         // with a bit more knowledge about Rust?
//         let tail_empty = empty.eq(match v.last() {
//             None => return Err(()),
//             Some(el) => el,
//         });
// 
//         if ! tail_empty {
//             v.push(empty);
//         }
// 
//         Ok(Name { data: v,})
// 
//     }
// 
// }
// 
// /*----------------------------------------------------------------------------*/
// 
// impl AsBytes for ResourceRecord {
// 
//     fn to_bytes(&self, target: &mut Vec<u8>) -> Result<(), &'static str> {
// 
//         for l in &self.data {
//             let _l = l.to_bytes(target)?;
//         }
// 
//         Ok(())
// 
//     }
// 
//      /*-----------------------------------------------------------------------*/
// 
//     fn from_bytes(bytes: &[u8])
//         -> Result<Self, &'static str> {
// 
//         let mut offset = 0;
//         let mut v = Vec::<Label>::new();
// 
//         let mut len : usize = 1;
// 
//         while 0 < len {
// 
//             let l = Label::from_bytes(&bytes[offset ..])?;
// 
//             len = l.len() as usize;
//             v.push(l);
// 
//             offset = offset + len + 1;
//         }
// 
//         Ok(Name { data: v})
// 
//     }
// 
// }
// 
// /*----------------------------------------------------------------------------*/
// 
// impl fmt::Display for ResourceRecord {
// 
//     fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
// 
//         for l in &self.data {
//             let result = write!(f, "{}", l);
//             if result.is_err() {
//                 return result;
//             }
//         }
// 
//         Ok(())
// 
//     }
// 
// }
/*----------------------------------------------------------------------------*/
