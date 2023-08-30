use crate::{
    extra_colored::get_colored_bit,
    utils::{bytes_used, insert_bytes, insert_string, pack_varint, read_varint},
};
use color_eyre::Result;
use colored::Colorize;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use tokio::io::{AsyncReadExt, AsyncWriteExt};

const PROTOCOL_VERSION: u32 = 760;

pub async fn get_mc_info(ip: &str, port: u16) -> Result<ServerInfo> {
    let mut stream = tokio::net::TcpStream::connect(format!("{}:{}", ip, port)).await?;
    stream.set_nodelay(true)?;
    send_request(ip, port, &mut stream).await?;
    let json = read_string(&mut stream).await?;

    stream.shutdown().await?;

    let server_info: ServerInfo = serde_json::from_str(&json)?;
    Ok(server_info)
}

async fn read_string(stream: &mut tokio::net::TcpStream) -> Result<String> {
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

    let request_length = bytes_used(packet_id as u64)
        + bytes_used(proctol_version as u64)
        + bytes_used(ip.len() as u64)
        + ip.len() as u8
        + 2
        + 1;

    let server_address_len = pack_varint(ip.len() as u32);
    let total_request_length = request_length + bytes_used(request_length as u64);

    let mut data = vec![0u8; total_request_length as usize];
    let mut idx: usize = 0;

    insert_bytes(pack_varint(request_length as u32), &mut data, &mut idx);
    insert_bytes(packet_id as u64, &mut data, &mut idx);
    insert_bytes(proctol_version, &mut data, &mut idx);
    insert_bytes(server_address_len, &mut data, &mut idx);
    insert_string(ip, &mut data, &mut idx);
    insert_bytes(port as u64, &mut data, &mut idx);
    insert_bytes(next_state, &mut data, &mut idx);

    return data;
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ServerInfo {
    pub version: Version,
    pub players: Players,
    pub description: Description,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub favicon: Option<String>,

    #[serde(default)]
    pub enforces_secure_chat: bool,

    #[serde(default)]
    pub previews_chat: bool,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub modinfo: Option<ModInfo>,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Version {
    pub name: String,
    pub protocol: u32,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Players {
    pub max: u32,
    pub online: u32,

    #[serde(default)]
    pub sample: Vec<Sample>,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Sample {
    pub name: String,
    pub id: String,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase", untagged)]
pub enum Description {
    Old(String),
    New(DescriptionNew),
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct DescriptionNew {
    #[serde(default)]
    pub extra: Vec<Extra>,

    pub text: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Extra {
    pub bold: Option<bool>,

    pub color: Option<String>,
    pub text: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ModInfo {
    #[serde(rename = "type")]
    pub type_field: String,
    pub mod_list: Vec<Value>,
}

impl Description {
    pub fn get_colored(&self) -> String {
        if let Description::Old(text) = self {
            return text.clone();
        }

        if let Description::New(new) = self {
            if new.extra.is_empty() {
                return new.text.clone();
            }

            let mut tmp = String::new();
            for extra in &new.extra {
                let mut colored_text = get_colored_bit(
                    &extra.clone().color.unwrap_or(String::from("white")),
                    extra.bold.unwrap_or(false),
                    &extra.text,
                );

                if extra.bold.unwrap_or(false) {
                    colored_text = colored_text.bold();
                }

                tmp += format!("{}", colored_text).as_str();
            }
            return tmp;
        }

        return String::new();
    }
}
