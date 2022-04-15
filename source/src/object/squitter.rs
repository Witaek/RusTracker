#![allow(dead_code)]

use crate::ressources::binary_fun::bin2dec;


pub struct Squitter {
    pub msg: [bool; 112],
}

impl Default for Squitter {
    fn default () -> Squitter {
        Squitter{msg : [false; 112],}
    }
}

impl Squitter {
    pub fn get_df(&self) -> u32{            //get dowlink format
        return bin2dec(&self.msg[0..5])
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

    pub fn convert(&self) -> [u8;14] {
        let mut res = [0;14]; 
        for i in 0..(112/8) {
            res[i]=bin2dec(&self.msg[i*8..i*8+8]) as u8
        }
        return res;
    }
}
