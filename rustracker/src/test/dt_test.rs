//test of module data_treatment

use crate::data_treatment::position::coor;
use crate::data_treatment::position::altitude_barometric;
use crate::data_treatment::identification::callsign;

#[cfg(test)]

//-------------------------------------------------------------------------------------------
//test of sub_module position

const EVEN_BIN: [bool;56] = [false, true, false, true, true, false,
                            false, false, true, true, false, false, false, false, true, 
                            true, true, false, false, false, false, false, true, false, 
                            true, true, false, true, false, true, true, false, true, false, 
                            false, true, false, false, false, false, true, true, false, false, 
                            true, false, false, false, true, false, true, false, true, true, false, false];

const ODD_BIN: [bool;56] = [false, true, false, true, true, false,
                            false, false, true, true, false, false, false, false, true, true,
                            true, false, false, false, false, true, true, false, false, true,
                            false, false, false, false, true, true, false, true, false, true,
                            true, true, false, false, true, true, false, false, false, true,
                            false, false, false, false, false, true, false, false, true, false];



mod tests_pos {
    use super::*;

    #[test]
    fn coor_works() {
        println!("Longitude : {} || Latitude : {}", coor(&EVEN_BIN, &ODD_BIN).0 ,coor(&EVEN_BIN, &ODD_BIN).1 );
        assert_eq!(coor(&EVEN_BIN, &ODD_BIN), (52.257202,3.9193726));
    }

    #[test]
    fn altitude_barometric_works() {
        println!("Altitude : {}", altitude_barometric(&EVEN_BIN));
        assert_eq!(altitude_barometric(&EVEN_BIN), 38000);
    }
}

//-------------------------------------------------------------------------------------------
//test of sub_module identification

const CALLSIGN_BIN: [bool;56] = [false, false, true, false, false, false, false,
                                false, false, false, true, false, true, true, false, false,
                                true, true, false, false, false, false, true, true, false,
                                true, true, true, false, false, false, true, true, true, false,
                                false, false, false, true, true, false, false, true, false, true,
                                true, false, false, true, true, true, false, false, false, false, false];
mod tests_cs {

    use super::*;

    #[test]
    fn callsign_works() {
        println!("Identification : {}", callsign(&CALLSIGN_BIN));
        assert_eq!(callsign(&CALLSIGN_BIN), String::from("KLM1023 "));
    }

}