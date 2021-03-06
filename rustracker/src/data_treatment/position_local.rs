use crate::ressources::binary_fun::bin2dec;
use super::position_tool::{modulo, get_cpr_lat, get_cpr_lon, get_f, nl_calcul};

const NZ: f32 = 15.;




pub fn coor_local(data: &[bool; 56], lat_ref: &f32, lon_ref: &f32) -> Result<(f32,f32),String> {

    //calculation of longitude

    let i = if *get_f(data) {1.} else {0.};

    let d_lat = 360. / (4. * NZ - i);

    let cpr_lat = match bin2dec(get_cpr_lat(data)) {
        Ok(a) => a as f32 / 131072., //max value of (2^17)
        Err(a) => return Err(a)
    };  

    let j = (lat_ref/d_lat).floor() +  ( ( modulo(&lat_ref, &d_lat) / d_lat ) - cpr_lat + 0.5 ).floor();

    let lat = d_lat * (j + cpr_lat);


    //calculation of longitude

    let nl = nl_calcul(&lat);

    let d_lon = 360. / (nl - i).max(1.);

    let cpr_lon = match bin2dec(get_cpr_lon(data)) {
        Ok(a) => a as f32 / 131072., //max value of (2^17)
        Err(a) => return Err(a)
    };  

    let m = (lon_ref/d_lon).floor() +  ( ( modulo(&lon_ref, &d_lon) / d_lon ) - cpr_lon + 0.5 ).floor();

    let lon = d_lon * (m + cpr_lon);
    

    return Ok((lat,lon));
}
