use arch_program::{account::AccountInfo, pubkey::Pubkey};
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
}

pub trait AIAgentState: Sized {
    fn try_serialize(&self) -> Result<Vec<u8>, std::io::Error>;
    fn load(account: &AccountInfo) -> Result<Self, std::io::Error>;
    fn save(&self, account: &AccountInfo) -> Result<(), std::io::Error>;
    fn increase_value(&mut self, value: u64);
}

impl AIAgentState for AIAgent {
    fn try_serialize(&self) -> Result<Vec<u8>, std::io::Error> {
        Ok(self.try_serialize()?)
    }

    fn load(account: &AccountInfo) -> Result<Self, std::io::Error> {
        let data = account.data.borrow();
        Self::try_from_slice(&data)
    }

    fn save(&self, account: &AccountInfo) -> Result<(), std::io::Error> {
        let data = self.try_serialize()?;
        let mut account_data = account.data.borrow_mut();
        account_data[..data.len()].copy_from_slice(&data);
        Ok(())
    }

    fn increase_value(&mut self, value: u64) {
        self.value += value;
    }
}