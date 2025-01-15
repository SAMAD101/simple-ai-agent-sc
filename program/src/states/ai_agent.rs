use arch_program::{ pubkey::Pubkey, utxo::UtxoMeta };
use borsh::{BorshDeserialize, BorshSerialize};

#[derive(Clone, Debug, BorshSerialize, BorshDeserialize)]
pub struct AIAgent {
    pub id: Pubkey,  // Unique identifier of the AI agent
    pub name: String,  // Name of the AI agent
    pub creator: Pubkey,  // Signer of the creation transaction
    pub value: u64,  // Token value of the AI agent on Arch Network
}

#[derive(Clone, Debug, BorshSerialize, BorshDeserialize)]
pub struct AIAgentParams {
    pub id: Pubkey,
    pub name: String,
    pub creator: Pubkey,
    pub value: u64,
    pub utxo: UtxoMeta,
    pub tx_hex: [u8]
}