#![allow(dead_code)]
use crate::object::squitter::Squitter;

pub fn init_device()-> (rtlsdr_mt::Controller, rtlsdr_mt::Reader) {
    let (mut ctl, rdr) = rtlsdr_mt::open(0).unwrap();
    ctl.set_center_freq(1090_000_000).unwrap();
    ctl.set_ppm(130).unwrap();
    ctl.set_sample_rate(2_000_000).unwrap();
    ctl.enable_agc().unwrap();
    println!("{}",ctl.center_freq());
    println!("{}",ctl.tuner_gain());


    return (ctl,rdr)
}

pub fn amp(bytes: &[u8])-> Vec<f64> {
    let n = bytes.len();
    let mut samples: Vec<f64> = vec!();
    for i in (0..(n-2)).step_by(2) {
        samples.push((((bytes[i] as u32).pow(2) + (bytes[i+1] as u32).pow(2)) as f64).sqrt())
    }
    return samples;
}

pub fn extraction(samples: Vec<f64>)-> Vec<[f64;224]> {
    let mut packets: Vec<[f64;224]> = vec![];
    let n = samples.len();
    for i in 0..(n - (112*2 + 8*2) - 1){
        let av_max_value = (samples[i] + samples[i+2] + samples[i+7] + samples[i+9])/4.; //average value of the hifh amplitude in the preambule*
        // preambule logical detection
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
        let mut s = Squitter::default();
        for i in 0..112 {
            let a = sample[i*2];
            let b = sample[i*2 + 1];
            if a>b {
                s.msg[i] = true;
            } else if a==b {
                continue;
            }
        }
        binaries.push(s);
    }
    return binaries;
}








