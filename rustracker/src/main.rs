#![allow(unused_imports)]

//declaration des modules
mod data_treatment;
mod object;
mod ressources;
use crate::object::squitter::Squitter;

//main
fn main() {
    test_position();
    test_identification();
}

fn test_position() {

    let even_data = [false, true, false, true, true, false,
    false, false, true, true, false, false, false, false, true, 
    true, true, false, false, false, false, false, true, false, 
    true, true, false, true, false, true, true, false, true, false, 
    false, true, false, false, false, false, true, true, false, false, 
    true, false, false, false, true, false, true, false, true, true, false, false];

    let odd_data =  [false, true, false, true, true, false,
    false, false, true, true, false, false, false, false, true, true,
    true, false, false, false, false, true, true, false, false, true,
    false, false, false, false, true, true, false, true, false, true,
    true, true, false, false, true, true, false, false, false, true,
    false, false, false, false, false, true, false, false, true, false];

    let (a,b) = data_treatment::position::coor(&even_data, &odd_data);
    println!( "latitude : {}", a);
    println!( "longitude : {}", b);
    let c = data_treatment::position::altitude_barometric(&even_data);
    println!( "altitude : {}", c);
}

fn test_identification() {
    let bin = [false, false, true, false, false, false, false,
    false, false, false, true, false, true, true, false, false,
    true, true, false, false, false, false, true, true, false,
    true, true, true, false, false, false, true, true, true, false,
    false, false, false, true, true, false, false, true, false, true,
    true, false, false, true, true, true, false, false, false, false, false];   // binaire exemple

    let number = data_treatment::identification::callsign(&bin);
    println!("{}",number);
}
