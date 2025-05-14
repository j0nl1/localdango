use {
    anyhow::{Error, Result},
    bot_types::{Args, ClientType},
    dango_faucet::serve,
    dango_mock_httpd::{BlockCreation, GenesisOption, Preset, TestOption},
    grug::{HexByteArray, Inner},
    std::str::FromStr,
};

const DEFAULT_CHAIN_ID: &str = "dev-6";
const DEFAULT_HTTP_PORT: &str = "8080";
const DEFAULT_FAUCET_PORT: &str = "8082";
const DEFAULT_SK: &str = "8a8b0ab692eb223f6a2927ad56e63c2ae22a8bc9a5bdfeb1d8127819ddcce177";
const DEFAULT_USERNAME: &str = "owner";

#[tokio::main]
async fn main() -> Result<()> {
    let config = Args::default();

    let http_port = config.find("--http-port").unwrap_or(DEFAULT_HTTP_PORT);

    let faucet_port = config.find("--faucet-port").unwrap_or(DEFAULT_FAUCET_PORT);

    let chain_id = config.find("--chain-id").unwrap_or(DEFAULT_CHAIN_ID);

    let sk = HexByteArray::from_str(config.find("--signing-key").unwrap_or(DEFAULT_SK))?;

    let username = config.find("--username").unwrap_or(DEFAULT_USERNAME);

    tokio::try_join!(
        async {
            dango_mock_httpd::run(
                http_port.parse().map_err(Error::msg)?,
                BlockCreation::OnBroadcast,
                None,
                TestOption {
                    chain_id: chain_id.to_string(),
                    ..Default::default()
                },
                GenesisOption::preset_test(),
                true,
                None,
            )
            .await
            .map_err(Error::msg)
        },
        async {
            serve(
                ClientType::Http(format!("http://localhost:{}", http_port)).try_into()?,
                username,
                sk.into_inner(),
                chain_id,
                &format!("0.0.0.0:{}", faucet_port),
            )
            .await
        }
    )
    .unwrap();
    Ok(())
}
