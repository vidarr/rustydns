use rustydns::{AsBytes, AsStr, DnsEntity};
use std::cmp::PartialEq;
/*----------------------------------------------------------------------------*/

pub fn print_name_bytes(bytes : &[u8]) {

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

pub fn check_to_bytes<T: AsBytes + AsStr>(s: &str, expected: Vec<u8>) -> bool {

    let object = T::from_str(s);
    if object.is_err() {
        return false;
    }

    let object = object.unwrap();

    let mut v = Vec::<u8>::new();

    if ! object.to_bytes(&mut v).is_ok() {
        return false;
    }

    print_name_bytes(&v[..]);

    expected.eq(&v)

}

/*----------------------------------------------------------------------------*/

pub fn check_from_bytes<T: AsBytes + AsStr + PartialEq<T>>
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

/*----------------------------------------------------------------------------*/

pub fn check_partial_eq<T: PartialEq + DnsEntity>(str1: &str, str2: &str) -> bool {

    let result1 = T::from_str(str1);
    let result2 = T::from_str(str2);

    if result1.is_err() || result2.is_err() {
        return false;
    }

    let entity1 = result1.unwrap();
    let entity2 = result2.unwrap();

    entity1.eq(&entity2)

}

/*----------------------------------------------------------------------------*/
