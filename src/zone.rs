// Copyright (c) 2018, Michael J. Beer
// All rights reserved.

// Redistribution and use in source and binary forms, with or without
// modification, are permitted provided that the following conditions are met:

// * Redistributions of source code must retain the above copyright notice, this
//   list of conditions and the following disclaimer.

// * Redistributions in binary form must reproduce the above copyright notice,
//   this list of conditions and the following disclaimer in the documentation
//   and/or other materials provided with the distribution.

// * Neither the name of the copyright holder nor the names of its
//   contributors may be used to endorse or promote products derived from
//   this software without specific prior written permission.

// THIS SOFTWARE IS PROVIDED BY THE COPYRIGHT HOLDERS AND CONTRIBUTORS "AS IS"
// AND ANY EXPRESS OR IMPLIED WARRANTIES, INCLUDING, BUT NOT LIMITED TO, THE
// IMPLIED WARRANTIES OF MERCHANTABILITY AND FITNESS FOR A PARTICULAR PURPOSE ARE
// DISCLAIMED. IN NO EVENT SHALL THE COPYRIGHT HOLDER OR CONTRIBUTORS BE LIABLE
// FOR ANY DIRECT, INDIRECT, INCIDENTAL, SPECIAL, EXEMPLARY, OR CONSEQUENTIAL
// DAMAGES (INCLUDING, BUT NOT LIMITED TO, PROCUREMENT OF SUBSTITUTE GOODS OR
// SERVICES; LOSS OF USE, DATA, OR PROFITS; OR BUSINESS INTERRUPTION) HOWEVER
// CAUSED AND ON ANY THEORY OF LIABILITY, WHETHER IN CONTRACT, STRICT LIABILITY,
// OR TORT (INCLUDING NEGLIGENCE OR OTHERWISE) ARISING IN ANY WAY OUT OF THE USE
// OF THIS SOFTWARE, EVEN IF ADVISED OF THE POSSIBILITY OF SUCH DAMAGE.
//
use ::std::collections::HashMap;
use ::std::fmt;
use dnslabel::Label;
use dnsname::Name;
use dnsrecord::Record;
use std::str::FromStr;

/******************************************************************************
 *                                             TYPE
 ******************************************************************************/

enum ZoneEntry {

    Record(Record),
    Zone(Zone),
}

/*----------------------------------------------------------------------------*/

pub struct Zone {

    entries : HashMap<Label, ZoneEntry>,

}

/*----------------------------------------------------------------------------*/

impl<'a> Zone {


    pub fn new() -> Zone {

        Zone { entries : HashMap::new(),}

    }

    /*-----------------------------------------------------------------------*/

    /// Tries to find a record for name within the zone.
    pub fn lookup(&'a self, name : &Name) -> Option<&Record> {

        self.internal_lookup(name.to_slice())

    }

    /*-----------------------------------------------------------------------*/

    pub fn add(&mut self, name : Name, record : Record) -> Result<(), &'static str> {

        // Get rid of terminal empty label
        let labels = name.to_slice();
        let len = labels.len();
        if 1 > len {
            return Err("Require NAME");
        }

        self.internal_add(&labels[0 .. len - 1], record)

    }

    /*-----------------------------------------------------------------------*/

    pub fn write(&self, f: &mut fmt::Write) -> fmt::Result {
        self.internal_fmt(f, &[])
    }

    /*------------------------------------------------------------------------*/

    pub fn add_from_str(&mut self, s: &str) -> Result<(), &'static str> {

        // First version: use iterators:
        //  Guess its rather imperformant, but the simplest solution iI could come
        //  up with right now ...
        let mut parts = s.split_whitespace();
        let name_str = parts.next().ok_or("DNS name missing")?;
        let record_string = parts.collect::<Vec<_>>().join(" ");
        let name = Name::from_str(&name_str)?;
        let record = Record::from_str(&record_string)?;

        self.add(name, record)

    }

    /*-----------------------------------------------------------------------*/

    fn internal_lookup(&self, labels : &[Label]) -> Option<&Record> {

        match self.entries.get(&labels[0]){

            Some(ZoneEntry::Zone(zone)) => {
                zone.internal_lookup(&labels[1..])
            },
            Some(ZoneEntry::Record(record)) => Some(record),
            None => None

        }

    }

    /*-----------------------------------------------------------------------*/

    fn internal_add(&mut self, labels: &[Label], record : Record) -> Result<(), &'static str> {

        match labels.len() {

            0 => Err("Require name"),
            1 => {
                if self.entries.contains_key(&labels[0]) {
                    return Err("Entry already there")
                }
                self.entries.insert(labels[0].clone(), ZoneEntry::Record(record));
                Ok(())
            },
            _ => {

                match self.entries.get_mut(&labels[0]) {

                    Some(ZoneEntry::Zone(ref mut zone)) => return zone.internal_add(&labels[1..], record),
                    Some(ZoneEntry::Record(_)) => return Err("Record exists already"),
                    None => {},
                };
                let mut zone = Zone::new();
                zone.internal_add(&labels[1..], record)?;
                self.entries.insert(labels[0].clone(), ZoneEntry::Zone(zone));
                Ok(())
            }

        }

    }

    /*-----------------------------------------------------------------------*/

    fn internal_fmt(&self, f: &mut fmt::Write, labels: &[Label]) -> fmt::Result {

        for (t, e) in &self.entries {

            match e  {

                ZoneEntry::Zone(zone) => {
                    let mut labels = labels.to_vec();
                    labels.push(*t);
                    zone.internal_fmt(f, &labels)?;
                },
                ZoneEntry::Record(record) => write!(f, "{} {}\n", Name::labels_to_string(labels), record)?

            }

        }

        Ok(())

    }

}

/*-----------------------------------------------------------------------*/

impl fmt::Display for ZoneEntry {

    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {

        match self {
            ZoneEntry::Record(record) => write!(f, "{}", record),
            ZoneEntry::Zone(zone) => write!(f, "{}", zone)
        }

    }

}

/*-----------------------------------------------------------------------*/

impl fmt::Display for Zone {

    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {

        write!(f, "[");

        for (name, record) in &self.entries {

            write!(f, "{} {};", name, record);

        }

        write!(f, "]");

        Ok(())

    }

}

/*----------------------------------------------------------------------------*/
