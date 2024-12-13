use ic_cdk::export::candid::{CandidType, Deserialize};
use ic_cdk_macros::{query, update, init};
use std::collections::HashMap;
use std::sync::RwLock;

#[derive(CandidType, Deserialize, Clone, Debug)]
struct Token {
    owner: String,
    balance: u64,
}

#[derive(Default)]
struct State {
    tokens: HashMap<String, Token>, // Mapping of owner to Token details
}

// Thread-safe, globally accessible state
lazy_static::lazy_static! {
    static ref STATE: RwLock<State> = RwLock::new(State {
        tokens: HashMap::new(),
    });
}

// Access mutable state
fn get_state() -> std::sync::RwLockWriteGuard<'static, State> {
    STATE.write().unwrap()
}

// Access immutable state
fn get_state_read() -> std::sync::RwLockReadGuard<'static, State> {
    STATE.read().unwrap()
}

// Safe arithmetic helpers
fn safe_add(a: u64, b: u64) -> Result<u64, String> {
    a.checked_add(b).ok_or_else(|| "Addition Overflow".to_string())
}

fn safe_sub(a: u64, b: u64) -> Result<u64, String> {
    a.checked_sub(b).ok_or_else(|| "Subtraction Overflow".to_string())
}

// Initialize the contract state
#[init]
fn init() {
    let mut state = get_state();
    state.tokens = HashMap::new();
}

// Sending tokens
#[update]
fn send_tokens(to: String, amount: u64) -> Result<(), String> {
    let mut state = get_state();
    let caller = ic_cdk::caller().to_string();

    // Check if the sender has enough balance
    if let Some(sender) = state.tokens.get_mut(&caller) {
        sender.balance = safe_sub(sender.balance, amount)?; // Safely subtract
    } else {
        return Err("Sender does not exist.".to_string());
    }

    // Update the receiver's balance
    let receiver = state.tokens.entry(to.clone()).or_insert(Token {
        owner: to.clone(),
        balance: 0,
    });
    receiver.balance = safe_add(receiver.balance, amount)?; // Safely add

    Ok(())
}

// Receiving tokens
#[update]
fn receive_tokens(amount: u64) -> Result<(), String> {
    let mut state = get_state();
    let caller = ic_cdk::caller().to_string();

    // Update the caller's token balance
    let token = state.tokens.entry(caller.clone()).or_insert(Token {
        owner: caller.clone(),
        balance: 0,
    });
    token.balance = safe_add(token.balance, amount)?; // Safely add

    Ok(())
}

// Fetching balance
#[query]
fn get_balance() -> u64 {
    let state = get_state_read();
    let caller = ic_cdk::caller().to_string();

    if let Some(token) = state.tokens.get(&caller) {
        token.balance
    } else {
        0
    }
}
