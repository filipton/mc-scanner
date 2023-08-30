use color_eyre::Result;
use tokio::io::{AsyncWriteExt, AsyncReadExt};

#[tokio::main]
async fn main() -> Result<()> {
    let ip = "127.0.0.1";
    let port = 25565;

    let proctol_version = pack_varint(760);
    let packet_id = 0u8;
    let next_state = pack_varint(1);

    let request_length = bytes_used(packet_id as u32)
        + bytes_used(proctol_version as u32)
        + bytes_used(ip.len() as u32)
        + ip.len() as u8
        + 2
        + 1;

    let server_address_len = pack_varint(ip.len() as u32);
    let total_request_length = request_length + bytes_used(request_length as u32);

    let mut data = vec![0u8; total_request_length as usize];
    let mut idx: usize = 0;

    insert_bytes(pack_varint(request_length as u32), &mut data, &mut idx);
    insert_bytes(packet_id as u32, &mut data, &mut idx);
    insert_bytes(proctol_version, &mut data, &mut idx);
    insert_bytes(server_address_len, &mut data, &mut idx);
    insert_string(ip, &mut data, &mut idx);
    insert_bytes(port, &mut data, &mut idx);
    insert_bytes(next_state, &mut data, &mut idx);

    println!("{:?}", data);

    let status_request_packet = [1u8, 0u8];

    let mut stream = tokio::net::TcpStream::connect(format!("{}:{}", ip, port)).await?;
    stream.set_nodelay(true)?;

    stream.write_all(&data).await?;
    stream.write_all(&status_request_packet).await?;

    let mut buffer = [0u8; 1024];
    loop {
        let bytes_read = stream.read(&mut buffer).await?;

        if bytes_read == 0 {
            break;
        }

        println!("READ: {:?}", &buffer[..bytes_read]);
    }

    Ok(())
}

fn pack_varint(mut value: u32) -> u32 {
    let mut varint = 0u32;

    loop {
        varint <<= 8;

        let tmp = value & 0x7F;
        value >>= 7;

        varint += tmp as u32;
        if value != 0 {
            varint |= 0x80;
        } else {
            break;
        }
    }

    return varint;
}

fn bytes_used(mut value: u32) -> u8 {
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

fn insert_bytes(data_byte: u32, data: &mut Vec<u8>, idx: &mut usize) {
    let bytes_used = bytes_used(data_byte);

    for i in (0..bytes_used).rev() {
        data[*idx] = ((data_byte >> (i * 8)) & 0xFF) as u8;
        *idx += 1;
    }
}

fn insert_string(string: &str, data: &mut Vec<u8>, idx: &mut usize) {
    let len = pack_varint(string.len() as u32);
    let string = string.as_bytes();

    for i in 0..len {
        data[*idx] = string[i as usize];
        *idx += 1;
    }
}
