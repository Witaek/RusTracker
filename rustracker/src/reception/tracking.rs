#![allow(dead_code)]
#![allow(unused_variables)]

use std::collections::HashMap;
use crate::object::plane::Plane;
use crate::object::squitter::Squitter;
use std::collections::hash_map::Entry::{Occupied, Vacant};
//use std::io::{self, BufReader, BufWriter, Read, Write, ErrorKind};
use std::net::*;
use std::borrow::Borrow;
use zmq::*;

use geojson::{Feature, FeatureCollection, GeoJson, Geometry, Value};
use serde_json::{Map, to_value, to_writer};

use tokio::io::{self, AsyncWriteExt};
use tokio::fs::File;
use tokio::time::{sleep, Duration};

use std::sync::mpsc::{channel, Sender, Receiver};

pub struct Track {
    track_list: HashMap<String,Plane>,
    sock: zmq::Socket,
    pub geojson : FeatureCollection,
    sender : Sender<String>,
}


impl Track {
    pub fn new(sock: zmq::Socket, sender : Sender<String>) -> Self {
        Self {
            track_list: HashMap::new(),
            sock,
            geojson : FeatureCollection {
                bbox: None,
                foreign_members: None,
                features: vec![],
            },
            sender,
        }
    }


    pub fn tracking(&mut self) {

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
        let adress = s.get_adress();
        if s.get_df()==17 {
            let plane = match self.track_list.entry(adress.clone()) {
                Vacant(entry) => entry.insert(Plane::new(&s)),
                Occupied(entry) => entry.into_mut(),
            };
            plane.update_plane(s);
            plane.display();
            self.edit_geojson(adress);
        }
    }

    pub fn edit_geojson(&mut self, adress: String) {
        let mut flag = false;

        let plane_feat = self.track_list.get(&adress).unwrap().into_feat(adress);

        for i in 0..self.geojson.features.len() {
            //actualize the feature if the plane is known
            if self.geojson.features[i].id.as_ref().unwrap().eq(&plane_feat.id.as_ref().unwrap()) {
                self.geojson.features[i] = plane_feat.clone();
                flag = true;
            }
        }

        if !flag {
            //add the feature of a new plane
            self.geojson.features.push(plane_feat);
        }
        self.sender.send(self.geojson.to_string()).unwrap();
    }
}