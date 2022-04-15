#![allow(dead_code)]
use crate::object::squitter::Squitter;
use std::io::{self, BufReader, BufWriter, Read, Write, ErrorKind};
use num_complex::Complex;


pub fn init_device(channel: usize)-> soapysdr::Device {
    println!("init device");
    let source = soapysdr::Device::new("driver=rtlsdr").unwrap();
    source.set_frequency(soapysdr::Direction::Rx, channel, 1090_000_000_f64, "IGNORE").expect("failed to set frequency");
    source.set_sample_rate(soapysdr::Direction::Rx, channel, 2_000_000_f64).expect("failed to set sample rate");
    source.set_gain_mode(soapysdr::Direction::Rx, channel, true).expect("failed to set gain on auto");
    return source;
}

pub fn amp(iq_sample: &[Complex<f32>])-> Vec<f64> {
    let n = iq_sample.len();
    let mut samples: Vec<f64> = vec!();
    for i in 0..n {
        samples.push(iq_sample[i].norm() as f64);
    }
    return samples;   
}

pub fn extraction(samples: Vec<f64>)-> Vec<[f64;224]> {
    let mut packets: Vec<[f64;224]> = vec![];
    let n = samples.len();
    for i in 0..(n - (112*2 + 8*2) - 1){
        let av_max_value = (samples[i] + samples[i+2] + samples[i+7] + samples[i+9])/4.; //average value of the hifh amplitude in the preambule*
        // preambule logical detection
        if av_max_value < 0.05 {continue} //seuil de détection
        if  samples[i]    >  samples [i+1] &&
            samples[i+1]  <  samples [i+2] &&
            samples[i+1]  <  av_max_value &&
            samples[i+2]  >  samples[i+3] &&
            samples[i+3]  <  av_max_value &&
            samples[i+4]  <  av_max_value &&
            samples[i+5]  <  av_max_value &&
            samples[i+6]  <  av_max_value &&
            samples[i+6]  <  samples[i+7] &&
            samples[i+7]  >  samples[i+8] &&
            samples[i+8]  <  samples[i+9] &&
            samples[i+8]  <  av_max_value &&
            samples[i+9]  >  samples[i+10] &&
            samples[i+10] <  av_max_value &&
            samples[i+11] <  av_max_value &&
            samples[i+12] <  av_max_value &&
            samples[i+13] <  av_max_value &&
            samples[i+14] <  av_max_value &&
            samples[i+15] <  av_max_value  {
                packets.push(samples[(i+8*2)..(i+8*2+112*2)].try_into().expect("slice with incorrect length for packets"));
        }
    }
    return packets;
}

pub fn sample2binary(packets: Vec<[f64;224]>) -> Vec<Squitter> {
    let mut binaries: Vec<Squitter> = vec![];
    for sample in packets {
        let mut s = Squitter::default(); //initialisation d'un squitter remplie de 0
        for i in 0..112 {
            let a = sample[i*2];
            let b = sample[i*2 + 1];
            if a>b { //front descendant = 1
                s.msg[i] = true;
            } else if a==b {
                continue;
            }
        }
        if s.crc_check() {binaries.push(s);}
    }
    return binaries;
}