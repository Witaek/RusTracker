#![allow(dead_code)]
#![allow(unused_variables)]

use std::collections::HashMap;
use crate::object::plane::Plane;
use crate::object::squitter::Squitter;
use std::collections::hash_map::Entry::{Occupied, Vacant};
use std::io::{self, BufReader, BufWriter, Read, Write, ErrorKind};
use std::fs::File;
use std::net::*;
use std::borrow::Borrow;
use zmq::*;


pub struct Track {
    track_list: HashMap<String,Plane>,
    sock: zmq::Socket,
}

impl Track {
    pub fn new (sock: zmq::Socket) -> Self {
        Self {
            track_list: HashMap::new(),
            sock,
        }
    }

    pub fn tracking(&mut self)-> () {

        loop {
            let msg = self.sock.recv_bytes(0);
            let s = match msg {
                Ok(data) => Squitter::from_msg(data),
                Err(data) => panic!("Erreur de reception"),
            };
            
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
            //self.edit_geojson(note)
        }
    }
}