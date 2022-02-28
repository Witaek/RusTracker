mod data_treatment;

fn main() {
    test_identification()
}

fn test_identification() {
    let bin: u64 = 0b00100000001011001100001101110001110000110010110011100000;   // binaire exemple

    let number = data_treatment::identification::callsign(&bin);
    println!("{}",number);
}