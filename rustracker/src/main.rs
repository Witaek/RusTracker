mod data_treatment;
mod object;

fn main() {
    test_position();
    test_identification();
}

fn test_position() {
    let even_data : u64 = 0b01011000110000111000001011010110100100001100100010101100;
    let odd_data : u64 =  0b01011000110000111000011001000011010111001100010000010010;
    let (a,b) = data_treatment::position::coor(&even_data, &odd_data);
    println!( "latitude : {}", a);
    println!( "longitude : {}", b);
    let c = data_treatment::position::altitude_barometric(&even_data);
    println!( "altitude : {}", c);
}

fn test_identification() {
    let bin: u64 = 0b00100000001011001100001101110001110000110010110011100000;   // binaire exemple

    let number = data_treatment::identification::callsign(&bin);
    println!("{}",number);
}