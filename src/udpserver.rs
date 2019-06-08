/*
 * (C) 2019 Michael J. Beer
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
use std::net::SocketAddr;
use std::collections::VecDeque;
use std::sync::{Arc, Mutex};

use udp::{MAX_SAFE_UDP_PAYLOAD_LEN, Handler, Message};

/*----------------------------------------------------------------------------*/

pub struct UdpServer<'a> {

    poll : Poll,
    listen_socket : UdpSocket,
    out_queue : Arc<Mutex<VecDeque<Message>>>,
    handler : &'a Handler,

}

/*----------------------------------------------------------------------------*/

/**
 * TODO: due to edge trigger mode, data is currently not read reliably
 */
impl<'a> UdpServer<'a> {

    pub fn bind_to<UH : Handler> (
        listen_addr_str : &str,
        handler : &'a UH,
        queue : Option<Arc<Mutex<VecDeque<Message>>>>)
        -> Result<UdpServer<'a>, &'static str> {

            let max_queue_len = 200;

            let listen_addr = match listen_addr_str.parse() {

                Ok(addr) => addr,
                Err(_) => return Err("Could not parse address string")

            };

            let listen_socket = match UdpSocket::bind(&listen_addr) {
                Ok(s) => s,
                Err(_) => return Err("Could not bind to socket")
            };

            let poll = match Poll::new() {
                Ok(p) => p,
                Err(_) => return Err("could not create mio:Poll")
            };

            if poll.register(
                &listen_socket,
                Token(0),
                Ready::readable() | Ready::writable(),
                PollOpt::edge()).is_err() {

                return Err("could not register listening socket");

            };

            let out_queue = match queue {
                None => Arc::new(Mutex::new(VecDeque::with_capacity(max_queue_len))),
                Some(q) => q.clone(),
            };

            Ok(UdpServer {poll, listen_socket, out_queue, handler})

        }

    pub fn run(&self) {

        let mut events = Events::with_capacity(1024);
        let timeout = Duration::from_millis(500);

        loop {

            let result = self.poll.poll(&mut events, Some(timeout));

            if result.is_err() {
                println!("Exception occured during polling");
                continue;
            };


            for event in &events {

                let readiness = event.readiness();

                if readiness.is_readable() {
                    int_handle_read(self);
                }

                if readiness.is_writable() {

                    if int_handle_write(self).is_err() {
                        println!("Sending of data failed");
                    }

                }

            }

        }

    }

    pub fn send(&self, dest : SocketAddr, num_bytes : usize, buffer : [u8; 512]) {

        self.out_queue.lock().unwrap().push_front(Message{
            addr : dest,
            num_bytes : num_bytes,
            buffer : buffer,
        });

    }

}

/*----------------------------------------------------------------------------*/

fn int_handle_read(udp_server : &UdpServer) {

    println!("Incoming data:");

    let mut buffer = [0; MAX_SAFE_UDP_PAYLOAD_LEN];

    match udp_server.listen_socket.recv_from(&mut buffer) {
        Ok((num_bytes, addr)) => {
            udp_server.handler.handle(Message{addr, buffer, num_bytes});
        }
        //    udp_server.handler.handle(udp_server, soa, num_bytes, buffer),
        Err(_) => println!("Did not receive data"),

    };
}

/*----------------------------------------------------------------------------*/

fn int_handle_write(udp_server : &UdpServer) -> ::std::io::Result<usize> {

    println!("Ready to write");

    let mut out_queue = udp_server.out_queue.lock().unwrap();

    let bytes_sent = match out_queue.pop_back() {
        None => 0,
        Some(msg) => {
            let slice_to_send = &msg.buffer[0 .. msg.num_bytes];
            udp_server.listen_socket.send_to(slice_to_send, &msg.addr)?
        }
    };

    Ok(bytes_sent)
}

/*----------------------------------------------------------------------------*/

