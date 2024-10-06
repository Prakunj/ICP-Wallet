#[cfg(test)]
mod tests {
    use crate::{get_balance, receive_tokens, send_tokens, TokenAccount, TokenWallet, WALLET};

    // Mock setup function
    fn mock_setup() {
        unsafe {
            WALLET = Some(TokenWallet {
                accounts: vec![
                    (
                        "account_1".to_string(),
                        TokenAccount {
                            balance: 1_000_000_000,
                        },
                    ),
                    (
                        "account_2".to_string(),
                        TokenAccount {
                            balance: 2_000_000_000,
                        },
                    ),
                ]
                .into_iter()
                .collect(),
            });
        }
    }

    #[test]
    fn test_get_balance() {
        mock_setup();

        let balance = get_balance("account_1".to_string());
        assert_eq!(balance, 1_000_000_000);

        let balance = get_balance("account_2".to_string());
        assert_eq!(balance, 2_000_000_000);

        let balance = get_balance("non_existent_account".to_string());
        assert_eq!(balance, 0);
    }

    #[test]
    fn test_send_tokens() {
        mock_setup();

        let result = send_tokens("account_1".to_string(), 500_000_000);
        assert_eq!(result, "Sent 500000000 tokens to account_2");

        assert_eq!(get_balance("account_1".to_string()), 600_000_000);
    }

    #[test]
    fn test_receive_tokens() {
        mock_setup();

        let result = receive_tokens("account_2".to_string(), 1_000_000_000);
        assert_eq!(result, "Received 1000000 tokens into account_2");

        let caller = ic_cdk::caller().to_string();
        assert_eq!(get_balance(caller), 1_000_000);
    }
}
