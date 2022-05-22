#![allow(dead_code)]
#![allow(unused_variables)]

//CODE IN DEVELOPPEMENT [NOT WORKING]

use super::squitter::Squitter;
use crate::data_treatment::identification::callsign;
use crate::data_treatment::position_global::{coor_global, altitude_barometric, altitude_gnss};
use crate::data_treatment::position_local::{coor_local};
use crate::data_treatment::position_tool::{angle};
use crate::data_treatment::speed::speed;

use geojson::{Feature, Geometry, Value, PointType};
use geojson::feature::Id;

use serde_json::{to_value, Map};

use std::time::Instant;

pub struct Plane {
    //definition of characteristic attributes of the plane
    icao: String,                               //icao address
    complement: String,                         //complementary information about the plane (from a database) [type string temporaire]
    callsign: String,                           //callsign of the flight  
    position: (f32,f32),                        //actual position of the plane (longitude, latitude)
    pos_flag: (bool,bool,bool),                 //1st : true if even have been detected     2: true if odd msg have been detected       3: Global position have been set
    altitude: u32,                              //altitude of the plane
    speed: (f32, String, f32, String, f32),     //speed, track angle, vertical speed, speed type, track
    wake_vortex_cat: String,
    
    //past data
    position_history: Vec<(f32, f32, u32)>,     //historical of all past position
    trajectory: Vec<PointType>,                 //historical formated for geoJSON
    speed_history: Vec<(f32,String, f32,String, f32)>,   //historical of all past speed
    
    //usefull binary msg or data
    data_pos: (Squitter,Squitter),               //tuple of most recent even and odd data from positional messages
    i_traj: u8,

    //time of last message
    pub last_msg_time: Instant,

    //[A PREVOIR] ajout de la gestion de la distance de l'avion à une ou plusieurs sources
}



//methods implementation
impl Plane {

    pub fn new(msg: &Squitter) -> Self { //constructeur à compléter [ACTUELLEMENT : POUR LES TEST]
        let mut n = Self {
            icao: msg.get_adress().unwrap(),
            complement: "".to_owned(),
            callsign: "".to_owned(),
            position: (0.,0.),
            altitude: (0),
            speed: (0.0, String::from("N/A"), 0.0, String::from(""),0.),
            wake_vortex_cat: "".to_owned(),
            
            position_history: vec![],
            trajectory: vec![],
            speed_history: vec![],
            
            data_pos: (Squitter::default(),Squitter::default()),
            pos_flag: (false,false,false),
            i_traj: 0,

            last_msg_time: Instant::now(),
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
        match msg.get_tc().unwrap() {
            1..=4 if self.callsign == String::from("") => {
                self.set_callsign(&msg);
                },
            9..=18 => {
                self.set_altitude_baro(&msg);
                if self.pos_flag.2 {
                    self.set_local_position(&msg);
                } else {
                    self.pairing(msg);
                    self.set_global_position();
                }
                self.add_position();

                //check for absurd trajectory :
                if (2<self.i_traj)&&(self.i_traj<6) {
                    self.check_angle();
                }

                },
            20..=22 => {
                self.set_altitude_gnss(&msg);
                if self.pos_flag.2 {
                    self.set_local_position(&msg);
                } else {
                    self.pairing(msg);
                    self.set_global_position();
                }
                self.add_position();

                //check for absurd trajectory :
                if (2<self.i_traj)&&(self.i_traj<6) {
                    self.check_angle();
                }

                },
            19 => {
                self.set_speed(&msg);
                self.add_speed();
                },
            _=>(),
        };
        &self.set_time();
    }

    pub fn set_time(&mut self) {
        self.last_msg_time = Instant::now();
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

    pub fn set_global_position(&mut self) -> () {
        //set the plane's position
        if self.pos_flag.0 && self.pos_flag.1 {     //if we have a even and odd message
            let (even_msg, odd_msg) = &self.data_pos;
            let even_data = even_msg.get_data();
            let odd_data = odd_msg.get_data();
            match coor_global(even_data, odd_data) {
                Ok(a) => self.position = a,
                Err(a) => ()
            }

            let p: PointType = vec![self.position.1 as f64,self.position.0 as f64];
            self.trajectory.push(p);

            //can now decode local position
            self.pos_flag.2 = true;
        }
    }

    pub fn set_local_position(&mut self, msg: &Squitter) -> () {

        //set the plane's position
        let (lat_ref, lon_ref) = &self.position;
        match coor_local(msg.get_data(), lat_ref, lon_ref) {
            Ok(a) => self.position = a,
            Err(a) => ()
        };

        let p: PointType = vec![self.position.1 as f64,self.position.0 as f64];
        self.trajectory.push(p);
    }

    pub fn add_position(&mut self) -> () {

        //add a position record to the history
        let (lat, lon) = self.position.clone();
        let alt = self.altitude.clone();
        self.position_history.push((lat,lon, alt));

    }

    pub fn set_altitude_baro(&mut self, msg: &Squitter) -> () {
        //set the plane's position
        match altitude_barometric(msg.get_data()) {
        Ok(a) => self.altitude = a,
        Err(a) => ()
        };
    }

    pub fn set_altitude_gnss(&mut self, msg: &Squitter) -> () {
        //set the plane's position
        match altitude_gnss(msg.get_data()) {
            Ok(a) => self.altitude = a,
            Err(a) => ()
        };
    }

    pub fn set_speed(&mut self, msg: &Squitter) -> () {
        match speed(&msg.get_data()) {
            Ok(a) => self.speed =a,
            Err(a) => ()
        }
    }

    pub fn add_speed(&mut self) -> () {
        let (speed, vrtype, vr, speedtype, trackangle) = self.speed.clone();
        self.speed_history.push((speed, vrtype, vr, speedtype, trackangle));
    }

    pub fn get_complement(&self, msg: &Squitter) -> () {
        //call the database to have supplementary informations
    }

    pub fn set_wvc(&mut self, msg: &Squitter) -> () {
        //use the first squitter receive from a plane to get the wake vortex category
        let val: (u32,u32) = (msg.get_tc().unwrap(), msg.get_ca().unwrap());
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


        properties.insert(String::from("callsign"), to_value(self.callsign.clone()).unwrap());
        properties.insert(String::from("altitude"), to_value(self.altitude).unwrap());

        properties.insert(String::from("speed"), to_value(self.speed.0).unwrap());
        properties.insert(String::from("sp_tp"), to_value(self.speed.1.clone()).unwrap());
        properties.insert(String::from("sd_vt"), to_value(self.speed.2).unwrap());
        properties.insert(String::from("track"), to_value(self.speed.4).unwrap());

        //add set up geometry
        let geometry = Geometry::new(Value::LineString(self.trajectory.clone()));

        let feat = Feature {
            bbox: None,
            id: Some(Id::String(adress)),
            properties: Some(properties),
            foreign_members: None,
            geometry: Some(geometry),
        };
        return feat;
    }

    pub fn check_angle(&mut self) -> () {
        //check for absurd trajectory
        let n = self.position_history.len();
        let coor_old = (self.position_history[n-2].0, self.position_history[n-2].0);
        let coor_new = (self.position_history[n-1].0, self.position_history[n-1].0);
        let calculated_angle = angle(coor_old,coor_new);

        let res = match (calculated_angle,self.speed.4) {
            (a,b) if (((0.<a) &&(a<=180.) && (0.<b) && (b<=180.)) || ((180.<a) &&(a<360.) && (180.<b) && (b<360.))) => (a-b).abs(),
            (a,b) if (((0.<a) && (a<=180.) && (180.<b) &&(b<360.)) || ((180.<a) && (a<360.) && (0.<b) && (b<= 180.))) => (360.-a+b).abs(),
            _=>0.,
        };

        if res>90. {self.reset_position();}
        else {self.i_traj += 1}
    }

    pub fn reset_position(&mut self) {
        self.position = (0.,0.);
        self.position_history = vec![];
        self.trajectory = vec![];
        self.data_pos = (Squitter::default(),Squitter::default());
        self.pos_flag = (false,false,false);
        self.i_traj= 0;
    }
}
