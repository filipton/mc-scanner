// https://github.com/LhAlant/MinecraftSLP

use color_eyre::Result;

mod info_getter;
mod utils;

#[tokio::main]
async fn main() -> Result<()> {
    let ip = "topp.jartex.fun";
    let port = 25565;

    println!("{:?}", info_getter::get_mc_info(ip, port).await?);

    Ok(())
}
