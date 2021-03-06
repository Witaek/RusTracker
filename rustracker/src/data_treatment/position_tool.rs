#![allow(dead_code)]

//parsing function
pub fn get_tc(data: &[bool]) -> &[bool]{                //get type code
    return &data[0..5]
}

pub fn get_ss(data: &[bool]) -> &[bool]{                //get Surveillance status
    return &data[5..7]
}

pub fn get_saf(data: &[bool]) -> &bool{                 //get Single antenna flag
    return &data[7]
}

pub fn get_alt(data: &[bool]) -> &[bool]{               //get Encoded altitude
    return &data[8..20]
}

pub fn get_t(data: &[bool]) -> &bool{                   //get time
    return &data[20]
}

pub fn get_f(data: &[bool]) -> &bool{                   //get cpr format
    return &data[21]
}

pub fn get_cpr_lat(data: &[bool]) -> &[bool]{           //get cpr latitude
    return &data[22..39]
}

pub fn get_cpr_lon(data: &[bool]) -> &[bool]{           //get cpr longitude
    return &data[39..56]
}


//modulo in rust return negative value, so must be redefine
pub fn modulo(x: &f32,y: &f32) -> f32 {
    return x-y*(x/y).floor();
}

//NL return the nomber of longitude zone corresponding with the latitude
const PI: f32 = std::f32::consts::PI;
const NZ: f32 = 15.;

pub fn nl_calcul(&lat: &f32) -> f32 {
    if lat == 87. || lat == -87. {
        return 2.;
    } else if lat == 87. || lat == -87. {
        return 1.;
    } else {
        let a= 2. * PI;
        let b = PI / (2. * NZ);
        let c = lat * (PI / 180.);
        return (a/(1.- ( 1.- b.cos()) / c.cos().powi(2) ).acos()).floor();
    };
}


pub fn angle(old: (f32,f32), new: (f32,f32) ) -> f32 {

    let phi1 = old.0*PI/180.;
    let phi2 = new.0*PI/180.;
    
    let lmb1 = old.1*PI/180.;
    let lmb2 = new.1*PI/180.;
    
    let y = (lmb2-lmb1).sin() * phi2.cos();
    let x = phi1.cos()*phi2.sin() - phi1.sin()*phi2.cos()*(lmb2-lmb1).cos();
    let res = y.atan2(x);
    
    return (res*180./PI + 360.)%360.
    
}