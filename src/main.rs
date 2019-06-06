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
extern crate mio;

mod netio;
use self::netio::{MAX_SAFE_UDP_PAYLOAD_LEN, UdpHandler, bind_to, setup_poll, run_poll};
use std::net::SocketAddr;

/*----------------------------------------------------------------------------*/

struct DummyHandler {

}

/*----------------------------------------------------------------------------*/

impl UdpHandler for DummyHandler {

    fn handle (&self, addr : SocketAddr, bytes_used : usize, buffer : [u8; MAX_SAFE_UDP_PAYLOAD_LEN]) {
        let data_str = match std::str::from_utf8(&buffer[..bytes_used]) {
            Err(_) => "Could not decode data",
            Ok(s) => s,
        };

        println!("{}", data_str);

    }

}

/*----------------------------------------------------------------------------*/

fn main() {

    let listen_addr_str = "127.0.0.1:1104";

    let listen_socket = match bind_to(listen_addr_str) {

        Ok(sock) => sock,
        Err(msg) => {
            println!("{}", msg);
            return;
        }

    };

    println!("Bound to {}", listen_addr_str);

    let poll = match setup_poll(&listen_socket) {
        Ok(p) => p,
        Err(msg) => {
            println!("{}", msg);
            return;
        }
    };

    run_poll(poll, listen_socket, DummyHandler{});

}

/*----------------------------------------------------------------------------*/
