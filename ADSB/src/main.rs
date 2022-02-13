use std::str;

const ALPHABET: &str= "#ABCDEFGHIJKLMNOPQRSTUVWXYZ#####_###############0123456789######";


fn main() {
    let bin: u64 = 0b00100000001011001100001101110001110000110010110011100000;

    let number = callsign(&bin);
    println!("{}",number);

}

fn decoupage(&bin: &u64) -> [String;10] {
    let bin_str: String = format!("{bin:056b}");

    let mut arr: [String;10] = Default::default();
    arr[0] = bin_str[..5].to_owned();
    arr[1] = bin_str[5..8].to_owned();

    let mut index = 2;
    for i in [8, 14, 20, 26, 32, 38, 44, 50] {
        arr[index] = bin_str[i..i+6].to_owned();
        
        index += 1;
    }

    arr
}

fn callsign(&bin: &u64) -> String {
    let arr = decoupage(&bin);
    let mut flight_number: String = String::from("");
    for elem in &arr[2..] {
        let index: usize = usize::from_str_radix(elem, 2).expect("Not binary");
        let letter = &ALPHABET[index..index+1];
        flight_number.push_str(letter)
    };
    flight_number
}
