pub fn bin2dec(binary: &[bool]) -> u32 {     //binary to decimal conversion using binary as an array of bits (as boolean)
    let n: u32 = binary.len().try_into().unwrap();
    let mut res: u32 = 0;
    let mut i: u32 = 0;
    for &bit in binary {
        if bit {
            res += 2_u32.pow(n-1-i);
        }
        i+=1;
    }
    return res;
}