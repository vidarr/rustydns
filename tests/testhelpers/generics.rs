// Copyright (c) 2018, Michael J. Beer
// All rights reserved.
//
// Redistribution and use in source and binary forms, with or without
// modification, are permitted provided that the following conditions are met:
//
// * Redistributions of source code must retain the above copyright notice, this
//   list of conditions and the following disclaimer.
//
// * Redistributions in binary form must reproduce the above copyright notice,
//   this list of conditions and the following disclaimer in the documentation
//   and/or other materials provided with the distribution.
//
// * Neither the name of the copyright holder nor the names of its
//   contributors may be used to endorse or promote products derived from
//   this software without specific prior written permission.
//
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
use ::std::cmp::PartialEq;
use rustydns::{AsBytes, DnsEntity};
use testhelpers::common::{print_name_bytes, print_str_as_bytes};
/*----------------------------------------------------------------------------*/

pub fn check_from_bytes<T: AsBytes + FromStr + PartialEq<T>>
(bytes: &[u8], exp: Result<&str, &str>) -> bool {

    let result = T::from_bytes(bytes);
    match exp {
        Ok(s) => {
            let entity = &result.unwrap();
            let exp = &T::from_str(s).ok().unwrap();
            exp.eq(entity)
        },
        Err(_) => result.is_err()
    }

}

/*----------------------------------------------------------------------------*/

pub fn check_partial_eq<T: PartialEq + DnsEntity>(str1: &str, str2: &str) -> bool {

    let result1 = T::from_str(str1);
    let result2 = T::from_str(str2);

    if result1.is_err() || result2.is_err() {
        return false;
    }

    let entity1 = result1.ok().unwrap();
    let entity2 = result2.ok().unwrap();

    entity1.eq(&entity2)

}

/*----------------------------------------------------------------------------*/

pub fn check_to_bytes<T: AsBytes + FromStr>(s: &str, expected: Vec<u8>) -> bool {

    let object = T::from_str(s);
    if object.is_err() {
        return false;
    }

    let object = object.ok().unwrap();

    let mut v = Vec::<u8>::new();

    if ! object.to_bytes(&mut v).is_ok() {
        return false;
    }

    print_name_bytes(&v[..]);

    expected.eq(&v)

}

/*----------------------------------------------------------------------------*/

pub fn check_to_from_string<T>
(s: &str, expected: Result<&'static str, &'static str>) -> bool
where T: FromStr + ToString {

    print!("{} ", s);
    print_str_as_bytes(s);

    let object = T::from_str(s);
    match expected {
        Err(_) => {
            println!("Could not parse");
            object.is_err()
        },
        Ok(_) => {
            let serialized = object.ok().unwrap().to_string();
            print_str_as_bytes(&serialized);
            s.to_string().eq(&serialized)
        }
    }
}

/*----------------------------------------------------------------------------*/
