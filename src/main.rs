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
extern crate rustydns;

use ::std::str::FromStr;
use rustydns::{Name,Record,Zone};
use ::std::io::Write;

/*----------------------------------------------------------------------------*/

fn insert(zone: &mut Zone, name: &str, record: &str) -> Result<(), &'static str> {

        let n = Name::from_str(name)?;
        let r = Record::from_str(record)?;

        zone.add(n, r)?;

        Result::Ok(())

}

/*----------------------------------------------------------------------------*/

fn check_zone_add_record() {

    let mut z = Zone::new();
    let name = Name::from_str("ubeer.org").unwrap();
    let record = Record::from_str("A 1.2.3.4").unwrap();

    z.add(name, record);

    insert(&mut z, "www.ubeer.org", "A 8.8.8.8");
    insert(&mut z, "mail.ubeer.org", "PTR ubeer.org");

    println!("{}", z);

    let mut s = String::new();
    z.write(&mut s);
    println!("{}", s);

}

/*----------------------------------------------------------------------------*/

fn main() {

    check_zone_add_record();
}

