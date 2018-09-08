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
use ::std::str::FromStr;
use ::std::fmt;
use ::std::cmp;
use dnstraits::{AsBytes, DnsEntity};
use ::std::hash::{Hash, Hasher};
use ::std::iter::{Map, IntoIterator, FromIterator};
use ::std::slice;

/*----------------------------------------------------------------------------*/

#[derive(Copy, Clone)]
pub struct Label {
    data : [u8; 64],
}

/*----------------------------------------------------------------------------*/

impl AsBytes for Label {

    fn to_bytes(&self, target: &mut Vec<u8>) -> Result<(), &'static str> {

        let len = self.data[0];

        if len > 63 {
            return Err("Label too long")
        }

        target.extend(self.data[..(len + 1) as usize].iter().cloned());

        Ok(())

    }

    /*------------------------------------------------------------------------*/

    fn from_bytes(bytes: &[u8]) -> Result<Self, &'static str> {

        let bytes_len = bytes.len();

        let len = bytes[0] as usize;

        if bytes_len < len + 1 {
            return Err("too few bytes");
        }

        if len > 63 {
            return Err("Label too long");
        }

        let mut data = [0u8; 64];

        data[.. len + 1].copy_from_slice(&bytes[.. len + 1]);

        Ok(Label { data })

    }

}

/*----------------------------------------------------------------------------*/

impl FromStr for Label {

    type Err = &'static str;

    /// DNS Label from a string
    fn from_str(string : &str) -> Result<Self, &'static str> {

        let len = string.len();

        if 63 < len {
            return Err("Label longer than 63 chars")
        }

        let mut data :[u8; 64] = [0; 64];
        data[0] = len as u8;
        data[1 .. len + 1].clone_from_slice(string.as_bytes());
        Ok( Label { data})


    }
}

/*----------------------------------------------------------------------------*/

impl DnsEntity for Label {}

/*----------------------------------------------------------------------------*/

impl fmt::Display for Label {

    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {

        for u in self.data[1..].iter() {
            let result = match u {
                &0 => break,
                &other => write!(f, "{}", other as char),
            };

            if result.is_err() {
                return result;
            }

        }

        Ok(())
    }

}

/*----------------------------------------------------------------------------*/

impl cmp::PartialEq for Label {

    fn eq(&self, other: &Label) -> bool {

        if self.len() != other.len() {
            return false;
        }

        self.into_normalized_iter().zip(
            other.into_normalized_iter()).all(|(a,b)| a == b)

    }

}

/*----------------------------------------------------------------------------*/

impl cmp::Eq for Label {}
/*----------------------------------------------------------------------------*/

impl Label {

    pub fn len(&self) -> usize {
        self.data[0] as usize
    }

    fn into_normalized_iter<'a>(&'a self) -> Map<slice::Iter<'a, u8>, fn(&u8) -> u8> {

        let len = self.len();
        self.data[1 .. 1 + len].into_iter().map(_to_ascii_uppercase)

    }
}

/*----------------------------------------------------------------------------*/

impl Hash for Label {

    fn hash<H: Hasher>(&self, state : &mut H) {

        for octet in self.into_normalized_iter() {
            octet.hash(state);
        }

    }
}

/******************************************************************************
 *                                        HELPER METHODS
 ******************************************************************************/


fn _to_ascii_uppercase(byte : &u8) -> u8 {

    (*byte as char).to_ascii_uppercase() as u8

}

/*----------------------------------------------------------------------------*/

fn _deref_u8(octet_ref: &u8) -> u8 {
    *octet_ref
}

/*----------------------------------------------------------------------------*/

/// Turn a label into a sequence of bytes
/// TODO: Test
impl<'a> IntoIterator for &'a  Label {

    type Item = u8;

    type IntoIter = Map<slice::Iter<'a, u8>, fn(&u8) -> u8>;

    fn into_iter(self) -> Map<slice::Iter<'a, u8>, fn(&u8) -> u8> {

        let len = self.len();
        self.data[1 .. 1 + len].into_iter().map(_deref_u8)

    }

}


// impl<'a> IntoIterator for &'a  Label {
// 
//     type Item = u8;
// 
//     type IntoIter = slice::Iter<'a, u8>;
// 
//     fn into_iter(self) -> slice::Iter<'a, u8> {
// 
//         let len = self.len();
//         self.data[1 .. 1 + len].into_iter()
// 
//     }
// 
// }

/*----------------------------------------------------------------------------*/

/// Create a Label from an octet stream
// TODO: Test
impl FromIterator<u8> for Label {

    fn from_iter<I: IntoIterator<Item=u8>>(iter : I) -> Self {

        let mut data = [0u8;64];

        let mut iter = iter.into_iter();

        match iter.next() {
            Some(octet) => data[0] = octet,
            _ => return Label { data }
        };

        for i in 1 .. data[0] {

            match iter.next() {
                Some(octet) => data[i as usize] = octet,
                     _ => return Label { data : [0u8; 64] }
            }
        }

        Label {data}

    }

}

/*----------------------------------------------------------------------------*/
