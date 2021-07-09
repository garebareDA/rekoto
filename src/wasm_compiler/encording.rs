use byteorder::{ByteOrder, LittleEndian};

pub(crate) fn ieee754(n: u32) -> [u8; 4] {
    let mut buf: [u8; 4] = [0; 4];
    LittleEndian::write_u32(&mut buf, n);
    return buf;
}

pub(crate) fn encode_string(str: &str) -> Vec<u16> {
    let v: Vec<u16> = str.encode_utf16().collect();
    return v;
}

pub(crate) fn signedLEB128(mut n: u8) -> Vec<u8> {
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

pub(crate) fn unsignedLEB128(mut n: u8) -> Vec<u8> {
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