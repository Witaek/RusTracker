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


const CHUNKS_NUMBER: u32 = 16;
const BYTES_NUMBER: u32 = 32000;


pub struct Track {
    track_list: HashMap<String,Plane>,
}

impl Track {
    pub fn new () -> Self {
        Self {track_list: HashMap::new()}
    }

    pub fn tracking(&mut self)-> () {
        let (ctl, mut rdr) = init_device();

        rdr.read_async(15, 32_000*2, |bytes| {self.add_track(amp(bytes));}).unwrap();
    }

    fn add_track(&mut self, samples: Vec<f64>) ->() {
        let binaries = sample2binary(extraction(samples));
        for s in binaries {
            self.update_track(s);
        }
    }

    fn update_track(&mut self, s: Squitter) {
        //cette fonction doit mettre à jour ou ajouter un avion (Plane) de l'attribut tracklist de self
        if s.crc_check() && s.get_df()==17 {
            println!("crc vrai");
            let plane = match self.track_list.entry(s.get_adress()) {
                Vacant(entry) => entry.insert(Plane::new(&s)),
                Occupied(entry) => entry.into_mut(),
            };
            plane.update_plane(s);
            plane.display();
        }
    }
}