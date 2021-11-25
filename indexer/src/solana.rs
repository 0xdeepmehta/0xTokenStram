use std::{str::FromStr, thread};

use solana_client::{pubsub_client, rpc_client::RpcClient};
use solana_sdk::{account::Account, pubkey::Pubkey};

use crate::{establish_connection, models::Stream};

fn get_all_program_accounts() -> Vec<(Pubkey, Account)> {
    let program_pub_key = Pubkey::from_str("DcGPfiGbubEKh1EnQ86EdMvitjhrUo8fGSgvqtFG4A9t")
        .expect("program address invalid");
    let url = "https://api.devnet.solana.com".to_string();
    let client = RpcClient::new(url);

    client
        .get_program_accounts(&program_pub_key)
        .expect("Something went wrong")
}

pub fn subscribe_to_program() {
    let url = "ws://api.devnet.solana.com".to_string();
    let program_pub_key = Pubkey::from_str("DcGPfiGbubEKh1EnQ86EdMvitjhrUo8fGSgvqtFG4A9t")
        .expect("program address invalid");

    thread::spawn(move || loop {
        let subscription =
            pubsub_client::PubsubClient::program_subscribe(&url, &program_pub_key, None)
                .expect("Something went wrong");

        let conn = establish_connection();

        loop {
            let response = subscription.1.recv();
            match response {
                Ok(response) => {
                    let pda_pubkey = response.value.pubkey;
                    let pda_account: Account = response.value.account.decode().unwrap();

                    let stream = Stream::new(pda_pubkey, &pda_account.data);
                    match stream {
                        Some(a) => Stream::insert_or_update(a, &conn),
                        _ => {
                            println!("data didn't parsed");
                            continue;
                        }
                    };
                }
                Err(_) => {
                    break;
                }
            }
        }
        get_accounts_and_update()
    });
}

pub fn get_accounts_and_update() {
    let program_accounts = get_all_program_accounts();
    let conn = establish_connection();
    for item in program_accounts.iter() {
        let stream = Stream::new(item.0.to_string(), &item.1.data);
        match stream {
            Some(a) => Stream::insert_or_update(a, &conn),
            _ => continue,
        };
    }
}
