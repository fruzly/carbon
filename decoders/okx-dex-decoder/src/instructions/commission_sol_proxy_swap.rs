use super::super::types::*;

use carbon_core::{borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0x1e21d05b1f9d2512")]
pub struct CommissionSolProxySwap {
    pub data: SwapArgs,
    pub commission_rate: u16,
    pub commission_direction: bool,
    pub order_id: u64,
}

pub struct CommissionSolProxySwapInstructionAccounts {
    pub payer: solana_pubkey::Pubkey,
    pub source_token_account: solana_pubkey::Pubkey,
    pub destination_token_account: solana_pubkey::Pubkey,
    pub source_mint: solana_pubkey::Pubkey,
    pub destination_mint: solana_pubkey::Pubkey,
    pub commission_account: solana_pubkey::Pubkey,
    pub sa_authority: solana_pubkey::Pubkey,
    pub source_token_sa: solana_pubkey::Pubkey,
    pub destination_token_sa: solana_pubkey::Pubkey,
    pub source_token_program: solana_pubkey::Pubkey,
    pub destination_token_program: solana_pubkey::Pubkey,
    pub associated_token_program: solana_pubkey::Pubkey,
    pub system_program: solana_pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for CommissionSolProxySwap {
    type ArrangedAccounts = CommissionSolProxySwapInstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let [payer, source_token_account, destination_token_account, source_mint, destination_mint, commission_account, sa_authority, source_token_sa, destination_token_sa, source_token_program, destination_token_program, associated_token_program, system_program, _remaining @ ..] =
            accounts
        else {
            return None;
        };

        Some(CommissionSolProxySwapInstructionAccounts {
            payer: payer.pubkey,
            source_token_account: source_token_account.pubkey,
            destination_token_account: destination_token_account.pubkey,
            source_mint: source_mint.pubkey,
            destination_mint: destination_mint.pubkey,
            commission_account: commission_account.pubkey,
            sa_authority: sa_authority.pubkey,
            source_token_sa: source_token_sa.pubkey,
            destination_token_sa: destination_token_sa.pubkey,
            source_token_program: source_token_program.pubkey,
            destination_token_program: destination_token_program.pubkey,
            associated_token_program: associated_token_program.pubkey,
            system_program: system_program.pubkey,
        })
    }
}
