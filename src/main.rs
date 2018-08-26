extern crate rustydns;
use rustydns::ToBytes;

/*------------------------------------------------------------------------*/

fn main() {

    let name1 = rustydns::Name::from_str("www.ubeer.org").unwrap();
    let name2 = rustydns::Name::from_str("www.ubeer.com").unwrap();

    println!("{}", name1);
    println!("{}", name2);
    // rustydns::print_dns_name(&name, &mut io::stdout());

    for (l1, l2) in name1.iter().zip(name2.iter()) {
        println!("{}", l1.eq(&l2));
    }

    let mut name_bytes = Vec::<u8>::new();
    match name1.to_bytes( &mut name_bytes) {
        Err(s) => println!("{}", s),
        Ok(_) => ()
    };

    for i in name_bytes {
        print!("{} ", i);
    }

    println!("");

}
