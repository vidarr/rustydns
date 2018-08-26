extern crate rustydns;

use rustydns::Label;
use rustydns::Name;
use rustydns::ToBytes;

/******************************************************************************
 *                                            HELPERS
 ******************************************************************************/

fn print_name_bytes(bytes : &[u8]) {

    for b in bytes {
        print!("{} ", b);
    }
    print!("      ");

    let mut i = 0;
    let mut size_u8 = bytes[i];
    let mut size = size_u8 as usize;
    print!("{}", size);

    while (i + 1 + size < bytes.len()) && (size > 0) {

        let l = &bytes[i + 1 .. i + 1 + size];

        for b in l {
            print!("{}", *b as char);
        }

        print!(" ");
        i = i + 1 + size;

        size_u8 = bytes[i];
        size = size_u8 as usize;
        print!("{}", size);

    }

    println!("");

}



fn check_to_bytes<T: ToBytes>(object : T, expected: Vec<u8>) -> bool {

    let mut v = Vec::<u8>::new();

    if ! object.to_bytes(&mut v).is_ok() {
        return false;
    }

    print_name_bytes(&v[..]);

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

/*----------------------------------------------------------------------------*/

fn check_name_to_bytes(s: &str, expected: Vec<u8>) -> bool {

    let n = Name::from_str(s);
    if n.is_err() {
        return false;
    }

    check_to_bytes(n.unwrap(), expected)

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

/*----------------------------------------------------------------------------*/

#[test]
fn test_name_to_bytes() {

    assert!(check_name_to_bytes("", vec![0]));
    assert!(check_name_to_bytes("www", vec![3, b'w', b'w', b'w', 0]));
    assert!(check_name_to_bytes("Aa", vec![2, b'A', b'a', 0]));
    assert!(check_name_to_bytes("aBC.de.fghI", vec![4, b'f', b'g', b'h', b'I', 2, b'd', b'e', 3, b'a', b'B', b'C', 0]));
    assert!(check_name_to_bytes(".aBC.de.fghI", vec![4, b'f', b'g', b'h', b'I', 2, b'd', b'e', 3, b'a', b'B', b'C', 0]));

    assert!(check_name_to_bytes("www.Aa", vec![2, b'A', b'a', 3, b'w', b'w', b'w', 0]));
    assert!(check_name_to_bytes(".www.Aa", vec![2, b'A', b'a', 3, b'w', b'w', b'w', 0]));

}
