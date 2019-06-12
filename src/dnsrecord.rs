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
use dnstraits::{AsBytes};
use ::std::str::FromStr;
use ::std::net::Ipv4Addr;
use dnsname::Name;
use ::std::fmt;
use ::std::cmp::PartialEq;

/******************************************************************************
 *                                             TYPE
 ******************************************************************************/

/// Representation of a DNS Resource Record - Data portion
#[derive(PartialEq)]
#[derive(Debug)]
pub enum Record {

    A(Ipv4Addr),
    PTR(Name)

}

/*----------------------------------------------------------------------------*/

// pub enum RecordType {
//     TypeA = 1,
//     TypePtr = 12,
// 
// }

/*----------------------------------------------------------------------------*/

impl FromStr for Record {

    type Err = &'static str;

    fn from_str(s : &str) -> Result<Record, &'static str> {

        let s = s.to_string();
        let mut splitter = s.splitn(2, " ");
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
            "PTR" => match Name::from_str(remainder.unwrap()) {
                Ok(name) => Ok(Record::PTR(name)),
                Err(_) => Err("Could not parse DNS name")
            }
            &_ => Err("Unknown DNS type")
        }

    }

}

/*----------------------------------------------------------------------------*/

// impl ToString for Record {
// 
//     fn to_string(&self) -> String {
// 
//         match self {
//             Record::A(ref addr) => "A ".to_string() + &addr.to_string(),
//             Record::PTR(ref name) => "PTR ".to_string() + &name.to_string(),
//             _ => "Unknown Record".to_string()
//         }
// 
//     }
// }

/*----------------------------------------------------------------------------*/

impl AsBytes for Ipv4Addr {

    fn to_bytes(&self, target: &mut [u8]) -> Result<usize, &'static str> {

        if target.len() < 4 {
            return Err("Target buffe too small");
        }

        target[.. 4].copy_from_slice(&self.octets());
        Ok(4)

    }

    fn from_bytes(bytes: &[u8]) -> Result<Ipv4Addr, &'static str> {

        if bytes.len() != 4 {
            Err("Require exactly 4 octets")
        } else {
            Ok(Ipv4Addr::new(bytes[0], bytes[1], bytes[2], bytes[3]))
        }

    }

}

/*----------------------------------------------------------------------------*/

impl fmt::Display for Record {

    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {

        let (t, d) =
            match self {
                Record::A(ref addr) => ("A", addr.to_string()),
                Record::PTR(ref name) => ("PTR", name.to_string()),
            };
        write!(f, "{} {}", t, d);

        Ok(())

    }

}

/*----------------------------------------------------------------------------*/

// impl AsBytes for Record {
// 
//     fn to_bytes(&self, mut target: &[u8]) -> Result<(), &'static str> {
// 
//         match self {
//             A(address) => {
//                 target.push_back(TypeA);
//                 target.push_back(1);   // 'class' field
//                 // resource data length (1 octet)
//                 address.to_bytes(target)
//             }
//             PTR(name) => {
//                 target.push_back(TypePtr);
//                 name.to_bytes(target)
//             },
//         }
// 
//         target.extend_from_slice(&self.octets());
//         Ok(())
// 
//     }
// 
//     fn from_bytes(bytes: &[u8]) -> Result<Ipv4Addr, &'static str> {
// 
//     }
// 
// }

// impl ::std::cmp::PartialEq for Record {
// 
//     fn eq(&self, other: &Record) -> bool {
// 
//         self.iter().zip(other.data.iter()).all(
//             |(a,b)| a.eq(b))
// 
//     }
// 
// }

/*----------------------------------------------------------------------------*/
