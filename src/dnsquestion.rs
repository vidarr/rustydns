// Copyright (c) 2021, Michael J. Beer
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

/*----------------------------------------------------------------------------*/
use ::std::str;
use ::std::str::FromStr;
use dnstraits::AsBytes;
use ::std::fmt;
use dnsname::Name;

/*----------------------------------------------------------------------------*/

#[derive(Clone, Copy)]
pub enum QuestionType {

    A = 1,
    Ns = 2,
    Cname = 5,
    Ptr = 12,
    Hinfo = 13,
    Mx = 15,
    Axfr = 252,
    Any = 255

}

/*----------------------------------------------------------------------------*/

impl FromStr for QuestionType {

    type Err = &'static str;

    fn from_str(string: &str) -> Result<Self, &'static str> {

        let qt = match string {

            "A" => QuestionType::A,
            "NS" => QuestionType::Ns,
            "CNAME" => QuestionType::Cname,
            "PTR" => QuestionType::Ptr,
            "HINFO" => QuestionType::Hinfo,
            "MX" => QuestionType::Mx,
            "AXFR" => QuestionType::Axfr,
            "ANY" => QuestionType::Any,
            _ => return Err("Unknown Question Type")
        };

        Ok(qt)

    }

}

/*----------------------------------------------------------------------------*/

impl AsBytes for QuestionType {

    fn to_bytes(&self, target: &mut [u8]) -> Result<usize, &'static str> {

        let max_len = target.len();

        if max_len < 2 {

            return Err("Too few bytes left to write QuestionType");

        }

        let qt_u16 = *self as u16;
        let qt_u16_be = u16::to_be_bytes(qt_u16);

        let len = qt_u16_be.len();

        target[.. len].copy_from_slice(&qt_u16_be);

        Ok(qt_u16_be.len())

    }

     /*-----------------------------------------------------------------------*/

    fn from_bytes(bytes: &[u8])
        -> Result<Self, &'static str> {

        let bytes_len = bytes.len();

        let len = 2;

        if bytes_len < len {
            return Err("too few bytes");
        }

        let qtype_u16= u16::from_be_bytes([bytes[0], bytes[1]]);

        let qtype = match qtype_u16 {

            1 => QuestionType::A,
            2 => QuestionType::Ns,
            5 => QuestionType::Cname,
            12 => QuestionType::Ptr,
            13 => QuestionType::Hinfo,
            15 => QuestionType::Mx,
            252 => QuestionType::Axfr,
            255 => QuestionType::Any,
            _ => return Err("Invalid question type")
        };

        Ok(qtype)

    }

}

/*----------------------------------------------------------------------------*/

impl fmt::Display for QuestionType {

    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {

        // This one is going to be really expensive, isn't it???

        write!(f, "{}", QuestionType::to_string(&self)).ok();
        Ok(())
    }

}

/*----------------------------------------------------------------------------*/

impl PartialEq for QuestionType {

    fn eq(&self, other : &Self) -> bool {

        let me = *self as u16;
        let you = *other as u16;

        print!("Comparing {} with {}: {}", &me, &you, me == you);

        me == you

    }

}

/*----------------------------------------------------------------------------*/

impl QuestionType {

    fn to_string(qt: &QuestionType) -> String {

        match qt {

            QuestionType::A => "A".to_string(),
            QuestionType::Ns => "NS".to_string(),
            QuestionType::Cname => "CNAME".to_string(),
            QuestionType::Ptr => "PTR".to_string(),
            QuestionType::Hinfo => "HINFO".to_string(),
            QuestionType::Mx => "MX".to_string(),
            QuestionType::Axfr => "AXFR".to_string(),
            QuestionType::Any => "ANY".to_string()

        }

    }

}

/*----------------------------------------------------------------------------*/

pub struct Question {

    name : Vec<Name>,
    qtype : QuestionType,
    qclass : u16

}

/*----------------------------------------------------------------------------*/
