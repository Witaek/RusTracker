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
    let addr = "tcp://157.195.159.63:5500";
    let sock = ctx.socket(zmq::PUSH).unwrap();
    sock.connect(addr).unwrap();
    

    
    

    tracking(0, &sock);
}

pub fn tracking(channel: usize, sock : &zmq::Socket)-> () {
    let mut stat = (0,0); //Local::now().to_rfc3339_opts(SecondsFormat::Secs, false)
    let source = init_device(channel);
    let mut stream = source.rx_stream::<Complex<f32>>(&[channel]).unwrap();
    let mut buf = vec![Complex::new(0.0, 0.0); stream.mtu().unwrap()];
    stream.activate(None).expect("failed to activate stream");

    let time_wait = time::Duration::from_secs(10);

    let (mut receiver,updater) = channel_starting_with((0,0,Local::now().to_rfc3339_opts(SecondsFormat::Secs, false)));

    thread::spawn (
        move || {
            loop {
                let stat = receiver.latest();
                println!("{} | nb_msg_true : {} | nb_msg_detected : {}", stat.2, stat.0, stat.1);
                thread::sleep(time_wait);
            }
        }
    );

    loop {
        let read_size = buf.len();
        //stream.read return the nomber of samples read in addition to start the reading
        let buf_len = match stream.read(&[&mut buf[..read_size]], 1_000_000)  {
            Ok(a)=>a,
            Err(a)=> {println!("{}",a); continue}
        };
        let samples = amp(&buf[..buf_len]);
        let (n_true, n_tot) = send_squitter(samples, sock);
        stat.0 += n_true;
        stat.1 += n_tot;
        updater.update((stat.0, stat.1, Local::now().to_rfc3339_opts(SecondsFormat::Secs, false))).unwrap();
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