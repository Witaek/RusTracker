#![allow(dead_code)]
#![allow(unused_variables)]

use std::collections::HashMap;
use crate::object::plane::Plane;
use crate::object::squitter::Squitter;
use std::collections::hash_map::Entry::{Occupied, Vacant};

use geojson::{FeatureCollection};


use std::sync::mpsc::Receiver;

use geojson::feature::Id;

use single_value_channel::Updater;


pub struct Track {
    track_list: HashMap<String,Plane>,
    sock: zmq::Socket,
    pub geojson : FeatureCollection,
    updater_msg : Updater<String>,
    receiver_rm : Receiver<bool>,
}


impl Track {
    pub fn new(sock: zmq::Socket, updater_msg : Updater<String>, receiver_rm : Receiver<bool>) -> Self {
        Self {
            track_list: HashMap::new(),
            sock,
            geojson : FeatureCollection {
                bbox: None,
                foreign_members: None,
                features: vec![],
            },
            updater_msg,
            receiver_rm,
        }
    }


    pub fn tracking(&mut self) {

        loop {

            //if we receive a message from receiver_rm this mean we have to check the tracks to remove the old ones
            if self.receiver_rm.try_recv().is_ok() {
                self.remove_old_track();
            };

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
        let adress = match s.get_adress() {
            Ok(a) => a,
            Err(a) => return ()
        };
        if s.get_df()==Ok(17) {
            let plane = match self.track_list.entry(adress.clone()) {
                Vacant(entry) => entry.insert(Plane::new(&s)),
                Occupied(entry) => entry.into_mut(),
            };
            plane.update_plane(s);
            plane.display();
            self.edit_geojson(adress);
        }
        self.updater_msg.update(self.geojson.to_string()).unwrap();
    }

    pub fn edit_geojson(&mut self, adress: String) {
        let mut flag = false;

        let plane_feat = self.track_list.get(&adress).unwrap().into_feat(adress);

        for i in 0..self.geojson.features.len() {
            //actualize the feature if the plane is known
            if self.geojson.features[i].id.as_ref().unwrap().eq(&plane_feat.id.as_ref().unwrap()) {
                self.geojson.features[i] = plane_feat.clone();
                flag = true;
                break;
            }
        }

        if !flag {
            //add the feature of a new plane
            self.geojson.features.push(plane_feat);
        }
    }

    pub fn remove_old_track(&mut self){

        let mut rm_id_list : Vec<String> = vec![];  //list of id to be remove

        for plane in &self.track_list {
            //set remove minimum to 30 seconds
            if plane.1.last_msg_time.elapsed().as_secs() >= 30 {

                rm_id_list.push(plane.0.clone());

                //actualise the geojson
                for i in 0..self.geojson.features.len() {
                    
                    if self.geojson.features[i].id.as_ref().unwrap().eq(&Id::String(plane.0.clone())) {
                        self.geojson.features.remove(i);
                        break;
                    }
                }
            }
        }

        //remove from the hashmap
        for id in rm_id_list {
            self.track_list.remove(&id);
        }
    }
}