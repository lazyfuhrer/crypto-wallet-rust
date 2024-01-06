use clap::{Command, Arg};
use anyhow::Result;
use std::env;
use std::str::FromStr;
use web3::types::Address;
use secp256k1::SecretKey;

mod utils;
mod eth_wallet;

#[tokio::main]
async fn main() -> Result<()> {
    dotenv::dotenv().ok();
    let endpoint = env::var("QUICKNODE_HOLESKY_WS")?;
    let web3_con = eth_wallet::establish_web3_connection(&endpoint).await?;

    let matches = Command::new("cli-tool")
        .version("0.1.0")
        .author("Biswarghya Biswas")
        .about("A simple CLI tool (Currently supports Holesky testnet)")
        .subcommand(
            Command::new("create")
                .about("Creates a wallet pair")
        )
        .subcommand(
            Command::new("send")
                .about("Sends ether to an address")
                .arg(Arg::new("to-addr")
                    .short('t')
                    .long("to-addr")
                    .value_name("TO-ADDRESS")
                    .help("The address to send to")
                    .required(true)
                    .value_parser(clap::value_parser!(String))) 
                .arg(Arg::new("value")
                    .short('v')
                    .long("value")
                    .value_name("VALUE")
                    .help("The value to send")
                    .required(true)
                    .value_parser(clap::value_parser!(String))) 
                .arg(Arg::new("secret-key")
                    .short('s')
                    .long("secret-key")
                    .value_name("KEY")
                    .help("The secret key to use")
                    .required(true)
                    .value_parser(clap::value_parser!(String))) 
        )
        .subcommand(
            Command::new("balance")
                .about("Checks balance of an address")
                .arg(Arg::new("addr")
                    .short('a')
                    .long("address")
                    .value_name("ADDRESS")
                    .help("The address to check balance")
                    .required(true)
                    .value_parser(clap::value_parser!(String))) 
        )
        .subcommand(
            Command::new("block")
                .about("Returns current block number")
        )
        .get_matches();

    if let Some(_matches) = matches.subcommand_matches("create") {
        let (secret_key, pub_key) = eth_wallet::generate_keypair();
        let pub_address = eth_wallet::public_key_address(&pub_key);

        println!("secret key: {}", &secret_key.to_string());
        println!("public key: {}", &pub_key.to_string());
        println!("public address: {:?}", pub_address);
    }

    if let Some(matches) = matches.subcommand_matches("send") {

        let to_addr = matches.get_one::<String>("to-addr").unwrap(); 
        let value = matches.get_one::<String>("value").unwrap();
        let secret_key = matches.get_one::<String>("secret-key").unwrap();

        let transaction = eth_wallet::create_eth_transaction(
            Address::from_str(to_addr)?,
            value.parse::<f64>()?,
        );
        let transact_hash = eth_wallet::sign_and_send(&web3_con, transaction, &SecretKey::from_str(secret_key)?).await?;
        println!("Transaction Hash: {:?}", transact_hash);
    }

    if let Some(matches) = matches.subcommand_matches("balance") {
        let address = matches.get_one::<String>("addr").unwrap();

        let balance = eth_wallet::get_balance_in_eth_static(&address, &web3_con).await?;
        println!("wallet balance: {}", &balance);
    }

    if let Some(_matches) = matches.subcommand_matches("block") {
        let block_number = web3_con.eth().block_number().await?;
        println!("block number: {}", &block_number);
    }

    Ok(())
}