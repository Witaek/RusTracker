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
    pub fn get_df(&self) -> u32{            //get dowlink format
        return bin2dec(&self.msg[0..5])
    }

    pub fn get_ca(&self) -> u32{        //get capatibility
        return bin2dec(&self.msg[5..8]);
    }

    pub fn get_adress(&self) -> String{    //get icao adress
        return bin2hex(&self.msg[8..32]);

    }

    pub fn get_data(&self) -> &[bool;56]{      //get the data block
        return self.msg[32..88].try_into().expect("slice with incorrect length");
    }

    pub fn get_pi(&self) -> &[bool]{        //get crc bits
        return &self.msg[88..112];
    }

    pub fn get_tc(&self) -> u32{            //get type code
        return bin2dec(&self.msg[32..37]);
    }


}
