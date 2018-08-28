use std::str;
use std::slice;
use std::fmt;
use std::cmp;
use dnstraits::{AsBytes, AsStr, DnsEntity};
use dnslabel::Label;

/******************************************************************************
 *                                             TYPE
 ******************************************************************************/

pub struct Name {
    /* Last Label MUST always be empty ! */
    data : Vec<Label>,
}

/*----------------------------------------------------------------------------*/

pub enum QuestionType {

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

/// Representation of a DNS question
pub struct Question {

    name : Name,
    qtype : QuestionType,
    qclass : u16

}

/*----------------------------------------------------------------------------*/

/// Representation of a DNS Resource Record
pub struct ResourceRecord {

    name : Name,

}

/*----------------------------------------------------------------------------*/

/// In-memory representation of a DNS message
pub struct DnsMessage {

    id : u16,
    flags: u16,
    questions : Vec<Question>,
    answers : Vec<ResourceRecord>,
    authorities : Vec<ResourceRecord>,
    additionals : Vec<ResourceRecord>

}

/*----------------------------------------------------------------------------*/

impl AsStr for Name {

    /// Parse a string into a DNS Name
    fn from_str(string : &str) -> Result<Self, ()> {

        let mut v = Vec::<Label>::new();

        for label_str in string.split(".") {

            match Label::from_str(label_str) {
                Ok(label) => {
                    v.insert(0, label);
                },
                Err(_) => return Err(())
            };
        }

        let empty = Label::from_str("").unwrap();

        // Ok, that's really ugly, perhaps we can simplify this
        // with a bit more knowledge about Rust?
        let tail_empty = empty.eq(match v.last() {
            None => return Err(()),
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

    fn to_bytes(&self, target: &mut Vec<u8>) -> Result<(), &'static str> {

        for l in &self.data {
            let _l = l.to_bytes(target)?;
        }

        Ok(())

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

        for l in &self.data {
            let result = write!(f, "{}", l);
            if result.is_err() {
                return result;
            }
        }

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

impl Name {

    pub fn iter(&self) -> slice::Iter<Label> {

        self.data.iter()

    }

}

/*----------------------------------------------------------------------------*/
