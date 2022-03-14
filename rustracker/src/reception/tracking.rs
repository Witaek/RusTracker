#![allow(dead_code)]
#![allow(unused_variables)]

use std::collections::HashMap;
use crate::object::plane::Plane;
use crate::object::squitter::Squitter;
use super::sampling::init_device;
use super::sampling::extraction;
use super::sampling::amp;
use super::sampling::sample2binary;
use std::collections::hash_map::Entry::{Occupied, Vacant};
use num_complex::Complex;
use std::io::{self, BufReader, BufWriter, Read, Write, ErrorKind};
use std::fs::File;


pub struct Track {
    track_list: HashMap<String,Plane>,
}

impl Track {
    pub fn new () -> Self {
        Self {track_list: HashMap::new()}
    }

    pub fn tracking(&mut self, channel: usize)-> () {
        let source = init_device(channel);
        let mut stream = source.rx_stream::<Complex<f32>>(&[channel]).unwrap();
        let mut buf = vec![Complex::new(0.0, 0.0); stream.mtu().unwrap()];
        stream.activate(None).expect("failed to activate stream");

        loop {
            let read_size = buf.len();
            //stream.read return the nomber of samples read in addition to start the reading
            let buf_len = stream.read(&[&mut buf[..read_size]], 1_000_000).expect("read failed");
            let samples = amp(&buf[..buf_len]);
            self.add_track(samples);
        } 
    }

    fn add_track(&mut self, samples: Vec<f64>) ->() {
        let binaries = sample2binary(extraction(samples));
        for s in binaries {
            self.update_track(s);
        }
    }

    fn update_track(&mut self, s: Squitter) {
        //cette fonction doit mettre à jour ou ajouter un avion (Plane) de l'attribut tracklist de self
        if s.get_df()==17 {
            let plane = match self.track_list.entry(s.get_adress()) {
                Vacant(entry) => entry.insert(Plane::new(&s)),
                Occupied(entry) => entry.into_mut(),
            };
            plane.update_plane(s);
            plane.display();
        }
    }
}