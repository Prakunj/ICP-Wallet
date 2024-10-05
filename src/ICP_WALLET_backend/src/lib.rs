use candid::{CandidType, Deserialize};
use ic_cdk_macros::{init, update};
use std::collections::HashMap;

#[derive(CandidType, Deserialize, Clone, Debug)]
struct TokenAccount {
    balance: u64,
}

#[ic_cdk::query]
fn greet(name: String) -> String {
    format!("Hello, {}!", name)
}

#[derive(Default)]
struct TokenWallet {
    accounts: HashMap<String, TokenAccount>,
}

static mut WALLET: Option<TokenWallet> = None;

#[init]
fn init() {
    // ic_cdk::setup();
    ic_cdk::print(format!("Initializing wallet23"));
    unsafe {
        let mut accounts = HashMap::new();
        // Initialize with some predefined accounts and their balances
        accounts.insert(
            "account_1".to_string(),
            TokenAccount {
                balance: 1_000_000_000,
            },
        );
        accounts.insert(
            "account_2".to_string(),
            TokenAccount {
                balance: 2_000_000_000,
            },
        );

        WALLET = Some(TokenWallet { accounts });
    }
}

#[update]
fn send_tokens(receiver: String, amount: u64) -> String {
    unsafe {
        if WALLET.is_none() {
            return "Wallet not initialized2".to_string();
        }

        let wallet = WALLET.as_mut().unwrap();

        let sender = ic_cdk::caller().to_string();
        let sender_account = wallet
            .accounts
            .entry(sender.clone())
            .or_insert(TokenAccount {
                balance: 3_000_000_000,
            });
        if sender_account.balance < amount {
            return "Insufficient balance".to_string();
        }

        sender_account.balance -= amount;

        let receiver_account = wallet
            .accounts
            .entry(receiver.clone())
            .or_insert(TokenAccount { balance: 0 });
        receiver_account.balance += amount;

        format!("Sent {} tokens to {}", amount, receiver)
    }
}

#[ic_cdk::query]
fn get_balance(account: String) -> u64 {
    unsafe {
        let wallet = WALLET.as_ref().expect("Wallet not initialized");
        wallet.accounts.get(&account).map_or(0, |acc| acc.balance)
    }
}

#[update]
fn receive_tokens(account: String, amount: u64) -> String {
    unsafe {
        let wallet = WALLET.as_mut().expect("Wallet not initialized");

        let receiver_account = wallet
            .accounts
            .entry(account.clone())
            .or_insert(TokenAccount { balance: 0 });

        receiver_account.balance += amount;
        format!("Received {} tokens into {}", amount, account)
    }
}
