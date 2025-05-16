use {
    bot_types::ClientType,
    clap::{command, Parser},
    dango_faucet::serve,
    dango_genesis::GrugOption,
    dango_mock_httpd::{BlockCreation, GenesisOption, Preset, TestOption},
    dango_testing::constants::owner,
    localdango::{Config, User, DEFAULT_CHAIN_ID, DEFAULT_FAUCET_PORT, DEFAULT_HTTP_PORT},
};

#[derive(Parser)]
#[command(author, version, about, next_display_order = None)]
struct Cli {
    #[command(subcommand)]
    command: Command,
}

#[derive(Parser)]
enum Command {
    /// Run the chain
    Run {
        /// The port to run the HTTP server on
        #[arg(long, default_value_t = DEFAULT_HTTP_PORT)]
        http_port: u16,

        /// The port to run the faucet on
        #[arg(long, default_value_t = DEFAULT_FAUCET_PORT)]
        faucet_port: u16,

        /// The chain ID to use
        #[arg(long, default_value_t = DEFAULT_CHAIN_ID.to_string())]
        chain_id: String,
    },
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let cli = Cli::parse();

    let config: Config = match cli.command {
        Command::Run {
            http_port,
            faucet_port,
            chain_id,
        } => {
            let owner: User = User {
                username: owner::USERNAME.clone(),
                public_key: owner::PUBLIC_KEY,
                private_key: owner::PRIVATE_KEY,
            };

            let genesis = GenesisOption {
                grug: GrugOption {
                    owner_username: owner::USERNAME.clone(),
                    ..GrugOption::preset_test()
                },
                ..GenesisOption::preset_test()
            };

            Config {
                genesis,
                http_port,
                faucet_port,
                chain_id,
                owner,
            }
        }
    };

    tokio::try_join!(
        async {
            dango_mock_httpd::run(
                config.http_port,
                BlockCreation::OnBroadcast,
                None,
                TestOption {
                    chain_id: config.chain_id.to_string(),
                    ..Default::default()
                },
                config.genesis,
                true,
                None,
            )
            .await
            .map_err(anyhow::Error::msg)
        },
        async {
            serve(
                ClientType::Http(format!("http://localhost:{}", config.http_port)).try_into()?,
                config.owner.username.as_ref(),
                config.owner.private_key,
                &config.chain_id,
                &format!("0.0.0.0:{}", config.faucet_port),
            )
            .await
        }
    )
    .unwrap();
    Ok(())
}
