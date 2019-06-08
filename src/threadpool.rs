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
use std::cell::RefCell;
use std::thread::{spawn, JoinHandle};
use std::sync::{Arc,Mutex,Condvar};
use std::sync::atomic::{Ordering, AtomicBool};
use std::collections::VecDeque;

use udp::{ContinueState, Handler, Message};


/*----------------------------------------------------------------------------*/

pub struct SyncQueue<T> {
    queue : Mutex<VecDeque<T>>,
    cond_var : Condvar,
}

/*----------------------------------------------------------------------------*/

impl<T> SyncQueue<T> {

    fn new(capacity : usize) -> SyncQueue<T> {
        SyncQueue {
            queue : Mutex::new(VecDeque::with_capacity(capacity)),
            cond_var : Condvar::new(),
        }
    }

    pub fn enqueue(&self, msg : T) {
        self.queue.lock().unwrap().push_front(msg);
        self.cond_var.notify_one();
    }

    pub fn deque(&self) -> T {
        let mut guard = self.queue.lock().unwrap();
        loop {
            println!("Poll...");
            match guard.pop_back() {
                Some(msg) => return msg,
                None => guard = self.cond_var.wait(guard).unwrap(),
            }
        }
    }

}

/*----------------------------------------------------------------------------*/

pub struct Threadpool<H: Handler + Send + Sync> {
    stop : Arc<AtomicBool>,
    handler : Arc<H>,
    in_queue : Arc<SyncQueue<Message>>,
    threads : RefCell<Vec<JoinHandle<()>>>,
}

/*----------------------------------------------------------------------------*/

impl<H> Threadpool<H>
where H: 'static + Handler + Send + Sync {

    pub fn new(handler : Arc<H>, max_queue_len : usize)
        -> Threadpool<H> {

            Threadpool {
                stop : Arc::new(AtomicBool::new(false)),
                handler : handler,
                in_queue : Arc::new(SyncQueue::new(max_queue_len)),
                threads : RefCell::new(Vec::new()),

            }

        }

    pub fn run(&self, num_threads : usize) {

        for i in 1 .. num_threads {

            let in_queue = self.in_queue.clone();
            let handler = self.handler.clone();
            let stop = self.stop.clone();

            let thread = spawn(move || {
                loop {
                    let msg = in_queue.deque();

                    println!("thread {}", i);
                    match handler.handle(msg) {
                        ContinueState::Stop => 
                            stop.store(true, Ordering::Relaxed),
                        _ => {},
                    }

                    if stop.load(Ordering::Relaxed) {
                        break;
                    }
                }
            } );

            self.threads.borrow_mut().push(thread);
        }

    }

    pub fn join(&self) {

        let mut threads = self.threads.borrow_mut();

        for thread in threads.drain(..) {
            thread.join().unwrap();
        }

    }

}

/*----------------------------------------------------------------------------*/

impl <H> Handler for Threadpool<H>
where H: 'static + Handler + Send + Sync {

    fn handle (&self, msg : Message) -> ContinueState {

        self.in_queue.enqueue(msg);
        ContinueState::Continue

    }

}

/*----------------------------------------------------------------------------*/
