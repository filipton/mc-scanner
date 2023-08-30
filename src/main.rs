// https://github.com/LhAlant/MinecraftSLP

use clap::Parser;
use color_eyre::Result;

mod info_getter;
mod utils;

#[derive(Debug, Parser)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[arg(index = 1)]
    ip: String,

    #[arg(index = 2)]
    port: u16,
}

#[tokio::main]
async fn main() -> Result<()> {
    color_eyre::install()?;
    let args = Args::parse();

    println!("{:?}", info_getter::get_mc_info(&args.ip, args.port).await?);

    Ok(())
}
