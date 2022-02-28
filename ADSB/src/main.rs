mod data_treatment;

fn main() {
    test_position();
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