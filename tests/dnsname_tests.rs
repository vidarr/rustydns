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

use rustydns::Name;
use testhelpers::{check_to_bytes, check_from_bytes, check_partial_eq, check_to_from_string};
mod testhelpers;

/*----------------------------------------------------------------------------*/

#[test]
fn test_name_to_bytes() {

    assert!(check_to_bytes::<Name>("", vec![0]));
    assert!(check_to_bytes::<Name>("www", vec![3, b'w', b'w', b'w', 0]));
    assert!(check_to_bytes::<Name>("Aa", vec![2, b'A', b'a', 0]));
    assert!(check_to_bytes::<Name>(
        "aBC.de.fghI",
        vec![
            4,
            b'f',
            b'g',
            b'h',
            b'I',
            2,
            b'd',
            b'e',
            3,
            b'a',
            b'B',
            b'C',
            0,
        ],
    ));
    assert!(check_to_bytes::<Name>(
        ".aBC.de.fghI",
        vec![
            4,
            b'f',
            b'g',
            b'h',
            b'I',
            2,
            b'd',
            b'e',
            3,
            b'a',
            b'B',
            b'C',
            0,
        ],
    ));

    assert!(check_to_bytes::<Name>(
        "www.Aa",
        vec![2, b'A', b'a', 3, b'w', b'w', b'w', 0],
    ));
    assert!(check_to_bytes::<Name>(
        ".www.Aa",
        vec![2, b'A', b'a', 3, b'w', b'w', b'w', 0],
    ));

}

/*----------------------------------------------------------------------------*/

#[test]
fn test_name_from_bytes() {

    assert!(check_from_bytes::<Name>(&[0], Ok("")));
    assert!(check_from_bytes::<Name>(
        &[3, b'w', b'w', b'w', 0],
        Ok("www"),
    ));
    assert!(check_from_bytes::<Name>(&[2, b'A', b'a', 0], Ok("Aa")));
    assert!(check_from_bytes::<Name>(
        &[
            4,
            b'f',
            b'g',
            b'h',
            b'I',
            2,
            b'd',
            b'e',
            3,
            b'a',
            b'B',
            b'C',
            0,
        ],
        Ok("aBC.de.fghI"),
    ));
    assert!(check_from_bytes::<Name>(
        &[
            4,
            b'f',
            b'g',
            b'h',
            b'I',
            2,
            b'd',
            b'e',
            3,
            b'a',
            b'B',
            b'C',
            0,
        ],
        Ok(".aBC.de.fghI"),
    ));

    assert!(check_from_bytes::<Name>(
        &[2, b'A', b'a', 3, b'w', b'w', b'w', 0],
        Ok("www.Aa"),
    ));
    assert!(check_from_bytes::<Name>(
        &[2, b'A', b'a', 3, b'w', b'w', b'w', 0],
        Ok(".www.Aa"),
    ));

}

/*----------------------------------------------------------------------------*/

#[test]
fn check_name_partial_eq() {
    assert!(check_partial_eq::<Name>("", ""));
    assert!(! check_partial_eq::<Name>("ubeer.org", ""));
    assert!(check_partial_eq::<Name>("ubeer.org", "ubeer.org"));
    assert!(check_partial_eq::<Name>("Ubeer.org", "ubeer.org"));
    assert!(check_partial_eq::<Name>("UbeeR.orG", "ubeer.org"));
    assert!(! check_partial_eq::<Name>("beer.org", "ubeer.org"));
    assert!(! check_partial_eq::<Name>("org", "ubeer.org"));
}

/*----------------------------------------------------------------------------*/

#[test]
fn check_name_to_from_string() {

    assert!(check_to_from_string::<Name>("", Ok("OK")));
    assert!(check_to_from_string::<Name>("org", Ok("OK")));
    assert!(check_to_from_string::<Name>("org", Ok("OK")));
    assert!(check_to_from_string::<Name>("org.ubeer", Ok("OK")));
    assert!(check_to_from_string::<Name>("org.ubeer", Ok("OK")));
    assert!(check_to_from_string::<Name>("org.ubeer.www", Ok("OK")));
}

/*----------------------------------------------------------------------------*/
