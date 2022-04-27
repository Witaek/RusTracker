mod reception;
mod object;
mod ressources;

use num_complex::Complex;
use reception::sampling::{amp, sample2binary, extraction, init_device};
use zmq::{Context, Message};

fn main() {
    let ctx = Context::new();
    let addr = "tcp://157.159.195.63:5500";
    let sock = ctx.socket(zmq::PUSH).unwrap();
    sock.connect(addr).unwrap();
    tracking(0, &sock);
}

pub fn tracking(channel: usize, sock : &zmq::Socket)-> () {
    let source = init_device(channel);
    let mut stream = source.rx_stream::<Complex<f32>>(&[channel]).unwrap();
    let mut buf = vec![Complex::new(0.0, 0.0); stream.mtu().unwrap()];
    stream.activate(None).expect("failed to activate stream");

    loop {
        let read_size = buf.len();
        //stream.read return the nomber of samples read in addition to start the reading
        let buf_len = stream.read(&[&mut buf[..read_size]], 1_000_000).expect("read failed");
        let samples = amp(&buf[..buf_len]);
        send_squitter(samples, sock);
    } 
}

fn send_squitter(samples: Vec<f64>, socket : &zmq::Socket) ->() {
    let binaries = sample2binary(extraction(samples));
    for s in binaries {
        println!("sending binary");
        let data = s.convert();
        let msg = Message::from(&data[..]);
        socket.send(msg, 0).unwrap();
    }
}