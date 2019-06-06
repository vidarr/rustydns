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

use mio::{Poll, Ready, Token, PollOpt, Events};
use mio::net::UdpSocket;
use std::time::Duration;

/*----------------------------------------------------------------------------*/

const MAX_SAFE_UDP_PAYLOAD_LEN : usize = 512;

/*----------------------------------------------------------------------------*/

pub enum Socket {
    Udp(UdpSocket),
    Tcp,
}

fn bind_to(listen_addr_str : &str) -> Result<Socket, &'static str> {

    let listen_addr : std::net::SocketAddr = match listen_addr_str.parse() {

        Ok(addr) => addr,
        Err(_) => return Err("Could not parse address string")

    };

    match UdpSocket::bind(&listen_addr) {
        Ok(s) => Ok(Socket::Udp(s)),
        Err(_) => Err("Could not bind to socket")
    }

}

/*----------------------------------------------------------------------------*/

fn setup_poll(listen_socket : &Socket) -> Result<Poll, &'static str> {

    let poll = match Poll::new() {
        Ok(p) => p,
        Err(_) => return Err("could not create mio:Poll")
    };

    let (token, ls) = match listen_socket {
        Socket::Udp(s) => (Token(0), s),
        Socket::Tcp => return Err("Unsupported socket type"),
    };

    if poll.register(ls, token, Ready::readable(), PollOpt::edge()).is_err() {
        return Err("could not register listening socket");
    };

    Ok(poll)

}

/*----------------------------------------------------------------------------*/

pub trait UdpHandler {

    fn handle (&self, [u8;512]);

}

/*----------------------------------------------------------------------------*/

fn run_poll<UH : UdpHandler> (poll : Poll, listen_socket : Socket, handler : UH) {

    let mut events = Events::with_capacity(1024);
    let timeout = Duration::from_millis(500);

    let ls = match listen_socket {
        Socket::Udp(s) => s,
        _ => {
            println!("Unsupported socket type");
            return;
        }

    };

    loop {

        let result = poll.poll(&mut events, Some(timeout));

        if result.is_err() {
            println!("Exception occured during polling");
            continue;
        };

        for event in &events {
            if Token(0) == event.token() {

                let mut buffer = [0; MAX_SAFE_UDP_PAYLOAD_LEN];

                println!("Incoming data:");

                match ls.recv_from(&mut buffer) {
                    Ok(_) => handler.handle(buffer),
                    Err(_) => println!("Did not receive data"),

                };

            }
        }

    }
}

/*----------------------------------------------------------------------------*/

struct DummyHandler {

}

/*----------------------------------------------------------------------------*/

impl UdpHandler for DummyHandler {
    fn handle(&self, buffer : [u8;512]) {

        let data_str = match std::str::from_utf8(&buffer) {
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
