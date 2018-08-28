extern crate rustydns;

use rustydns::Label;
use testhelpers::{check_to_bytes, check_from_bytes, check_partial_eq};
mod testhelpers;

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
    assert!(check_from_bytes::<Label>(&[64u8, 65], Err("LABEL EXCEEDS 63 char LIMIT")));

}


/*----------------------------------------------------------------------------*/

#[test]
fn check_label_partial_eq() {

    assert!(check_partial_eq::<Label>("", ""));
    assert!(! check_partial_eq::<Label>("1", ""));
    assert!(! check_partial_eq::<Label>("", "1"));
    assert!( check_partial_eq::<Label>("1", "1"));
    assert!( check_partial_eq::<Label>("a", "a"));
    assert!(! check_partial_eq::<Label>("a", "aa"));
    assert!( check_partial_eq::<Label>("aa", "aa"));
    assert!( check_partial_eq::<Label>("A", "a"));
    assert!( check_partial_eq::<Label>("dhgfe", "dhgfe"));
    assert!( check_partial_eq::<Label>("DHGFE", "dhgfe"));
    assert!( check_partial_eq::<Label>("dHGFE", "dhgfe"));
    assert!( check_partial_eq::<Label>("dhgfE", "dhgfe"));
    assert!( check_partial_eq::<Label>("dHGFe", "Dhgfe"));
    assert!(! check_partial_eq::<Label>("dHGVe", "DhgFe"));
    assert!(! check_partial_eq::<Label>(
            "dHGVedHGVedHGVedHGVedHGVedHGVedHGVedHGVedHGVedHGVedHGVedHGVedHGt",
            "dHGVedHGVedHGVedHGVedHGVedHGVedHGVedHGVedHGVedHGVedHGVedHGVedHGt"));

}

// TODO: Tests for PartialEq for Label, Name
