#![allow(dead_code)]
#![allow(unused_variables)]

use std::collections::HashMap;
use crate::object::plane::Plane;
use crate::object::squitter::Squitter;
use super::sampling::init_device;
use super::sampling::extraction;
use super::sampling::amp;
use super::sampling::sample2binary;


const CHUNKS_NUMBER: u32 = 16;
const BYTES_NUMBER: u32 = 32_000;


pub struct Track {
    track_list: HashMap<String,Plane>,
}

impl Track {
    pub async fn tracking(&mut self)-> () {
        let (ctl, mut rdr) = init_device();

        rdr.read_async(CHUNKS_NUMBER, BYTES_NUMBER, |bytes| {self.add_track(amp(bytes));}).unwrap();
    }

    fn add_track(&mut self, samples: Vec<f32>) ->() {
        let binaries = sample2binary(extraction(samples));
        for s in binaries {
            self.update_track(s);
        }
    }

    fn update_track(&mut self, s: Squitter) {
        //cette fonction doit mettre à jour ou ajouter un avion (Plane) de l'attribut tracklist de self
    }

}