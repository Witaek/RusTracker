#![allow(dead_code)]
use crate::ressources::binary_fun::bin2dec;

const ALPHABET : [char; 64]= ['#', 'A', 'B', 'C', 'D', 'E', 'F', 'G',
'H', 'I', 'J', 'K', 'L', 'M', 'N', 'O', 'P', 'Q', 'R', 'S',
'T', 'U', 'V', 'W', 'X', 'Y', 'Z', '#', '#', '#', '#', '#',
' ', '#', '#', '#', '#', '#', '#', '#', '#', '#', '#', '#',
'#', '#', '#', '#', '0', '1', '2', '3', '4', '5', '6', '7',
'8', '9', '#', '#', '#', '#', '#', '#'];

//Cuts down the 56 bits message into sections 
fn decoupage(data: &[bool; 56]) -> [&[bool];10] {               // In : reference to a u64 number, Out : array containing 10 strings

    let mut arr: [&[bool];10] = Default::default();      
    arr[0] = &data[..5];                                        // first 5 bits are TC
    arr[1] = &data[5..8];                                       // 3 next bits are CA
    let mut index = 2;
    for i in [8, 14, 20, 26, 32, 38, 44, 50] {                  // cuts the remaining bits into sections of 6
        arr[index] = &data[i..i+6];
        
        index += 1;
    }

    arr
}

//Translates the 6bits sections with the alphabet
pub fn callsign(&data: &[bool; 56]) -> String {
    let arr = decoupage(&data);                                                          
    let mut flight_number: String = String::from("");                                   
    for elem in &arr[2..] {                                                             // translates each section (first two aren't used)
        let index = bin2dec(elem) as usize;                                             // convert 6 bits section into decimal
        let letter = &ALPHABET[index];                                                  // looks for corresponding letter in the alphabet
        flight_number.push(*letter)                                                  
    };
    flight_number                                                                       
}