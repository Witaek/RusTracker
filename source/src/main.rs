mod reception;
mod object;
mod ressources;

use num_complex::Complex;
use reception::sampling::{amp, sample2binary, extraction, init_device};
use zmq::{Context, Message};
use single_value_channel::channel_starting_with;
use chrono::{Local, DateTime, SecondsFormat};
use single_value_channel::Updater;
use std::{thread, time};

fn main() {
    let ctx = Context::new();
    //tcp://127.0.0.1:5500
    let addr = "tcp://127.0.0.1:5500";
    let sock = ctx.socket(zmq::PUSH).unwrap();
    sock.connect(addr).unwrap();
    let (mut receiver,updater) = channel_starting_with((0,0,Local::now().to_rfc3339_opts(SecondsFormat::Secs, false)));

    let time_wait = time::Duration::from_secs(60*10);
    thread::spawn (
        move || {
            loop {
                thread::sleep(time_wait);
                let stat = receiver.latest();
                println!("{} | nb_msg_true : {} | nb_msg_detected : {}", stat.2, stat.0, stat.1);
            }
        }
    );

    tracking(0, &sock, updater);
}

pub fn tracking(channel: usize, sock : &zmq::Socket, updater: Updater<(u32,u32,String)>)-> () {
    let stat = (0,0); //Local::now().to_rfc3339_opts(SecondsFormat::Secs, false)
    let source = init_device(channel);
    let mut stream = source.rx_stream::<Complex<f32>>(&[channel]).unwrap();
    let mut buf = vec![Complex::new(0.0, 0.0); stream.mtu().unwrap()];
    stream.activate(None).expect("failed to activate stream");

    loop {
        let read_size = buf.len();
        //stream.read return the nomber of samples read in addition to start the reading
        let buf_len = stream.read(&[&mut buf[..read_size]], 1_000_000).expect("read failed");
        let samples = amp(&buf[..buf_len]);
        let (n_true, n_tot) = send_squitter(samples, sock);
        updater.update((n_true, n_tot, Local::now().to_rfc3339_opts(SecondsFormat::Secs, false))).unwrap();
    } 
}

fn send_squitter(samples: Vec<f64>, socket : &zmq::Socket) ->(u32,u32) {
    let (binaries,n_true,n_tot) = sample2binary(extraction(samples));
    for s in binaries {
        let data = s.convert();
        let msg = Message::from(&data[..]);
        socket.send(msg, 0).unwrap();
    }
    return (n_true,n_tot);
}