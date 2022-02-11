use std::str;

const ALPHABET: &str= "#ABCDEFGHIJKLMNOPQRSTUVWXYZ#####_###############0123456789######";


fn main() {
    let bin: u64 = 0b00100000001011001100001101110001110000110010110011100000;

    let number = callsign(&bin);
    println!("{}",number);

}

fn callsign(&bin: &u64) -> String {
    let bin_str: String = format!("{bin:056b}");

    let mut arr: [&str;10] = ["";10];
    arr[0] = &bin_str[..5];
    arr[1] = &bin_str[5..8];

    let mut index = 2;
    for i in [8, 14, 20, 26, 32, 38, 44, 50] {
        arr[index] = &bin_str[i..i+6];
        
        index += 1;
    }

    get_flight_number(arr)
}

fn get_flight_number(arr: [&str;10]) -> String {

    let mut flight_number: String = String::from("");
    for elem in &arr[2..] {
        let index: usize = usize::from_str_radix(elem, 2).expect("Not binary");
        //let letter: u8 = bytes_alphabet[index];
        let letter = &ALPHABET[index..index+1];
        flight_number.push_str(letter)
    };
    flight_number
}
