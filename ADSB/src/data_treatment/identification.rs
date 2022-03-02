use std::str;

const ALPHABET: &str= "#ABCDEFGHIJKLMNOPQRSTUVWXYZ##### ###############0123456789######";

//Cuts down the 56 bits message into sections 
fn decoupage(&bin: &u64) -> [String;10] {               // In : reference to a u64 number, Out : array containing 10 strings
    let bin_str: String = format!("{bin:056b}");        // Converts binary number to string

    let mut arr: [String;10] = Default::default();      
    arr[0] = bin_str[..5].to_owned();                   // first 5 bits are TC
    arr[1] = bin_str[5..8].to_owned();                  // 3 next bits are CA
    let mut index = 2;
    for i in [8, 14, 20, 26, 32, 38, 44, 50] {          // cuts the remaining bits into sections of 6
        arr[index] = bin_str[i..i+6].to_owned();
        
        index += 1;
    }

    arr
}

//Translates the 6bits sections with the alphabet
pub fn callsign(&bin: &u64) -> String {
    let arr = decoupage(&bin);                                                          
    let mut flight_number: String = String::from("");                                   
    for elem in &arr[2..] {                                                             // translates each section (first two aren't used)
        let index: usize = usize::from_str_radix(elem, 2).expect("Not binary");         // convert 6 bits section into decimal
        let letter = &ALPHABET[index..index+1];                                         // looks for corresponding letter in the alphabet
        flight_number.push_str(letter)                                                  
    };
    flight_number                                                                       
}