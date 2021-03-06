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
use ::std::iter::Map;
use ::std::slice;

/*----------------------------------------------------------------------------*/

#[derive(Copy, Clone)]
pub struct Label {
    data : [u8; 64],
}

/*----------------------------------------------------------------------------*/

impl AsBytes for Label {

    fn to_bytes(&self, target: &mut [u8]) -> Result<usize, &'static str> {

        let len_octet = self.data[0];
        let len = len_octet as usize;

        if len > 63 {
            return Err("Label too long");
        }

        if target.len() < len + 1 {
            return Err("Target buffer too small");
        }

        target[0] = len_octet;
        target[.. len + 1].copy_from_slice(&self.data[.. len + 1]);

        Ok(len + 1)

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

impl fmt::Debug for Label {


    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // Just an alias for Display ...
        write!(f, "{}", self)
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
