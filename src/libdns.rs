use std::str;
use std::slice;
use std::fmt;
use std::cmp;

/******************************************************************************
 *                                             TYPES
 ******************************************************************************/

pub struct Label {
    data : [u8; 64],
}

/*----------------------------------------------------------------------------*/

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

/******************************************************************************
 *                                            TRAITS
 ******************************************************************************/

pub trait AsBytes<T=Self> {

    fn to_bytes(&self, target: &mut Vec<u8>) -> Result<(), &'static str>;

    fn from_bytes(bytes: &[u8]) -> Result<T, &'static str>;

}



pub trait AsStr<T=Self> {

    fn from_str(string : &str) -> Result<T, ()>;

}


/******************************************************************************
                                      AsStr implementation
 ******************************************************************************/

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

/******************************************************************************
 *                                    AsBytes Implementation
 ******************************************************************************/

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

impl AsBytes for Name {

    fn to_bytes(&self, target: &mut Vec<u8>) -> Result<(), &'static str> {

        for l in &self.data {
            let l = l.to_bytes(target)?;
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

            len = l.data[0] as usize;
            v.push(l);

            offset = offset + len + 1;
        }

        Ok(Name { data: v})

    }

}

/******************************************************************************
 *                                            Display
 ******************************************************************************/

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

/******************************************************************************
 *                                   PartialEq IMPLEMENTATION
 ******************************************************************************/

impl cmp::PartialEq for Label {

    fn eq(&self, other: &Label) -> bool {

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

impl cmp::PartialEq for Name {

    fn eq(&self, other: &Name) -> bool {

        self.data.iter().zip(other.data.iter()).all(
            |(a,b)| a.eq(b))

    }

}
/******************************************************************************
 *                                      Name Implementation
 ******************************************************************************/

impl Name {

    pub fn iter(&self) -> slice::Iter<Label> {

        self.data.iter()

    }

}

/*----------------------------------------------------------------------------*/
