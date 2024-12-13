#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_safe_add() {
        // Test safe addition
        assert_eq!(safe_add(10, 20).unwrap(), 30);
        assert!(safe_add(u64::MAX, 1).is_err()); // Overflow should return an error
    }

    #[test]
    fn test_safe_sub() {
        // Test safe subtraction
        assert_eq!(safe_sub(20, 10).unwrap(), 10);
        assert!(safe_sub(10, 20).is_err()); // Underflow should return an error
    }

    #[test]
    fn test_receive_tokens() {
        // Initialize state
        init();

        // Call receive_tokens
        assert!(receive_tokens(100).is_ok());

        // Verify balance
        let balance = get_balance();
        assert_eq!(balance, 100);
    }

    #[test]
    fn test_send_tokens() {
        // Initialize state
        init();

        // Receive tokens for the sender
        assert!(receive_tokens(100).is_ok());

        // Send tokens to another user
        let recipient = "recipient_id".to_string();
        assert!(send_tokens(recipient.clone(), 50).is_ok());

        // Verify sender balance
        let sender_balance = get_balance();
        assert_eq!(sender_balance, 50);

        // Verify recipient balance
        let state = get_state_read();
        let recipient_token = state.tokens.get(&recipient).unwrap();
        assert_eq!(recipient_token.balance, 50);
    }

    #[test]
    fn test_send_tokens_insufficient_balance() {
        // Initialize state
        init();

        // Try sending tokens without receiving any first
        let recipient = "recipient_id".to_string();
        let result = send_tokens(recipient, 10);

        // Verify that an error is returned
        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), "Sender does not exist.".to_string());
    }

    #[test]
    fn test_send_tokens_overdraft() {
        // Initialize state
        init();

        // Receive tokens for the sender
        assert!(receive_tokens(50).is_ok());

        // Try sending more tokens than available
        let recipient = "recipient_id".to_string();
        let result = send_tokens(recipient, 100);

        // Verify that an error is returned
        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), "Subtraction Overflow".to_string());
    }

    #[test]
    fn test_get_balance_no_tokens() {
        // Initialize state
        init();

        // Verify balance for a new caller with no tokens
        let balance = get_balance();
        assert_eq!(balance, 0);
    }
}
