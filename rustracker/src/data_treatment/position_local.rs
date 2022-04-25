#![allow(dead_code)]

use crate::ressources::binary_fun::bin2dec;

const PI: f32 = std::f32::consts::PI;
const NZ: f32 = 15.;

fn modulo(x: &f32,y: &f32) -> f32 { //modulo in rust return negative value, so must be redefine
    return x-y*(x/y).floor();
}

fn nl_calcul(&lat: &f32) -> f32 {
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


pub fn coor_local(data: &[bool; 56], lat_ref: &f32, lon_ref: &f32) -> (f32,f32) {

    //calculation of longitude

    let i = if *get_f(data) {1.} else {0.};

    let d_lat = 360. / (4. * NZ - i);

    let cpr_lat = (bin2dec(get_cpr_lat(data)) as f32) / 131072.;

    let j = (lat_ref/d_lat).floor() +  ( ( modulo(&lat_ref, &d_lat) / d_lat ) - cpr_lat + 0.5 ).floor();

    let lat = d_lat * (j + cpr_lat);

    //calculation of longitude

    let nl = 360. / nl_calcul(&lat);

    let d_lon = 360. / (nl - i).max(1.);

    let cpr_lon = (bin2dec(get_cpr_lon(data)) as f32) / 131072.;

    let m = (lon_ref/d_lon).floor() +  ( ( modulo(&lon_ref, &d_lon) / d_lon ) - cpr_lon + 0.5 ).floor();


    let lon = d_lon * (m + cpr_lon);

    return (lat,lon);
}

fn get_f(data: &[bool]) -> &bool{                   //get cpr format
    return &data[21]
}

fn get_cpr_lat(data: &[bool]) -> &[bool]{           //get cpr latitude
    return &data[22..39]
}

fn get_cpr_lon(data: &[bool]) -> &[bool]{           //get cpr longitude
    return &data[39..56]
}
