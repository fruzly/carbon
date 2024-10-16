use carbon_core::borsh;
use carbon_core::deserialize::CarbonDeserialize;
use carbon_proc_macros::CarbonDeserialize;

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0xe445a52e51cb9a1d63f91179a69ccfd7")]
pub struct IncreaseObservation {
    pub oracle: solana_sdk::pubkey::Pubkey,
    pub new_observation_length: u64,
}
