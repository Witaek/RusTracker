#![allow(unused_imports)]

//declaration des modules
mod data_treatment;
mod object;
mod ressources;
use crate::object::squitter::Squitter;
mod test;

//main
fn main() {

    let (mut ctl, mut reader) = rtlsdr_mt::open(0).unwrap();

    ctl.enable_agc().unwrap();
    ctl.set_ppm(-2).unwrap();
    ctl.set_center_freq(774_781_250).unwrap();

    std::thread::spawn(move || {
        loop {
            let next = ctl.center_freq() + 1000;
            ctl.set_center_freq(next).unwrap();

            std::thread::sleep(std::time::Duration::from_secs(1));
        }
    });

    reader.read_async(4, 32768, |bytes| {
        println!("i[0] = {}", bytes[0]);
        println!("q[0] = {}", bytes[1]);
    }).unwrap();
}