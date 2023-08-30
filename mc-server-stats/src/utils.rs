use color_eyre::Result;
use tokio::{io::AsyncReadExt, net::TcpStream};

pub async fn read_varint(stream: &mut TcpStream) -> Result<u64> {
    let mut unpacked_varint = 0u64;
    let mut tmp = [0x80; 1];
    let mut i = 0u8;

    while (tmp[0] & 0x80) != 0 {
        stream.read_exact(&mut tmp).await?;

        let lhs: u64 = (tmp[0] & 0x7F).into();
        let rhs: u64 = (7 * i).into();
        let val = lhs << rhs;

        unpacked_varint |= val;
        i += 1;
    }

    return Ok(unpacked_varint);
}

pub fn pack_varint(mut value: u32) -> u64 {
    let mut varint = 0u64;

    loop {
        varint <<= 8;

        let tmp = value & 0x7F;
        value >>= 7;

        varint += tmp as u64;
        if value != 0 {
            varint |= 0x80;
        } else {
            break;
        }
    }

    return varint;
}

pub fn bytes_used(mut value: u64) -> u8 {
    if value == 0 {
        return 1;
    }

    for i in (1..=4).rev() {
        if value & 0xFF000000 != 0 {
            return i;
        } else {
            value <<= 8;
        }
    }

    return 0;
}

pub fn insert_bytes(data_byte: u64, data: &mut Vec<u8>, idx: &mut usize) {
    let bytes_used = bytes_used(data_byte);

    for i in (0..bytes_used).rev() {
        data[*idx] = ((data_byte >> (i * 8)) & 0xFF) as u8;
        *idx += 1;
    }
}

pub fn insert_string(string: &str, data: &mut Vec<u8>, idx: &mut usize) {
    let len = pack_varint(string.len() as u32);
    let string = string.as_bytes();

    for i in 0..len {
        data[*idx] = string[i as usize];
        *idx += 1;
    }
}
