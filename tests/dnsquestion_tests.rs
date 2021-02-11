/*
 * (C) 2021 Michael J. Beer
 * All rights reserved.
 *
 * Redistribution  and use in source and binary forms, with or with‐
 * out modification, are permitted provided that the following  con‐
 * ditions are met:
 *
 * 1. Redistributions of source code must retain the above copyright
 * notice, this list of conditions and the following disclaimer.
 *
 * 2. Redistributions in binary form must reproduce the above  copy‐
 * right  notice,  this  list  of  conditions and the following dis‐
 * claimer in the documentation and/or other materials provided with
 * the distribution.
 *
 * 3.  Neither the name of the copyright holder nor the names of its
 * contributors may be used to endorse or promote  products  derived
 * from this software without specific prior written permission.
 *
 * THIS  SOFTWARE IS PROVIDED BY THE COPYRIGHT HOLDERS AND CONTRIBU‐
 * TORS "AS IS" AND ANY EXPRESS OR  IMPLIED  WARRANTIES,  INCLUDING,
 * BUT NOT LIMITED TO, THE IMPLIED WARRANTIES OF MERCHANTABILITY AND
 * FITNESS FOR A PARTICULAR PURPOSE  ARE  DISCLAIMED.  IN  NO  EVENT
 * SHALL  THE COPYRIGHT HOLDER OR CONTRIBUTORS BE LIABLE FOR ANY DI‐
 * RECT, INDIRECT, INCIDENTAL, SPECIAL, EXEMPLARY, OR  CONSEQUENTIAL
 * DAMAGES (INCLUDING, BUT NOT LIMITED TO, PROCUREMENT OF SUBSTITUTE
 * GOODS OR SERVICES; LOSS OF USE, DATA, OR PROFITS; OR BUSINESS IN‐
 * TERRUPTION)  HOWEVER  CAUSED  AND  ON  ANY  THEORY  OF LIABILITY,
 * WHETHER IN CONTRACT, STRICT LIABILITY, OR TORT (INCLUDING  NEGLI‐
 * GENCE  OR  OTHERWISE)  ARISING  IN ANY WAY OUT OF THE USE OF THIS
 * SOFTWARE, EVEN IF ADVISED OF THE POSSIBILITY OF SUCH DAMAGE.
 */

/*----------------------------------------------------------------------------*/
extern crate rustydns;
mod testhelpers;

use testhelpers::{check_to_bytes,  check_from_bytes, check_to_from_string};
use rustydns::QuestionType;

/*----------------------------------------------------------------------------*/

#[test]
fn test_question_type_to_bytes() {

    assert!(check_to_bytes::<QuestionType>("A", vec![0u8, 1u8]));
    assert!(check_to_bytes::<QuestionType>("NS", vec![0u8, 2u8]));
    assert!(check_to_bytes::<QuestionType>("CNAME", vec![0u8, 5u8]));
    assert!(check_to_bytes::<QuestionType>("PTR", vec![0u8, 12u8]));
    assert!(check_to_bytes::<QuestionType>("HINFO", vec![0u8, 13u8]));
    assert!(check_to_bytes::<QuestionType>("MX", vec![0u8, 15u8]));
    assert!(check_to_bytes::<QuestionType>("AXFR", vec![0u8, 252u8]));
    assert!(check_to_bytes::<QuestionType>("ANY", vec![0u8, 255u8]));

}

/*----------------------------------------------------------------------------*/

#[test]
fn test_question_type_from_bytes() {

    assert!(check_from_bytes::<QuestionType>( &[0u8, 1u8], Ok("A")));
    assert!(check_from_bytes::<QuestionType>( &[0u8, 2u8], Ok("NS")));
    assert!(check_from_bytes::<QuestionType>( &[0u8, 5u8], Ok("CNAME")));
    assert!(check_from_bytes::<QuestionType>( &[0u8, 12u8], Ok("PTR")));
    assert!(check_from_bytes::<QuestionType>( &[0u8, 13u8], Ok("HINFO")));
    assert!(check_from_bytes::<QuestionType>( &[0u8, 15u8], Ok("MX")));
    assert!(check_from_bytes::<QuestionType>( &[0u8, 252u8], Ok("AXFR")));
    assert!(check_from_bytes::<QuestionType>( &[0u8, 255u8], Ok("ANY")));

}
/*----------------------------------------------------------------------------*/

#[test]
fn test_question_type_to_from_string() {

    assert!(check_to_from_string::<QuestionType>("A", Ok("A")));
    assert!(check_to_from_string::<QuestionType>("NS", Ok("NS")));
    assert!(check_to_from_string::<QuestionType>("CNAME", Ok("CNAME")));
    assert!(check_to_from_string::<QuestionType>("PTR", Ok("PTR")));
    assert!(check_to_from_string::<QuestionType>("HINFO", Ok("HINFO")));
    assert!(check_to_from_string::<QuestionType>("MX", Ok("MX")));
    assert!(check_to_from_string::<QuestionType>("AXFR", Ok("AXFR")));
    assert!(check_to_from_string::<QuestionType>("ANY", Ok("ANY")));
    assert!(check_to_from_string::<QuestionType>("InvaLid", Err("")));

}

/*----------------------------------------------------------------------------*/
