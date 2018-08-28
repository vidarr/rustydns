extern crate rustydns;

use rustydns::Name;
use testhelpers::{check_to_bytes, check_from_bytes, check_partial_eq};
mod testhelpers;

/*----------------------------------------------------------------------------*/

#[test]
fn test_name_to_bytes() {

    assert!(check_to_bytes::<Name>("", vec![0]));
    assert!(check_to_bytes::<Name>("www", vec![3, b'w', b'w', b'w', 0]));
    assert!(check_to_bytes::<Name>("Aa", vec![2, b'A', b'a', 0]));
    assert!(check_to_bytes::<Name>("aBC.de.fghI", vec![4, b'f', b'g', b'h', b'I', 2, b'd', b'e', 3, b'a', b'B', b'C', 0]));
    assert!(check_to_bytes::<Name>(".aBC.de.fghI", vec![4, b'f', b'g', b'h', b'I', 2, b'd', b'e', 3, b'a', b'B', b'C', 0]));

    assert!(check_to_bytes::<Name>("www.Aa", vec![2, b'A', b'a', 3, b'w', b'w', b'w', 0]));
    assert!(check_to_bytes::<Name>(".www.Aa", vec![2, b'A', b'a', 3, b'w', b'w', b'w', 0]));

}

/*----------------------------------------------------------------------------*/

#[test]
fn test_name_from_bytes() {

    assert!(check_from_bytes::<Name>(&[0], Ok("")));
    assert!(check_from_bytes::<Name>(&[3, b'w', b'w', b'w', 0], Ok("www")));
    assert!(check_from_bytes::<Name>(&[2, b'A', b'a', 0], Ok("Aa")));
    assert!(check_from_bytes::<Name>(&[4, b'f', b'g', b'h', b'I', 2, b'd', b'e', 3, b'a', b'B', b'C', 0], Ok("aBC.de.fghI")));
    assert!(check_from_bytes::<Name>(&[4, b'f', b'g', b'h', b'I', 2, b'd', b'e', 3, b'a', b'B', b'C', 0], Ok(".aBC.de.fghI")));

    assert!(check_from_bytes::<Name>(&[2, b'A', b'a', 3, b'w', b'w', b'w', 0], Ok("www.Aa")));
    assert!(check_from_bytes::<Name>(&[2, b'A', b'a', 3, b'w', b'w', b'w', 0], Ok(".www.Aa")));

}

/*----------------------------------------------------------------------------*/

#[test]
fn check_label_partial_eq() {

    assert!(check_partial_eq::<Name>("", ""));

}

