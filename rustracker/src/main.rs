#![allow(unused_imports)]

use crate::data_treatment::speed::speed;


//declaration des modules
mod data_treatment;
mod object;
mod ressources;
mod reception;
mod stream;
use crate::object::squitter::Squitter;
use crate::reception::tracking::Track;
mod test;
use crate::reception::sampling::amp;
use std::{thread, time};
use url::Url;
use tungstenite::*;


fn print_type_of<T>(_: &T) {
    println!("{}", std::any::type_name::<T>())
}
//main
fn main() {

    let mut socket = connect(Url::parse("ws://localhost:8080").unwrap()).expect("Can't connect").0;


    println!("Connected to the server");
    
    let mut radar1 = Track::new();

    //ctx is given now and will be send through methods of Track to Notice::send
    radar1.tracking(0, &socket);

}