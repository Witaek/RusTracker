//test of module object
#![allow(dead_code)]

use crate::object::squitter::Squitter;

#[cfg(test)]

const MSG1 : [bool; 112] = [true, false, false, false, true, true, false, true, false, true, false,
            false, true, false, false, false, false, true, false, false, false, false,
            false, false, true, true, false, true, false, true, true, false, false, false,
            true, false, false, false, false, false, false, false, true, false, true, true,
            false, false, true, true, false, false, false, false, true, true, false, true,
            true, true, false, false, false, true, true, true, false, false, false, false,
            true, true, false, false, true, false, true, true, false, false, true, true, true,
            false, false, false, false, false, false, true, false, true, false, true, true,
            true, false, true, true, false, false, false, false, false, true, false, false,
            true, true, false, false, false];

const MSG2 : [bool;112] = [true, false, false, false, true, true, false, true, false, true,
                        false, false, false, false, false, false, false, true, true, false,
                        false, false, true, false, false, false, false, true, true, true, false,
                        true, false, true, false, true, true, false, false, false, true, true,
                        false, false, false, false, true, true, true, false, false, false, false,
                        true, true, false, false, true, false, false, false, false, true, true,
                        false, true, false, true, true, true, false, false, true, true, false, false,
                        false, true, false, false, false, false, false, true, false, false, true,
                        false, false, true, true, false, true, false, false, true, false, false, true,
                        false, true, false, true, false, true, true, false, true, false, true, true, false];

const MSG3 : [bool;112] = [true, false, false, false, true, true, false, true, false, true, false, false,
                            false, false, false, false, false, true, true, false, false, false, true, false,
                            false, false, false, true, true, true, false, true, false, true, false, true,
                            true, false, false, false, true, true, false, false, false, false, true, true,
                            true, false, false, false, false, false, true, false, true, true, false, true,
                            false, true, true, false, true, false, false, true, false, false, false, false,
                            true, true, false, false, true, false, false, false, true, false, true, false,
                            true, true, false, false, false, false, true, false, true, false, false, false,
                            false, true, true, false, false, false, true, true, true, false, true, false,
                            false, true, true, true];


mod tests_squitter {
    use super::*;

    #[test]
    fn squitter1_work() {
        let s1: Squitter = Squitter {msg: MSG1};
        assert_eq!(true, s1.crc_check());
    }

    #[test]
    fn squitter2_work() {
        let s2: Squitter = Squitter {msg: MSG2};
        assert_eq!(true, s2.crc_check());
    }

    #[test]
    fn squitter3_work() {
        let s3: Squitter = Squitter {msg: MSG3};
        assert_eq!(true, s3.crc_check());
    }
}