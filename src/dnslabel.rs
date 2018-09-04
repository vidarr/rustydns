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
use ::std::fmt;
use ::std::cmp;
use dnstraits::{AsBytes, AsStr, DnsEntity};
use ::std::hash::{Hash, Hasher};

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

        let mut v = [0u8; 64];

        v[.. len + 1].copy_from_slice(&bytes[.. len + 1]);

        Ok(Label {
            data: v,
        })

    }

}

/*----------------------------------------------------------------------------*/

impl AsStr for Label {

    /// DNS Label from a string
    fn from_str(string : &str) -> Result<Self, ()> {

        let len = string.len();

        if 63 < len {
            return Err(())
        }

        let mut n :[u8; 64] = [0; 64];
        n[0] = len as u8;
        n[1 .. len + 1].clone_from_slice(string.as_bytes());
        Ok( Label { data: n,})
    }

}

/*----------------------------------------------------------------------------*/

impl DnsEntity for Label {}

/*----------------------------------------------------------------------------*/

impl fmt::Display for Label {

    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {

        for u in self.data.iter() {
            let result = match u {
                &0 => write!(f, "0"),
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

        // TODO: Something must be done here because currently:
        // a == b -> hash(a) == hash(b) DOES NOT HOLD
        self.data.iter().zip(other.data.iter()).all(
            |(a,b)| {
                let a_ascii = *a as char;
                let b_ascii = *b as char;
                a_ascii.to_ascii_uppercase().eq(
                    &b_ascii.to_ascii_uppercase())
            })

    }

}

/*----------------------------------------------------------------------------*/

impl cmp::Eq for Label {}
/*----------------------------------------------------------------------------*/

impl Label {

    pub fn len(&self) -> usize {
        self.data[0] as usize
    }

}

/*----------------------------------------------------------------------------*/

impl Hash for Label {

    fn hash<H: Hasher>(&self, state : &mut H) {

        self.data.hash(state);

    }

}

/*----------------------------------------------------------------------------*/