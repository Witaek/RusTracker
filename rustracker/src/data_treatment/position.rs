const PI: f32 = std::f32::consts::PI;
const NZ: f32 = 15.;

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

//découpage de la data selon le format |0:TC|1:SS|2:SAF|3:ALT|4:T|5:F|6:CPR-LAT|7:CPR-LON|
//OPTI : certaines données peuvent être inutiles (donc supprimer leurs découpages)
//insipiré de la branche identification (A potentiellement retravailler)
fn decoupage(&bin : &u64) -> [String; 8] {
    let bin_str: String = format!("{bin:056b}");
    let mut res: [String; 8] = Default::default();
    
    res[0]=bin_str[0..5].to_owned();
    res[1]=bin_str[5..7].to_owned();
    res[2]=bin_str[7..8].to_owned();
    res[3]=bin_str[8..20].to_owned();
    res[4]=bin_str[20..21].to_owned();
    res[5]=bin_str[21..22].to_owned();
    res[6]=bin_str[22..39].to_owned();
    res[7]=bin_str[39..56].to_owned();
    return res;
}

//calcul des coordonnées
pub fn coor(&even_data: &u64, &odd_data: &u64) -> (f32,f32) {
    //constant declaration
    let d_lat_even = 360. / (4. * NZ);
    let d_lat_odd = 360. / (4. * NZ - 1.);

    //binary parts
    let even_data_tab = decoupage(&even_data);
    let odd_data_tab = decoupage(&odd_data);

    //cpr conversion
    let cpr_lat_even = (u32::from_str_radix(even_data_tab[6].as_str(), 2).unwrap() as f32) / 131072.;//ma value of (2^17)
    let cpr_lat_odd = (u32::from_str_radix(odd_data_tab[6].as_str(), 2).unwrap() as f32) / 131072.;
    let cpr_lon_even = (u32::from_str_radix(even_data_tab[7].as_str(), 2).unwrap() as f32) / 131072.;
    let cpr_lon_odd = (u32::from_str_radix(odd_data_tab[7].as_str(), 2).unwrap() as f32) / 131072.;

    //index j calcul
    let j = j_calcul(&cpr_lat_even, &cpr_lat_odd);

    let mut lat_even = d_lat_even * ((j % 60.) + cpr_lat_even);
    let mut lat_odd = d_lat_odd * ((j % 60.) + cpr_lat_odd);

    //value correction
    if lat_even >= 270. {lat_even -= 360.};
    if lat_odd >= 270. {lat_odd -= 360.};

    //we keep the latitude of the most recent data according the time stamp
    let lat = 
        if u8::from_str_radix(odd_data_tab[4].as_str(), 2).unwrap() >= u8::from_str_radix(odd_data_tab[4].as_str(), 2).unwrap() {
            lat_even
        } else {
            lat_odd
        };
    
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
        if u8::from_str_radix(odd_data_tab[4].as_str(), 2).unwrap() >= u8::from_str_radix(odd_data_tab[4].as_str(), 2).unwrap() {
            lon_even
        } else {
            lon_odd
        };
    
    //value correction
    if lon >= 270. {lon -= 360.};

    return (lat, lon);
}

pub fn altitude_barometric (&data: &u64) -> u32 {           //TC between 9 and 18
    let mut alt_bin = decoupage(&data)[3].to_owned();
    

    let q_bit = alt_bin[7..8].to_owned();
    
    alt_bin.remove(7);

    let alt_dec = u32::from_str_radix(alt_bin.as_str(), 2).unwrap();
    let alt: u32 = 
        if q_bit.eq("1") { alt_dec * 25 - 1000 }
        else { alt_dec * 100 - 1000 };
    return alt;
}

pub fn altitude_gnss (&data: &u64) -> u32 {                 //TC between 20 and 22
    let alt_bin = decoupage(&data)[3].to_owned();
    return u32::from_str_radix(alt_bin.as_str(), 2).unwrap();
}
