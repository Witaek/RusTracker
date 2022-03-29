#![allow(unused_imports)]

use crate::data_treatment::speed::speed;


//declaration des modules
mod data_treatment;
mod object;
mod ressources;
mod reception;
mod stream;
use crate::object::squitter::Squitter;
use crate::reception::tracking::Track;
mod test;
use crate::reception::sampling::amp;

use zmq::{Context, Message, Error};



//main
fn main() {
    
    let ctx = Context::new();





    let mut radar1 = Track::new();
    let addr = "tcp://127.0.0.1:1234";

    let sock = ctx.socket(zmq::PUSH).unwrap();


    sock.connect(addr).unwrap();

    
    //ctx is given now and will be send through methods of Track to Notice::send
    radar1.tracking(0, &sock);
}