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
use ::std::collections::HashMap;
use dnslabel::Label;
use dnsname::Name;
use dnsrecord::Record;

/******************************************************************************
 *                                             TYPE
 ******************************************************************************/

pub enum ZoneEntry<'a> {

    Record(&'a Record),
    Zone(&'a Zone<'a>),
    NotFound,

}

/*----------------------------------------------------------------------------*/

pub struct Zone<'a> {

    entries : HashMap<Label, ZoneEntry<'a>>,

}

/*----------------------------------------------------------------------------*/

impl<'a> Zone<'a> {


    pub fn new() -> Zone<'a> {

        Zone {
            entries : HashMap::new()
        }

    }

    /// Tries to find a record for name within the zone.
    pub fn lookup(&self, name : &Name) -> Option<&Record> {

        let current = &mut &ZoneEntry::Zone(self);

        for label in name.into_iter() {

            match current {
                ZoneEntry::Zone(zone) => match zone.entries.get(label) {
                    None => return None,
                    Some(entry) => *current = entry,
                },
                _ => return None,
            };

        };

        match current {
            &mut ZoneEntry::Record(record) => Some(record),
            __ => None,
        }
    }

}
/*----------------------------------------------------------------------------*/

// impl Clone for Zone {
// 
//     fn clone(&self) -> Zone {
// 
//         let zone = Zone::new();
// 
//         for (k, v) in &self.entries {
// 
//             zone.entries.insert(k.clone(), v.clone());
// 
//         }
// 
//         zone
// 
//     }
// 
// }

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
