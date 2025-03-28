use solana_sdk::{pubkey, pubkey::Pubkey};

pub struct MeteoraPoolsDecoder;

pub mod accounts;
pub mod instructions;
pub mod types;

pub const PROGRAM_ID: Pubkey = pubkey!("Eo7WjKq67rjJQSZxS6z3YkapzY3eMj6Xy8X5EQVn5UaB");
