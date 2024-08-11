mod networks;

use std::env;
use std::fs;
use clap::Parser;
use networks::Network;
use reqwest::Client;
use serde::Deserialize;
use std::time::Duration;
use tokio::time::sleep;

/// Casa Simple Notification Service
#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
    #[clap(
        short,
        long,
        default_value = "ZW3ISEHZUHPO7OZGMKLKIIMKVICOUDRCERI454I3DB2BH52HGLSO67W754"
    )]
    account: String,

    #[clap(short, long, default_value = "algorand")]
    network: String,

    #[clap(short, long, default_value = "mainnet")]
    environment: String,
}

const CHECK_INTERVAL: Duration = Duration::from_secs(2);

#[derive(Deserialize, Debug)]
struct Transaction {
    id: Option<String>,
    #[serde(rename = "sender")]
    sender: Option<String>,
    #[serde(rename = "payment-transaction")]
    payment: Option<PaymentTransaction>,
}

#[derive(Deserialize, Debug)]
struct PaymentTransaction {
    #[serde(rename = "receiver")]
    receiver: Option<String>,
    #[serde(rename = "amount")]
    amount: Option<u64>,
}

#[derive(Deserialize, Debug)]
struct TransactionsResponse {
    transactions: Vec<Transaction>,
}

#[derive(Deserialize, Debug)]
struct HealthResponse {
    round: u64,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("Casa Simple Notification Service");

    let args = Args::parse();

    let config_path = "src/data/networks.json";
    let config_data: String = fs::read_to_string(config_path)?;
    let networks: Vec<Network> = serde_json::from_str(&config_data)?;

    let client = Client::new();
    let user_address = &args.account;

    // Get the current round
    let current_round =
        get_current_round(&client, &networks, &args.network, &args.environment).await?;

    let mut last_checked_round = current_round;

    loop {
        check_for_transactions(
            &client,
            &networks,
            user_address,
            &args.network,
            &args.environment,
            last_checked_round,
        )
        .await?;
        last_checked_round += 1;
        sleep(CHECK_INTERVAL).await;
    }
}

async fn get_current_round(
    client: &Client,
    networks: &[Network],
    network_name: &str,
    environment: &str,
) -> Result<u64, Box<dyn std::error::Error>> {
    for network in networks {
        if network.network == network_name && network.environment == environment {
            for endpoint in &network.endpoints {
                // Verify connection with health endpoint/get the current round
                let health_url = format!("{}/health", endpoint.indexer);
                let health_response = client.get(&health_url).send().await?.text().await?;
                println!("Health API Response: {}", health_response);

                let health: HealthResponse = serde_json::from_str(&health_response)?;
                return Ok(health.round);
            }
        }
    }
    Err("Failed to fetch current round".into())
}

async fn check_for_transactions(
    client: &Client,
    networks: &[Network],
    address: &str,
    network_name: &str,
    environment: &str,
    after_round: u64,
) -> Result<(), reqwest::Error> {
    for network in networks {
        if network.network == network_name && network.environment == environment {
            for endpoint in &network.endpoints {
                let url = format!(
                    "{}/v2/accounts/{}/transactions?min-round={}&max-round={}",
                    endpoint.indexer,
                    address,
                    after_round,
                    after_round + 1
                );

                println!("Endpoint: {}", url);

                let response = client.get(&url).send().await?.text().await?;

                println!("Response: {}", response);

                let transactions_response: Result<TransactionsResponse, _> =
                    serde_json::from_str(&response);
                match transactions_response {
                    Ok(parsed_response) => {
                        for transaction in parsed_response.transactions {
                            println!("Transaction found: {:?}", transaction);
                            if let Some(payment) = &transaction.payment {
                                if let Some(receiver) = &payment.receiver {
                                    if receiver == address {
                                        println!("Incoming transaction found: {:?}", transaction);
                                    }
                                }
                                if let Some(sender) = &transaction.sender {
                                    if sender == address {
                                        println!("Outgoing transaction found: {:?}", transaction);
                                    }
                                }
                            }
                        }
                    }
                    Err(e) => {
                        println!("Failed to parse response: {}", e);
                    }
                }
            }
        }
    }
    Ok(())
}
