use std::fmt;
use std::cmp;
use dnstraits::{AsBytes, AsStr, DnsEntity};

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

impl Label {

    pub fn len(&self) -> usize {
        self.data[0] as usize
    }

}
