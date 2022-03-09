//test of module object

use crate::object::squitter::Squitter;

#[cfg(test)]

const MSG : [bool; 112] = [true, false, false, false, true, true, false, true, false, true, false,
            false, true, false, false, false, false, true, false, false, false, false,
            false, false, true, true, false, true, false, true, true, false, false, false,
            true, false, false, false, false, false, false, false, true, false, true, true,
            false, false, true, true, false, false, false, false, true, true, false, true,
            true, true, false, false, false, true, true, true, false, false, false, false,
            true, true, false, false, true, false, true, true, false, false, true, true, true,
            false, false, false, false, false, false, true, false, true, false, true, true,
            true, false, true, true, false, false, false, false, false, true, false, false,
            true, true, false, false, false];


mod tests_squitter {
    use super::*;

    #[test]
    fn squitter_work() {
        let s: Squitter = Squitter {msg: MSG};
        assert_eq!(true, s.crc_check());
    }
}