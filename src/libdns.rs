use std::str;
use std::fmt;
use std::io;

/******************************************************************************
 *                                             TYPES
 ******************************************************************************/

pub struct Label {
    data : [u8; 64],
}

/*----------------------------------------------------------------------------*/

pub struct Name {
    data : Vec<Label>,
}

/*------------------------------------------------------------------------*/

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

/*------------------------------------------------------------------------*/

/// Representation of a DNS question
pub struct Question {

    name : Name,
    qtype : QuestionType,
    qclass : u16

}

/*------------------------------------------------------------------------*/

/// Representation of a DNS Resource Record
pub struct ResourceRecord {

    name : Name,

}

/*------------------------------------------------------------------------*/

/// In-memory representation of a DNS message
pub struct DnsMessage {

    id : u16,
    flags: u16,
    questions : Vec<Question>,
    answers : Vec<ResourceRecord>,
    authorities : Vec<ResourceRecord>,
    additionals : Vec<ResourceRecord>

}


/******************************************************************************
 *                                       PUBLIC FUNCTIONS
 ******************************************************************************/

impl Name {

    /// Parse a string into a DNS Name
    pub fn from_str(string : &str) -> Result<Self, ()> {

        let mut v = Vec::<Label>::new();

        for label_str  in string.split(".") {

            match Label::from_str(label_str) {
                Ok(label) => {
                    v.insert(0, label);
                },
                Err(_) => return Err(())
            };
        }

        Ok(Name { data: v,})

    }

}

/*------------------------------------------------------------------------*/

impl Label {

    /// DNS Label from a string
    pub fn from_str(string : &str) -> Result<Self, ()> {

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
