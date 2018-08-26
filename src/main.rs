extern crate rustydns;

/*------------------------------------------------------------------------*/

use std::io;

fn main() {

    let name = rustydns::Name::from_str("www.ubeer.org").unwrap();

    println!("{}", name);
    // rustydns::print_dns_name(&name, &mut io::stdout());

}
