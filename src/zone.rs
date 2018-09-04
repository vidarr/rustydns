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
use ::std::boxed::Box;
use ::std::collections::HashMap;
use dnslabel::Label;
use dnsname::Name;
use dnsrecord::Record;

/******************************************************************************
 *                                             TYPE
 ******************************************************************************/

enum ProcessState {

    Finished(Result<(), &'static str>),
    Ongoing

}

/*----------------------------------------------------------------------------*/

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

        Zone { entries : HashMap::new() }

    }

    /*-----------------------------------------------------------------------*/

    /// Tries to find a record for name within the zone.
    pub fn lookup(&'a self, name : &Name) -> Option<&Record> {

        self._lookup(name.to_slice())

    }

    /*-----------------------------------------------------------------------*/

    // pub fn add(&mut self, name : Name, record : Record) -> Result<(), &'static str> {

    //     self._add(name.to_slice(), record)

    // }

    /*-----------------------------------------------------------------------*/

    fn _lookup(&self, labels : &[Label]) -> Option<&Record> {

        match self.entries.get(&labels[0]){

            Some(ZoneEntry::Zone(zone)) => {
                zone._lookup(&labels[1..])
            },
            Some(ZoneEntry::Record(record)) => Some(record),
            None => None

        }

    }

    /*-----------------------------------------------------------------------*/

    // fn _add(&mut self, labels: &[Label], record : Record) -> Result<(), &'static str> {

    //     if labels.len() == 1 {

    //         self.entries.insert(labels[0].clone(), ZoneEntry::Record(record));
    //         Ok(())

    //     } else {

    //         let result = match self.entries.get_mut(&labels[0]) {

    //             Some(ZoneEntry::Zone(ref mut zone)) => ProcessState::AddZones(zone._add(&labels[1..], record)),
    //             Some(ZoneEntry::Record(_)) => ProcessState::Finished(Err("Record exists already")),
    //             None => ProcessState::Ongoing
    //         };

    //         match result {

    //             ProcessState::Finished(r) => r,
    //             ProcessState::Ongoing => {
    //                 // let zone = self._create_new_zone()
    //                 let mut zone = Zone::new();
    //                 zone._add(&labels[1..], record)?;
    //                 self.entries.insert(labels[0].clone(), ZoneEntry::Zone(zone));
    //                 Ok(())

    //             }

    //         }

    //     }

    // }

}

/*-----------------------------------------------------------------------*/
