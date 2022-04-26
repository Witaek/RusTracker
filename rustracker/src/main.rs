#![allow(unused_imports)]
#![allow(unused_must_use)]

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

use std::fs::*;
use std::thread::*;
use std::io::{Write, SeekFrom, Seek};

use std::sync::mpsc::{channel, Sender, Receiver};





fn main() {

    let ctx = Context::new();
    let addr = "tcp://127.0.0.1:5500";
    let sock = ctx.socket(zmq::PULL).unwrap();
    sock.bind(addr).unwrap();
    
    //use to receive the geojson
    let (sender_msg,receiver_msg) : (Sender<String>, Receiver<String>) = channel();

    //use to request an update of the tracks to remove the old ones
    let (sender_rm,receiver_rm) : (Sender<bool>, Receiver<bool>) = channel();

    let mut radar1 = Track::new(sock, sender_msg, receiver_rm);

    //create the geojson
    let mut file = OpenOptions::new()
        .write(true)
        .truncate(true)
        .create(true)
        .open("planes.geojson")
        .unwrap();
    
    let time_wait = time::Duration::from_secs(30);
    //removing old tracks
    thread::spawn (
        move || {
            loop {
                thread::sleep(time_wait);
                sender_rm.send(true);
            }
        }
    );

    //editing the geoJSON
    thread::spawn ( 
        move || {
            loop{
                thread::sleep(time::Duration::from_millis(1000));
                let json_content = receiver_msg.recv().unwrap();
                file.set_len(0);
                file.seek(SeekFrom::Start(0));
                file.write_all(json_content.as_bytes()).unwrap();
            }
        }
    );
    
    radar1.tracking();
}