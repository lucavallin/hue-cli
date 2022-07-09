use clap::{Parser, Subcommand};

#[derive(Parser)]
#[clap(author, version, about, long_about = None)]
#[clap(propagate_version = true)]
struct Cli {
    #[clap(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    #[clap(about = "Authenticate with a Philips Hue Bridge")]
    Auth {
        #[clap(value_parser)]
        #[clap(help = "IP address of the Philips Hue Bridge to authenticate with")]
        bridge_ip: Option<String>,
    },
}

#[tokio::main]
async fn main() -> Result<(), reqwest::Error> {
    let cli = Cli::parse();

    match &cli.command {
        Commands::Auth { bridge_ip } => {
            println!("Requested auth for Bridge IP: {:?}", bridge_ip);
            let resp = reqwest::Client::new()
                .post(format!("http://{}/api", bridge_ip.as_ref().unwrap()))
                .send()
                .await?
                .json()
                .await?;

            println!("{:#?}", resp);
            Ok(())
        }
    }
}
