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


//distance
pub fn distance(coor_1: (f32,f32), coor_2: (f32,f32) ) -> u16 {
    let r: f32 = 6371.; //earth radius

    let u = (PI * (coor_2.0 - coor_1.0) / 360.).sin().powf(2.);
    let v = (PI * (coor_2.1 - coor_1.1) / 360.).sin().powf(2.);
    let n = ((PI/180.)*coor_1.0).cos() * ((PI/180.)*coor_2.0).cos();

    let d = (2. * r *  (u + n * v).sqrt().asin()) as u16;

    return d;
}
