#![allow(dead_code)]
use crate::reception::sampling::sample2binary;

#[cfg(test)]
const REF: [bool; 112] = [true, true, true, true, false, false,
false, true, false, false, true, true, true, true,
true, false, true, true, false, false, false, false,
false, false, true, true, false, false, true, false,
false, true, false, false, true, true, true, false,
false, true, false, true, false, true, false, false,
true, true, false, true, false, true, false, false,
false, true, true, true, true, true, false, true, true,
true, true, true, true, false, false, false, true, false,
false, false, true, false, false, true, true, true, false,
false, true, true, true, false, true, true, true, true, true,
true, true, false, true, false, true, false, true, false, false,
true, false, false, false, true, true, true, true, true, true, false];

const SAMPLE: [f64; 224] = [0.65, 0.44, 0.29, 0.11, 0.91, 0.06, 0.49,
0.08, 0.25, 0.49, 0.4, 0.78, 0.06, 0.55, 0.72, 0.46, 0.04, 0.8, 0.47,
0.61, 0.81, 0.44, 0.41, 0.07, 0.97, 0.28, 0.47, 0.14, 0.94, 0.37, 0.02,
0.36, 0.69, 0.42, 0.66, 0.52, 0.15, 0.64, 0.4, 0.74, 0.21, 0.85, 0.2,
0.81, 0.52, 0.78, 0.17, 0.52, 0.26, 0.06, 0.27, 0.26, 0.51, 0.68, 0.92,
0.97, 0.7, 0.16, 0.71, 0.94, 0.31, 0.63, 0.87, 0.12, 0.37, 0.6, 0.29,
0.79, 0.61, 0.56, 1.0, 0.31, 0.2, 0.13, 0.08, 0.83, 0.11, 0.46, 0.79,
0.17, 0.14, 0.99, 0.87, 0.51, 0.08, 0.89, 0.62, 0.46, 0.16, 0.94, 0.04,
0.92, 0.85, 0.66, 0.79, 0.78, 0.48, 0.6, 0.86, 0.42, 0.13, 0.85, 0.92,
0.25, 0.41, 0.6, 0.32, 0.84, 0.05, 0.22, 0.17, 0.06, 0.99, 0.92, 0.65,
0.02, 0.82, 0.32, 0.48, 0.12, 0.33, 0.76, 0.67, 0.57, 0.18, 0.0, 0.81,
0.43, 0.88, 0.35, 0.99, 0.07, 0.66, 0.38, 0.34, 0.86, 0.52, 0.7, 0.22,
0.49, 0.87, 0.54, 0.62, 0.95, 0.52, 0.98, 0.32, 0.83, 0.67, 0.08, 0.33,
0.95, 0.2, 0.37, 0.32, 0.21, 0.39, 0.29, 0.86, 0.28, 0.03, 0.21, 0.06,
0.19, 0.29, 0.25, 0.56, 0.01, 0.75, 0.08, 0.34, 0.9, 0.87, 0.24, 0.59,
0.58, 0.66, 0.37, 0.93, 0.86, 0.76, 0.68, 0.19, 0.06, 0.75, 0.17, 0.81,
1.0, 0.3, 0.13, 0.12, 0.47, 0.46, 0.31, 0.42, 0.82, 0.62, 0.07, 0.25,
0.38, 0.09, 0.77, 0.93, 0.02, 0.36, 0.43, 0.19, 0.31, 0.05, 0.83, 0.72,
0.66, 0.15, 0.07, 0.73, 0.54, 0.61, 0.58, 0.76, 0.58, 0.72, 0.39, 0.19, 0.25];

mod tests_squitter {
    use super::*;

    #[test]
    fn sample2binary_work() {
        let s1 = &sample2binary(vec![SAMPLE])[0];
        assert_eq!(REF, s1.msg);
    }
}

