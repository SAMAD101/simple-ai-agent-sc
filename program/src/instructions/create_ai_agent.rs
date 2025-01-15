use arch_program::{
    self,
    account::AccountInfo,
    msg,
    entrypoint::ProgramResult,
    program_error::ProgramError,
    pubkey::Pubkey,
    program::next_account_info
};
use crate::states::*;

pub fn create_ai_agent(
    _program_id: &Pubkey,
    accounts: &[AccountInfo],
    instruction_data: &[u8]
) -> ProgramResult {
    msg!("AIAgent program is processing an instruction...");

    let account_iter = &mut accounts.iter();

    let signer_account = next_account_info(account_iter)?;
    if !signer_account.is_signer {
        return Err(ProgramError::MissingRequiredSignature);
    }

    let ai_agent_account = next_account_info(account_iter)?;
    if !ai_agent_account.is_writable {
        msg!("AI agent account is not writable");
        return Err(ProgramError::InvalidAccountData);
    }

    ai_agent_account.set_owner(signer_account.key);

    let mut state: AIAgent = AIAgentState::load(ai_agent_account).expect("Failed to load AI agent state");

    let mut params: AIAgentParams = borsh::from_slice(instruction_data).unwrap();

    state.name = params.name;
    state.creator = params.creator;
    state.value = if (params.value == 0) { 0 } else { return Err(ProgramError::InvalidArgument) };

    state.save(ai_agent_account).expect("Failed to save AI agent state");
    
    Ok(())
}