extern crate rustydns;

use rustydns::Label;
use rustydns::ToBytes;

/******************************************************************************
 *                                            HELPERS
 ******************************************************************************/

fn check_to_bytes<T: ToBytes>(object : T, expected: Vec<u8>) -> bool {

    let mut v = Vec::<u8>::new();

    if ! object.to_bytes(&mut v).is_ok() {
        return false;
    }

    expected.eq(&v)

}

/*----------------------------------------------------------------------------*/

fn check_label_to_bytes(s: &str, expected: Vec<u8>) -> bool {

    let l = Label::from_str(s);
    if l.is_err() {
        return false;
    }
    check_to_bytes(l.unwrap(), expected)

}

/******************************************************************************
 *                                             TESTS
 ******************************************************************************/

#[test]
fn test_label_to_bytes() {

    assert!(check_label_to_bytes("", vec![0]));
    assert!(check_label_to_bytes("www", vec![3, b'w', b'w', b'w']));
    assert!(check_label_to_bytes("Aa", vec![2, b'A', b'a']));

}
