#![allow(dead_code)]

use crate::ressources::binary_fun::bin2dec;

const PI: f32 = std::f32::consts::PI;
const NZ: f32 = 15.;

fn modulo(x: &f32,y: &f32) -> f32 { //modulo in rust return negative value, so must be redefine
    return x-y*(x/y).floor();
}

//NL return the nomber of longitude zone corresponding with the latitude
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



//calcul of latitude zone index
fn j_calcul(&cpr_lat_even : &f32, &cpr_lat_odd: &f32) -> f32 {
    return (((59. * &cpr_lat_even - 60. * &cpr_lat_odd) as f32) + 0.5).floor();
}

//calcul des coordonnées
pub fn coor(even_data: &[bool; 56], odd_data: &[bool; 56]) -> (f32,f32) {
    //constant declaration
    let d_lat_even = 360. / (4. * NZ);
    let d_lat_odd = 360. / (4. * NZ - 1.);

    //cpr conversion
    let cpr_lat_even = (bin2dec(get_cpr_lat(even_data)) as f32) / 131072.;  //max value of (2^17)
    let cpr_lat_odd = (bin2dec(get_cpr_lat(odd_data)) as f32) / 131072.;
    let cpr_lon_even = (bin2dec(get_cpr_lon(even_data)) as f32) / 131072.;
    let cpr_lon_odd = (bin2dec(get_cpr_lon(odd_data)) as f32) / 131072.;

    //index j calcul
    let j = j_calcul(&cpr_lat_even, &cpr_lat_odd);

    let lat_even = d_lat_even * (modulo(&j,&60.) + cpr_lat_even);
    let lat_odd = d_lat_odd * (modulo(&j,&60.) + cpr_lat_odd);

    

    //we keep the latitude of the most recent data according the time stamp
    let mut lat = 
        if get_t(even_data) >= get_t(odd_data) {
            lat_even
        } else {
            lat_odd
        };
    
    //value correction
    if lat >= 270. {lat -= 360.};
    
    
    let nl = nl_calcul(&lat);

    //longitudinal index
    let m = (cpr_lon_even * (nl - 1.0) - cpr_lon_odd * nl + 0.5).floor();

    //longitude zone size
    let n_even = nl.max(1.);
    let lat_minus = lat - 1.;
    let n_odd = nl_calcul(&lat_minus).max(1.);

    let d_lon_even = 360. / &n_even;
    let d_lon_odd = 360. / &n_odd;

    let lon_even = d_lon_even * (m%n_even + cpr_lon_even);
    let lon_odd = d_lon_odd * (m%n_odd + cpr_lon_odd);

    //longitude calcul
    let mut lon = 
        if get_t(even_data) >= get_t(odd_data) {
            lon_even
        } else {
            lon_odd
        };
    
    //value correction
    if lon >= 180. {lon -= 360.};

    return (lat, lon);
}

pub fn altitude_barometric (data: &[bool]) -> u32 {           //TC between 9 and 18
    let alt_bin = get_alt(data);
    
    let q_bit = &alt_bin[7];
    
    let mut alt_bin_wq: [bool;11] = [true; 11];              //altitude binary without q bit
    
    for k in 0..11 {
        if k < 7 {
            alt_bin_wq[k] = alt_bin[k]
        } else if k > 7 {
            alt_bin_wq[k] = alt_bin[k+1]
        }
    }
    let alt_dec = bin2dec(&alt_bin_wq);
    let alt: u32 = 
        if *q_bit { alt_dec * 25 - 1000 }
        else { alt_dec * 100 - 1000 };
    return alt;
}

pub fn altitude_gnss (data: &[bool]) -> u32 {                 //TC between 20 and 22
    return bin2dec(get_cpr_lat(data));
}



//parsing function
fn get_tc(data: &[bool]) -> &[bool]{                //get type code
    return &data[0..5]
}

fn get_ss(data: &[bool]) -> &[bool]{                //get Surveillance status
    return &data[5..7]
}

fn get_saf(data: &[bool]) -> &bool{                 //get Single antenna flag
    return &data[7]
}

fn get_alt(data: &[bool]) -> &[bool]{               //get Encoded altitude
    return &data[8..20]
}

fn get_t(data: &[bool]) -> &bool{                   //get time
    return &data[20]
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
