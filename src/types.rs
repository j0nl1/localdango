use {dango_genesis::GenesisOption, dango_types::account_factory::Username};

pub struct User {
    pub username: Username,
    pub public_key: [u8; 33],
    pub private_key: [u8; 32],
}

pub struct Config {
    pub http_port: u16,
    pub faucet_port: u16,
    pub chain_id: String,
    pub genesis: GenesisOption,
    pub owner: User,
}
