#![allow(dead_code)]

use super::ressources::binary_fun::bin2dec as bin2dec;
use super::ressources::binary_fun::bin2hex as bin2hex;

pub struct Squitter {
    msg: [bool; 112],
}

impl Squitter {
    pub fn get_df(&self) -> u32{            //get dowlink format
        return bin2dec(&self.msg[0..5])
    }

    pub fn get_ca(&self) -> &[bool]{        //get capatibility
        return &self.msg[5..8];
    }

    pub fn get_adress(&self) -> String{    //get icao adress
        return bin2hex(&self.msg[8..32]);

    }

    pub fn get_data(&self) -> &[bool]{      //get the data block
        return &self.msg[32..88];
    }

    pub fn get_pi(&self) -> &[bool]{        //get crc bits
        return &self.msg[88..112];
    }

    pub fn get_tc(&self) -> u32{            //get type code
        return bin2dec(&self.msg[32..37]);
    }
}

