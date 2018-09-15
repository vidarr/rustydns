/*
 * (C) 2018 Michael J. Beer
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

use ::std::str::FromStr;
use rustydns::{Name,Record,Zone};
/*----------------------------------------------------------------------------*/

fn check_zone_add(zone: &mut Zone, name_str: &str, record_str: &str) -> bool {

    let name = match Name::from_str(name_str) {
        Ok(n) => n,
        Err(_) => return false,
    };

    let record = match Record::from_str(record_str) {
        Ok(r) => r,
        Err(_) => return false,
    };

    if zone.add(name, record).is_err() {
        return false;
    }

    let name = match Name::from_str(name_str) {
        Ok(n) => n,
        Err(_) => return false,
    };

    let record = match Record::from_str(record_str) {
        Ok(r) => r,
        Err(_) => return false,
    };

    Option::Some(&record).eq(&zone.lookup(&name))

}

/*----------------------------------------------------------------------------*/

#[test]
fn test_zone_add_record() {

    let mut z = Zone::new();

    assert!(check_zone_add(&mut z, "ubeer.org", "A 1.2.3.4"));
    assert!(! check_zone_add(&mut z, "ubeer.org", "A 1.2.3.4"));
    check_zone_add(&mut z, "org", "A 5.6.7.8");
    check_zone_add(&mut z, "ubeer", "A 9.8.7.6");

    let mut zone_string = String::new();
    z.write(&mut zone_string).expect("Could not format zone");

    println!("{}\n",  zone_string);


}

/*----------------------------------------------------------------------------*/

#[test]
fn test_zone_add_from_str() {

    let mut zone = Zone::new();
    let name = Name::from_str("ubeer.org").unwrap();
    let record = Record::from_str("A 1.2.3.4").unwrap();

    let entry_string = format!("{} {}", name.to_string() , record.to_string());

    assert!(zone.add_from_str(&entry_string).is_ok());
    assert_eq!(zone.lookup(&name), Option::Some(&record));

    assert_eq!(zone.lookup(&Name::from_str("Not_In_Zone").unwrap()), Option::None);

    let name = Name::from_str("org").unwrap();
    let record = Record::from_str("PTR ubeer.org").unwrap();

    let entry_string = format!("{} {}", name.to_string(), record.to_string());

    // Should not be possible since 'org' is already a sub-zone
    assert!(zone.add_from_str(&entry_string).is_err());

}

/*----------------------------------------------------------------------------*/

// fn main() {
// 
//     check_zone_add_record();
// }
