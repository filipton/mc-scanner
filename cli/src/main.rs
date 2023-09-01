use clap::Parser;
use color_eyre::Result;
use mc_server_stats::get_mc_info;

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

    let mc_info = get_mc_info(&args.ip, args.port).await?;

    //println!("{:#?}", mc_info);
    println!("===== SERVER INFO =====");
    println!("{}\n", mc_info.description.get_colored());
    println!(
        "Players: {}/{}",
        mc_info.players.online, mc_info.players.max
    );
    println!("Version: {}", mc_info.version.name);

    if !mc_info.players.sample.is_empty() {
        println!("\n===== PLAYER LIST =====");
        for player in mc_info.players.sample {
            println!("{}", player.name);
        }
    }

    println!("=======================");

    Ok(())
}
