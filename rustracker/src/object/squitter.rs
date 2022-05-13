#![allow(dead_code)]

use crate::ressources::binary_fun::bin2dec;
use crate::ressources::binary_fun::bin2hex;


pub struct Squitter {
    pub msg: [bool; 112],
}

impl Default for Squitter {
    fn default () -> Squitter {
        Squitter{msg : [false; 112],}
    }
}

impl Squitter {

    pub fn from_msg(msg : Vec<u8>) -> Self {
        let mut s = Squitter::default();
        let mut msg_string = String::from("");
        for i  in 0..112/8 {
            let bytes = format!("{:08b}", msg[i]);
            msg_string.push_str(&bytes)
        }
        for i in 0..112 {
            if msg_string.get(i..i+1).unwrap() == "1" {
                s.msg[i]=true;
            }
        }
        return s;
    }


    pub fn get_df(&self) -> Result<u32,String> {            //get dowlink format
        match bin2dec(&self.msg[0..5]) {
            Ok(a) => return Ok(a as u32),
            Err(a) => Err(a)
        }
    }

    pub fn get_ca(&self) -> Result<u32,String> {        //get capatibility
        match bin2dec(&self.msg[5..8]) {
            Ok(a) => return Ok(a as u32),
            Err(a) => Err(a)
        }
    }

    pub fn get_adress(&self) -> Result<String,String> {    //get icao adress

        match bin2hex(&self.msg[8..32]) {
            Ok(a) => return Ok(a),
            Err(a) => Err(a)
        }

    }

    pub fn get_data(&self) -> &[bool;56]{      //get the data block
        return self.msg[32..88].try_into().expect("slice with incorrect length");
    }

    pub fn get_pi(&self) -> &[bool]{        //get crc bits
        return &self.msg[88..112];
    }

    pub fn get_tc(&self) -> Result<u32,String>{            //get type code

        match bin2dec(&self.msg[32..37]) {
            Ok(a) => return Ok(a as u32),
            Err(a) => Err(a)
        }
    }

    pub fn crc_check(&self) -> bool {
        let poly = [true, true, true, true, true, true, true,
                    true, true, true, true, true, true, false, 
                    true, false, false, false, false, false,
                    false, true, false, false, true];
        let mut msgc = self.msg.clone();
        let mut i = 0;
        while i < (112-24) {
            let bit0 = msgc[i];
            if bit0==false{i+=1;}
            else {
                for k in 0..25 {
                    msgc[i+k] = msgc[i+k] ^ poly[k]; //operation XOR
                }
            }
        }
        return [false;112]== msgc;
    }
}
