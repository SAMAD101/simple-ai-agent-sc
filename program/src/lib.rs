mod instructions;
mod states;

use arch_program::{
    account::AccountInfo,
    entrypoint,
    entrypoint::ProgramResult,
    msg,
    program_error::ProgramError,
    pubkey::Pubkey
};
use borsh::{BorshDeserialize, BorshSerialize};

use instructions::*;

use std::mem;
use crate::states::AIAgent;

entrypoint!(process_instruction);

#[derive(Debug, Clone, BorshSerialize, BorshDeserialize)]
enum AIAgentInstruction {
    CreateAIAgent { args: [u8; mem::size_of::<AIAgent>()] },
    IncreaseAIAgentValue { args: [u8; mem::size_of::<AIAgent>()] },
}

pub fn process_instruction(
    _program_id: &Pubkey,
    accounts: &[AccountInfo],
    instruction_data: &[u8]
) -> ProgramResult {
    let instruction = AIAgentInstruction::try_from_slice(instruction_data).map_err(|_| {
        msg!("Failed to deserialize instruction");
        ProgramError::InvalidInstructionData
    })?;

    match instruction {
        AIAgentInstruction::CreateAIAgent { args } => create_ai_agent(_program_id, accounts, &args),
        AIAgentInstruction::IncreaseAIAgentValue { args } => increase_ai_agent_value(_program_id, accounts, &args),
        _ => return Err(ProgramError::InvalidInstructionData),
    }
}