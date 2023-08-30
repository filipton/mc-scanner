// https://github.com/LhAlant/MinecraftSLP

use crate::utils::{bytes_used, insert_bytes, insert_string, pack_varint, read_varint};
use color_eyre::Result;
use tokio::io::{AsyncReadExt, AsyncWriteExt};

mod utils;

const PROTOCOL_VERSION: u32 = 760;

#[tokio::main]
async fn main() -> Result<()> {
    let ip = "127.0.0.1";
    let port = 25565;

    let mut stream = tokio::net::TcpStream::connect(format!("{}:{}", ip, port)).await?;
    stream.set_nodelay(true)?;
    send_request(ip, port, &mut stream).await?;
    let json = read_json(&mut stream).await?;

    println!("{}", json);

    Ok(())
}

async fn read_json(stream: &mut tokio::net::TcpStream) -> Result<String> {
    let _packet_length = read_varint(stream).await?;

    let mut buf = [0u8; 1];
    stream.read_exact(&mut buf).await?;
    let _packet_id = buf[0];

    let string_length = read_varint(stream).await?;
    let mut buf = vec![0u8; string_length as usize];
    stream.read_exact(&mut buf).await?;

    Ok(String::from_utf8(buf)?)
}

async fn send_request(ip: &str, port: u16, stream: &mut tokio::net::TcpStream) -> Result<()> {
    let data = construct_request(ip, port);
    let status_request_packet = [1u8, 0u8];

    stream.write_all(&data).await?;
    stream.write_all(&status_request_packet).await?;

    Ok(())
}

fn construct_request(ip: &str, port: u16) -> Vec<u8> {
    let proctol_version = pack_varint(PROTOCOL_VERSION);
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
    insert_bytes(port as u32, &mut data, &mut idx);
    insert_bytes(next_state, &mut data, &mut idx);

    return data;
}
