use crate::ressources::binary_fun::bin2dec;
use super::position_tool::{modulo, get_cpr_lat, get_cpr_lon, get_t, nl_calcul, get_alt};

const NZ: f32 = 15.;


//calcul of latitude zone index
fn j_calcul(&cpr_lat_even : &f32, &cpr_lat_odd: &f32) -> f32 {
    return (((59. * &cpr_lat_even - 60. * &cpr_lat_odd) as f32) + 0.5).floor();
}

//calcul des coordonnées
pub fn coor_global(even_data: &[bool; 56], odd_data: &[bool; 56]) -> (f32,f32) {
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

    let lon_even = d_lon_even * (modulo(&m, &n_even) + cpr_lon_even);
    let lon_odd = d_lon_odd * (modulo(&m, &n_odd) + cpr_lon_odd);

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
    let alt: u64 = 
        if *q_bit { alt_dec * 25 - 1000 }
        else { alt_dec * 100 - 1000 };
    return alt.try_into().expect("overflow with altitude");
}

pub fn altitude_gnss (data: &[bool]) -> u32 {                 //TC between 20 and 22
    return bin2dec(get_cpr_lat(data)).try_into().expect("overflow with altitude");
}