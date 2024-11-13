use carbon_core::{borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0xe445a52e51cb9a1ddff5e099311da3ac")]
pub struct UpdateRewardDuration {
    pub lb_pair: solana_sdk::pubkey::Pubkey,
    pub reward_index: u64,
    pub old_reward_duration: u64,
    pub new_reward_duration: u64,
}
