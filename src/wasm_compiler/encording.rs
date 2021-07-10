use byteorder::{ByteOrder, LittleEndian};

pub fn ieee_754(n: u32) -> Vec<u8> {
    let mut buf: [u8; 4] = [0; 4];
    LittleEndian::write_u32(&mut buf, n);
    return buf.to_vec();
}

pub fn encode_string(strs: &str) -> Vec<u8> {
    let mut vec:Vec<u8> = Vec::new();
    vec.push(strs.len() as u8);
    vec.append(&mut strs.as_bytes().to_vec());
    return vec;
}

pub fn signed_leb_128(mut n: u8) -> Vec<u8> {
    let mut buf: Vec<u8> = Vec::new();
    let mut more = true;
    loop {
        if !more {
            break;
        }

        let mut byte = n & 0x7f;
        n = n >> 7;
        if (n == 0 && (byte & 0x40) == 0) || (n == 1 && (byte & 0x40) != 0) {
            more = false;
        } else {
            byte = byte | 0x80;
        }
        buf.push(byte);
    }

    return buf;
}

pub fn unsigned_leb_128(mut n: u8) -> Vec<u8> {
    let mut buf: Vec<u8> = Vec::new();
    loop {
        let mut byte = n & 0x7f;
        n = n >> 7;
        if n != 0 {
            byte = byte | 0x80;
        }
        buf.push(byte);

        if n == 0 {
            break;
        }
    }

    return buf;
}