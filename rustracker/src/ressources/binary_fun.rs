pub fn bin2dec(binary: &[bool]) -> Result<u64,String> {     //binary to decimal conversion using binary as an array of bits (as boolean)
    let n: u32 = binary.len().try_into().unwrap();
    let mut res: u64 = 0;
    let mut i: u32 = 0;
    for &bit in binary {
        if bit {
            match 2_u64.checked_pow(n-1-i) {
                Some (b) =>
                    match res.checked_add(b) {
                        None => return Err(String::from("overflow at bin2dec")),
                        Some(a) => res = a
                    },
                None => return Err(String::from("overflow at bin2dec"))
            }
        }
        i+=1;
    }
    return Ok(res);
}

pub fn bin2hex(binary: &[bool]) -> Result<String,String> {     //binary to decimal conversion using binary as an array of bits (as boolean)
    let inter = bin2dec(&binary);
    match inter {
        Ok(a) =>return Ok(format!("{:x}",a)),
        Err(a) => return Err(a)
    }
    
}
