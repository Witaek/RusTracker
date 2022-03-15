#![allow(dead_code)]
#![allow(unused_variables)]

use crate::ressources::binary_fun::bin2dec;
use std::convert::TryInto;



fn cut_in_sections(msg: &[bool; 56]) -> [&[bool];12] {
    let mut arr: [&[bool];12] = Default::default();      

    arr[0] = &msg[..5];         //TC
    arr[1] = &msg[5..8];        //Sub-type
    arr[2] = &msg[8..9];        //Intent change flag
    arr[3] = &msg[9..10];       //IFR capability flag
    arr[4] = &msg[10..13];      //Navigation uncertainty category for velocity
    arr[5] = &msg[13..35];      //Sub-type specific fields
    arr[6] = &msg[35..36];      //Source bit for vertical rate
    arr[7] = &msg[36..37];      //Sign bit for vertical rate
    arr[8] = &msg[37..46];      //Vertical rate
    arr[9] = &msg[46..48];      //Reserved
    arr[10] = &msg[48..49];     //Sign bit for GNSS and Baro altitudes difference
    arr[11] = &msg[49..];       //Difference between GNSS and Baro altitudes

    arr
}
pub fn speed(msg: &[bool;56]) -> (f32, String, f32, String) {

    let data = cut_in_sections(msg);
    let sub_type = bin2dec(data[1]);

    if &sub_type == &1 || &sub_type == &2 {

        let dew: u32 = bin2dec(data[5][0..1].try_into().expect("slice with incorrect length"));
        let vew: i32 = bin2dec(data[5][1..11].try_into().expect("slice with incorrect length")) as i32;
        let dns: u32 = bin2dec(data[5][11..12].try_into().expect("slice with incorrect length"));
        let vns: i32 = bin2dec(data[5][12..].try_into().expect("slice with incorrect length")) as i32;

        let vx = match dew {
            1 => -1 * (vew - 1),
            0 => vew - 1,
            _ => panic!("dew different from 0 or 1"),
        };

        let vy = match dns {
            1 => -1 * (vns - 1),
            0 => vns - 1,
            _ => panic!("dns different from 0 or 1"),
        };

        let mut v: f32 = ((vy.pow(2) + vx.pow(2)) as f32).sqrt();
        if &sub_type == &2 {v = 4.0 * v}

        let vr = vertical_speed(&data[6].try_into().expect("slice with incorrect length"), &data[7].try_into().expect("slice with incorrect length"), &data[8].try_into().expect("slice with incorrect length"));

        (v, vr.1, vr.0, String::from("GS"))

    } else if &sub_type == &3 || &sub_type == &4 {

        //let sh: u32 = bin2dec(data[5][0..1].try_into().expect("slice with incorrect length"));                      //Status bit for magnetic heading
        //let hdg: i32 = bin2dec(data[5][1..11].try_into().expect("slice with incorrect length")) as i32;             //magnetic heading 
        let as_type: u32 = bin2dec(data[5][11..12].try_into().expect("slice with incorrect length"));                 //air-speed type
        let air_speed: i32 = bin2dec(data[5][12..].try_into().expect("slice with incorrect length")) as i32;          //air-speed
        

        let mut v = (air_speed - 1) as f32;
        if &sub_type == &4 {v = 4.0 * v}

        let vr = vertical_speed(&data[6].try_into().expect("slice with incorrect length"), &data[7].try_into().expect("slice with incorrect length"), &data[8].try_into().expect("slice with incorrect length"));


        (v, vr.1, vr.0, String::from("TAS"))
    } else {
        panic!("wrong speed sub_type")
    }
}

pub fn vertical_speed(vbit: &[bool;1], vsign: &[bool;1], vrate: &[bool;9]) -> (f32, String) {

    let vs = match vsign[0] {
        false => 64. * (bin2dec(vrate)-1) as f32,
        true => -64. * (bin2dec(vrate) as f32  -1.),
    };

    let source = match vbit[0] {
        false => String::from("GNSS"),
        true => String::from("Barometric"),
    };

    (vs, source)
}