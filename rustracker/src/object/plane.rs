#![allow(dead_code)]
#![allow(unused_variables)]

//CODE IN DEVELOPPEMENT [NOT WORKING]

use super::squitter::Squitter;
use crate::data_treatment::identification::callsign;
use crate::data_treatment::position::{coor, altitude_barometric, altitude_gnss};
use crate::data_treatment::speed::speed;
use std::net::*;

use geojson::{Feature, FeatureCollection, GeoJson, Geometry, Value, PointType};
use geojson::feature::Id;

use serde_json::{to_value, Map};

pub struct Plane {
    //definition of characteristic attributes of the plane
    icao: String,                               //icao address
    complement: String,                         //complementary information about the plane (from a database) [type string temporaire]
    callsign: String,                           //callsign of the flight  
    position: (f32,f32),                        //actual position of the plane (longitude, latitude)
    pos_flag: (bool,bool),                      //true if even and odd msg have been detected
    altitude: u32,                              //altitude of the plane
    speed: (f32, String, f32, String),                        //speed, track angle, vertical speed, speed type
    wake_vortex_cat: String,
    
    //past data
    position_history: Vec<(f32, f32, u32)>,     //historical of all past position
    trajectory: Vec<PointType>,                 //historical formated for geoJSON
    speed_history: Vec<(f32,String, f32,String)>,   //historical of all past speed
    
    //usefull binary msg or data
    data_pos: (Squitter,Squitter)               //tuple of most recent even and odd data from positional messages

    //[A PREVOIR] ajout de la gestion de la distance de l'avion à une ou plusieurs sources
}



//methods implementation
impl Plane {

    pub fn new(msg: &Squitter) -> Self { //constructeur à compléter [ACTUELLEMENT : POUR LES TEST]
        let mut n = Self {
            icao: msg.get_adress(),
            complement: "".to_owned(),
            callsign: "".to_owned(),
            position: (0.,0.),
            altitude: (0),
            speed: (0.0, String::from("N/A"), 0.0, String::from("")),
            wake_vortex_cat: "".to_owned(),
            
            position_history: vec![],
            trajectory: vec![],
            speed_history: vec![],
            
            data_pos: (Squitter::default(),Squitter::default()),
            pos_flag: (false,false),
        };
        n.set_wvc(msg);
        n.get_complement(msg);
        println!("nouvel avion");
        return n;
    }
}

impl Plane {

    pub fn update_plane(&mut self, msg: Squitter) -> () {
        //use a received Squitter to call the adequate fonction according to the type code
        match msg.get_tc() {
            1..=4 if self.callsign == String::from("") => {
                self.set_callsign(&msg);
                },
            9..=18 => {
                self.set_altitude_baro(&msg);
                self.pairing(msg);
                self.set_position();
                self.add_position();
                },
            20..=22 => {
                self.set_altitude_gnss(&msg);
                self.pairing(msg);
                self.set_position();
                self.add_position();
                },
            19 => {
                self.set_speed(&msg);
                self.add_speed();
                },
            _=>(),
        };
    }

    pub fn set_callsign(&mut self, msg: &Squitter) -> () {
        //update the callsign (using module data_treatment::identification)
        self.callsign = callsign(msg.get_data());
    }

    pub fn pairing(&mut self, msg: Squitter) -> () {
        //update the tuple self.data_pos with a new data
        if msg.msg[54]==false   {self.data_pos.0 = msg; self.pos_flag.0 = true; println!("even");}
        else                    {self.data_pos.1 = msg; self.pos_flag.1 = true; println!("odd");}

    }

    pub fn set_position(&mut self) -> () {
        //set the plane's position
        if self.pos_flag.0 && self.pos_flag.1 {     //if we have a even and odd message
            let (even_msg, odd_msg) = &self.data_pos;
            let even_data = even_msg.get_data();
            let odd_data = odd_msg.get_data();
            self.position = coor(even_data, odd_data);

            let p: PointType = vec![self.position.0 as f64,self.position.1 as f64];
            self.trajectory.push(p);
        }
    }

    pub fn add_position(&mut self) -> () {
        //add a position record to the history
        let (lat, lon) = self.position.clone();
        let alt = self.altitude.clone();
        self.position_history.push((lat,lon, alt));

        
    }

    pub fn set_altitude_baro(&mut self, msg: &Squitter) -> () {
        //set the plane's position
        self.altitude = altitude_barometric(msg.get_data());
    }

    pub fn set_altitude_gnss(&mut self, msg: &Squitter) -> () {
        //set the plane's position
        self.altitude = altitude_gnss(msg.get_data());
    }

    pub fn set_speed(&mut self, msg: &Squitter) -> () {
        self.speed = speed(&msg.get_data());
    }

    pub fn add_speed(&mut self) -> () {
        let (speed, vrtype, vr, speedtype) = self.speed.clone();
        self.speed_history.push((speed, vrtype, vr, speedtype));
    }

    pub fn get_complement(&self, msg: &Squitter) -> () {
        //call the database to have supplementary informations
    }

    pub fn set_wvc(&mut self, msg: &Squitter) -> () {
        //use the first squitter receive from a plane to get the wake vortex category
        let val: (u32,u32) = (msg.get_tc(), msg.get_ca());
        self.wake_vortex_cat=  match val {
            (2,1)       =>     "surface emergency vehicle".to_owned(),
            (2,3)       =>     "surface service vehicle".to_owned(),
            (2,4..=7)   =>     "ground obstruction".to_owned(),
            (3,1)       =>     "glider, sailplane".to_owned(),
            (3,2)       =>     "lighter-than-air".to_owned(),
            (3,3)       =>     "parachutist, skydiver".to_owned(),
            (3,4)       =>     "ultralight, hang-glider, paraglider".to_owned(),
            (3,5)       =>     "reserved".to_owned(),
            (3,6)       =>     "unmanned aerial vehicule".to_owned(),
            (3,7)       =>     "space or transatmospheric vehicle".to_owned(),
            (4,1)       =>     "light".to_owned(),
            (4,2)       =>     "medium 1".to_owned(),
            (4,3)       =>     "medium 2".to_owned(),
            (4,4)       =>     "high vortex aircraft".to_owned(),
            (4,5)       =>     "heavy".to_owned(),
            (4,6)       =>     "high performance".to_owned(),
            (4,7)       =>     "rotorcraft".to_owned(),
            _           =>     "Unknow".to_owned(),
        };
    }

    pub fn display(&self) -> () {
        //quick display of the plane information
        println!("---------------------------------------------------------------------");
        println!("icao :        {}", self.icao);
        println!("callsign :    {}", self.callsign);
        println!("wvc :         {}", self.wake_vortex_cat);
        println!("altitude :    {}", self.altitude);
        println!("position :    {:?}", self.position);
        println!("speed :       {} kt | {} : {} ft/min | {}", self.speed.0,self.speed.1, self.speed.2, self.speed.3);
    }

    pub fn into_feat(&self, adress: String) -> Feature {
        //return tuple of geojson::Feature first for the actual position second for the past trajectory
        let mut properties = Map::new();

        //add properties
        let alt_str = self.altitude.to_string();
        let speed_str = format!("{} kt | {} : {} ft/min | {}", self.speed.0,self.speed.1, self.speed.2, self.speed.3);
        properties.insert(self.callsign.clone(), to_value("callsign").unwrap());
        properties.insert(alt_str, to_value("altitude").unwrap());
        properties.insert(speed_str, to_value("speed").unwrap());

        //add set up geometry
        let geometry = Geometry::new(Value::LineString(self.trajectory.clone()));

        let feat = Feature {
            bbox: None,
            geometry: Some(geometry),
            id: Some(Id::String(adress)),
            properties: Some(properties),
            foreign_members: None,
        };
        return feat;
    }
}