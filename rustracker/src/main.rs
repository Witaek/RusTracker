#![allow(unused_imports)]

use crate::data_treatment::speed::speed;


//declaration des modules
mod data_treatment;
mod object;
mod ressources;
mod reception;
use crate::object::squitter::Squitter;
use crate::reception::tracking::Track;
mod test;
use std::{thread, time};
use zmq::*;



//main
fn main() {
    let ctx = Context::new();
    let addr = "tcp://127.0.0.1:5500";
    let sock = ctx.socket(zmq::PULL).unwrap();
    sock.bind(addr).unwrap();

    let mut radar1 = Track::new(sock);
    //ctx is given now and will be send through methods of Track to Notice::send
    radar1.tracking();

}