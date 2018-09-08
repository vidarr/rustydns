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
extern crate rustydns;
mod testhelpers;

use ::std::str::FromStr;

use rustydns::Label;
use testhelpers::{check_to_bytes, check_from_bytes, check_partial_eq, check_to_from_string};

/******************************************************************************
 *                                             TESTS
 ******************************************************************************/

#[test]
fn test_label_to_bytes() {

    assert!(check_to_bytes::<Label>("", vec![0]));
    assert!(check_to_bytes::<Label>("www", vec![3, b'w', b'w', b'w']));
    assert!(check_to_bytes::<Label>("Aa", vec![2, b'A', b'a']));

}

/*----------------------------------------------------------------------------*/

#[test]
fn test_label_from_bytes() {

    assert!(check_from_bytes::<Label>(&[0], Ok("")));
    assert!(check_from_bytes::<Label>(&[3, b'w', b'w', b'w'], Ok("www")));
    assert!(check_from_bytes::<Label>(&[2, b'A', b'a'], Ok("Aa")));
    assert!(check_from_bytes::<Label>(
            &[64u8, 65],
            Err("LABEL EXCEEDS 63 char LIMIT"),
            ));

}


/*----------------------------------------------------------------------------*/

#[test]
fn check_label_to_from_string() {

    assert!(check_to_from_string::<Label>("", Ok("OK")));
    assert!(check_to_from_string::<Label>("org", Ok("OK")));
    assert!(check_to_from_string::<Label>("TEST", Ok("OK")));
    assert!(check_to_from_string::<Label>("ubeer", Ok("OK")));
    assert!(check_to_from_string::<Label>("Ubeer", Ok("OK")));
    assert!(check_to_from_string::<Label>("aFGgG", Ok("OK")));
}

/*----------------------------------------------------------------------------*/

#[test]
fn check_label_partial_eq() {

    assert!(check_partial_eq::<Label>("", ""));
    assert!(!check_partial_eq::<Label>("1", ""));
    assert!(!check_partial_eq::<Label>("", "1"));
    assert!(check_partial_eq::<Label>("1", "1"));
    assert!(check_partial_eq::<Label>("a", "a"));
    assert!(!check_partial_eq::<Label>("a", "aa"));
    assert!(check_partial_eq::<Label>("aa", "aa"));
    assert!(check_partial_eq::<Label>("A", "a"));
    assert!(check_partial_eq::<Label>("dhgfe", "dhgfe"));
    assert!(check_partial_eq::<Label>("DHGFE", "dhgfe"));
    assert!(check_partial_eq::<Label>("dHGFE", "dhgfe"));
    assert!(check_partial_eq::<Label>("dhgfE", "dhgfe"));
    assert!(check_partial_eq::<Label>("dHGFe", "Dhgfe"));
    assert!(!check_partial_eq::<Label>("dHGVe", "DhgFe"));
    assert!(!check_partial_eq::<Label>(
            "dHGVedHGVedHGVedHGVedHGVedHGVedHGVedHGVedHGVedHGVedHGVedHGVedHGt",
            "dHGVedHGVedHGVedHGVedHGVedHGVedHGVedHGVedHGVedHGVedHGVedHGVedHGt",
            ));

}

/*----------------------------------------------------------------------------*/

use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};

fn hash_label(s: &str) ->u64 {

    let mut hasher = DefaultHasher::new();
    Label::from_str(s).ok().unwrap().hash(&mut hasher);
    hasher.finish()
}

#[test]
fn check_label_hash() {

    assert!(hash_label("") == hash_label(""));
    assert!(hash_label("1") == hash_label("1"));
    assert!(hash_label("a") == hash_label("a"));
    assert!(hash_label("aa") == hash_label("aa"));
    assert!(hash_label("A") == hash_label("a"));
    assert!(hash_label("dhgfe") == hash_label("dhgfe"));
    assert!(hash_label("DHGFE") == hash_label("dhgfe"));
    assert!(hash_label("dHGFE") == hash_label("dhgfe"));
    assert!(hash_label("dhgfE") == hash_label("dhgfe"));
    assert!(hash_label("dHGFe") == hash_label("Dhgfe"));

}

/*----------------------------------------------------------------------------*/
