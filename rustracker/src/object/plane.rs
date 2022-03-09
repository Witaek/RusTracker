#![allow(dead_code)]

//CODE IN DEVELOPPEMENT [NOT WORKING]

use super::squitter::Squitter;
use crate::data_treatment::identification::callsign;
use crate::data_treatment::position::coor;
use crate::data_treatment::position::altitude_barometric;

pub struct Plane {
    //definition of characteristic attributes of the plane
    icao: String,                               //icao address
    complement: String,                         //complementary information about the plane (from a database) [type string temporaire]
    callsign: String,                           //callsign of the flight  
    position: (f32,f32),                        //actual position of the plane (longitude, latitude)
    altitude: u32,                              //altitude of the plane
    speed: (u32,f32,i32,String),                //speed, track angle, vertical speed, speed type
    wake_vortex_cat: String,
    
    //past data
    position_history: Vec<(f32, f32, u32)>,      //historical of all past position
    speed_history: Vec<(u32,f32,i32,String)>,   //historical of all past speed
    
    //usefull binary msg or data
    data_pos: (Squitter,Squitter)                   //tuple of most recent odd and even data from positional messages

    //[A PREVOIR] ajout de la gestion de la distance de l'avion à une ou plusieurs sources
}



//methods implementation
impl Plane {

    pub fn new(a: String) -> Self { //constructeur à compléter [ACTUELLEMENT : POUR LES TEST]
        Self {
            icao: a,
            complement: "".to_owned(),
            callsign: "".to_owned(),
            position: (0.,0.),
            altitude: (0),
            speed: (0,0.,0,"".to_owned()),
            wake_vortex_cat: "Unknow".to_owned(),
            
            position_history: vec![],
            speed_history: vec![],
            
            data_pos: (Squitter::default(),Squitter::default()),
        }
    }

    pub fn update_plane(&self, msg: Squitter) -> () {
        //use a received Squitter to call the adequate fonction according to the type code
    }

    pub fn set_callsign(&mut self, msg: Squitter) -> () {
        //update the callsign (using module data_treatment::identification)
        self.callsign = callsign(msg.get_data());
    }

    pub fn pairing(&self) -> () {
        //update the tuple self.data_pos with a new data
    }

    pub fn set_position(&mut self) -> () {
        //set the plane's position
        let (even_msg, odd_msg) = &self.data_pos;
        let even_data = even_msg.get_data();
        let odd_data = odd_msg.get_data();

        if (even_data != &[false; 56]) && (odd_data != &[false; 56]) {      //an array full of false is the default squitter [TEST TO BE OPTIMISED]
            self.position = coor(even_data, odd_data);
        }
    }

    pub fn add_position(&self) -> () {
        //add a position record to the history
    }

    pub fn set_altitude(&mut self, msg: Squitter) -> () {
        //set the plane's position
        self.altitude = altitude_barometric(msg.get_data());
    }

    pub fn add_speed(&self) -> () {
        //add a speed record to the history
    }

    pub fn get_complement(&self) -> () {
        //call the database to have supplementary informations
    }

    pub fn set_wvc(&mut self, msg: Squitter) -> () {
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
        println!("l'adress icao de l'avion est {}", self.icao);
    }
}