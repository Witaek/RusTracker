pub fn bin2dec(binary: &[bool]) -> u64 {     //binary to decimal conversion using binary as an array of bits (as boolean)
    let n: u32 = binary.len().try_into().unwrap();
    let mut res: u64 = 0;
    let mut i: u32 = 0;
    for &bit in binary {
        if bit {
            res += 2_u64.pow(n-1-i);
        }
        i+=1;
    }
    return res;
}

pub fn bin2hex(binary: &[bool]) -> String {     //binary to decimal conversion using binary as an array of bits (as boolean)
    return format!("{:x}",bin2dec(&binary))
}