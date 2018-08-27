extern crate rustydns;

use rustydns::Label;
use rustydns::Name;
use rustydns::AsBytes;
use rustydns::AsStr;

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

/*----------------------------------------------------------------------------*/

fn check_to_bytes<T: AsBytes>(object : T, expected: Vec<u8>) -> bool {

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

/*----------------------------------------------------------------------------*/

fn check_from_bytes<T: AsBytes + AsStr + PartialEq<T>>
(bytes: &[u8], exp: Result<&str, &str>) -> bool {

    let result = T::from_bytes(bytes);
    match exp {
        Ok(s) => {
            let entity = &result.unwrap();
            let exp = &T::from_str(s).unwrap();
            exp.eq(entity)
        },
        Err(_) => result.is_err()
    }

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
fn test_label_from_bytes() {

    assert!(check_from_bytes::<Label>(&[0], Ok("")));
    assert!(check_from_bytes::<Label>(&[3, b'w', b'w', b'w'], Ok("www")));
    assert!(check_from_bytes::<Label>(&[2, b'A', b'a'], Ok("Aa")));
    assert!(check_from_bytes::<Label>(&[64u8, 65], Err("LABEL EXCEEDS 63 char LIMIT")));

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
