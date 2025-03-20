use carbon_core::{borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0xe445a52e51cb9a1d5a1741233ef4bcd0")]
pub struct UpdateFeeConfigEvent {
    pub timestamp: i64,
    pub admin: solana_sdk::pubkey::Pubkey,
    pub lp_fee_basis_points: u64,
    pub protocol_fee_basis_points: u64,
    pub protocol_fee_recipients: [solana_sdk::pubkey::Pubkey; 8],
}
