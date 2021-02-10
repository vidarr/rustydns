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
use ::std::str;
use ::std::str::FromStr;
use ::std::fmt;
use ::std::cmp;
use ::std::iter::IntoIterator;
use dnstraits::{AsBytes, DnsEntity};
use dnslabel::Label;

/******************************************************************************
 *                                             TYPE
 ******************************************************************************/

#[derive(Debug)]
pub struct Name {
    /* Last Label MUST always be empty ! */
    data : Vec<Label>,
}

/*----------------------------------------------------------------------------*/

impl FromStr for Name {

    type Err = &'static str;

    /// Parse a string into a DNS Name
    fn from_str(string : &str) -> Result<Self, &'static str> {

        let mut v = Vec::<Label>::new();

        for label_str in string.split(".") {

            match Label::from_str(label_str) {
                Ok(label) => {
                    v.insert(0, label);
                },
                Err(_) => return Err("String is not a DNS name")
            };
        }

        let empty = Label::from_str("").unwrap();

        // Ok, that's really ugly, perhaps we can simplify this
        // with a bit more knowledge about Rust?
        let tail_empty = empty.eq(match v.last() {
            None => return Err("Name is empty"),
            Some(el) => el,
        });

        if ! tail_empty {
            v.push(empty);
        }

        Ok(Name { data: v,})

    }

}

/*----------------------------------------------------------------------------*/

impl AsBytes for Name {

    fn to_bytes(&self, target: &mut [u8]) -> Result<usize, &'static str> {

        let max_len = target.len();
        let mut write_slice = target;

        for l in &self.data {
            match l.to_bytes(write_slice) {
                Ok(remainder) => if remainder < max_len {
                    write_slice = &mut write_slice[remainder..];
                } else {
                    return Err("Target buffer too small");
                },
                Err(s) => return Err(s),
            }
        }

        Ok(max_len - write_slice.len())

    }

     /*-----------------------------------------------------------------------*/

    fn from_bytes(bytes: &[u8])
        -> Result<Self, &'static str> {

        let mut offset = 0;
        let mut v = Vec::<Label>::new();

        let mut len : usize = 1;

        while 0 < len {

            let l = Label::from_bytes(&bytes[offset ..])?;

            len = l.len() as usize;
            v.push(l);

            offset = offset + len + 1;
        }

        Ok(Name { data: v})

    }

}

/*----------------------------------------------------------------------------*/

impl DnsEntity for Name {}

/*----------------------------------------------------------------------------*/

impl fmt::Display for Name {

    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {

        // This one is going to be really expensive, isn't it???

        write!(f, "{}", Name::labels_to_string(&self.data)).ok();
        Ok(())
    }

}

/*----------------------------------------------------------------------------*/

impl cmp::PartialEq for Name {

    fn eq(&self, other: &Name) -> bool {

        self.data.iter().zip(other.data.iter()).all(
            |(a,b)| a.eq(b))

    }

}

/*----------------------------------------------------------------------------*/

impl<'a> IntoIterator for &'a Name {

    type Item = &'a Label;
    type IntoIter = ::std::slice::Iter<'a, Label>;

    fn into_iter(self) -> Self::IntoIter {
        (&self.data).into_iter()
    }

}

/*----------------------------------------------------------------------------*/

impl Name {

    pub fn to_slice(&self) -> &[Label] {

        &self.data

    }

     /*-----------------------------------------------------------------------*/

    pub fn labels_to_string(labels: &[Label]) -> String {

        let strings = labels.iter().map(|x| x.to_string()).filter(|x| ! "".eq(x));
        strings.rev().collect::<Vec<String>>().join(".")

    }

}

/*----------------------------------------------------------------------------*/
