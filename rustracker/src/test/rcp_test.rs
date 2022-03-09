//test of module reception

use crate::reception::sampling::amp;

#[cfg(test)]

mod tests_sampling {
    use super::*;

    #[test]
    fn amp_work() {
        let bytes = [15,8];
        println!("{:?}",amp(&bytes));
        assert_eq!(vec![17_f64],amp(&bytes));
    }
}

