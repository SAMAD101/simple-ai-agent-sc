use arch_program::{ pubkey::Pubkey, utxo::UtxoMeta };
use borsh::{BorshDeserialize, BorshSerialize};

#[derive(Clone, Debug, BorshSerialize, BorshDeserialize)]
pub struct AIAgent {
    pub id: Pubkey, // Unique identifier of the AI agent
    pub name: String,  // Name of the AI agent
    pub creator: Pubkey, // Signer of the creation transaction
    pub creation_time: i64, // Unix timestamp
    pub description: String, // Description of the AI agent
    pub model: String,  // Model used in the AI agent
    pub version: String,  // Version of the AI agent
    pub url: String,  // URL of the AI agent
    pub value: u64,  // Token value of the AI agent on Arch Network
    pub tx_hex: String, // Transaction hex of the AI agent
    pub utxo: UtxoMeta, // UTXO of the AI agent
}