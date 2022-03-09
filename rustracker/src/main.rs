#![allow(unused_imports)]

//declaration des modules
mod data_treatment;
mod object;
mod ressources;
mod reception;
use crate::object::squitter::Squitter;
use crate::reception::tracking::Track;
mod test;

//main
fn main() {
    let mut radar1 = Track::new();
    radar1.tracking();
}