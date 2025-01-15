use arch_program::{
    self,
    account::{AccountInfo, AccountMeta},
    msg,
    entrypoint::{ProgramResult, MAX_PERMITTED_DATA_LENGTH},
    program_error::ProgramError,
    pubkey::Pubkey,
    program::{
        next_account_info,
        get_bitcoin_block_height,
        invoke,
        set_transaction_to_sign
    },
    system_instruction::{SystemInstruction},
    instruction::Instruction,
    bitcoin::{
        Transaction,
        transaction::Version,
        absolute::LockTime,
        consensus
    },
    helper::add_state_transition,
    transaction_to_sign::TransactionToSign,
    input_to_sign::InputToSign
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

    let rent;

    let ai_agent_account = next_account_info(account_iter)?;
    if !ai_agent_account.is_writable {
        return Err(ProgramError::InvalidAccountData);
    }

    let bitcoin_block_height = get_bitcoin_block_height();

    let mut params: AIAgentParams = borsh::from_slice(instruction_data).unwrap();

    if params.value != 0 {
        msg!("Got agent value {}, expected 0 for creation", params.value);
        return Err(ProgramError::InvalidArgument);
    }

    if ai_agent_account.is_writable {
        invoke(
            &SystemInstruction::new_create_account_instruction(
                params.utxo.txid().try_into().unwrap(),
                params.utxo.vout(),
                *ai_agent_account.key
            ),
            &[
                ai_agent_account.clone(),
            ],
        ).expect("Failed to create AI agent account");
    } else {
        msg!("AI agent account is not writable");
        return Err(ProgramError::InvalidAccountData);
    }

    let fees_tx: Transaction = consensus::deserialize(&params.tx_hex).unwrap();

    let mut tx = Transaction {
        version: Version::TWO,
        lock_time: LockTime::from_height(bitcoin_block_height.try_into().unwrap()).expect("Failed to create lock time"),
        input: instruction_data[0..4].to_vec().iter().map(|x| x.clone()).collect(),
        output: vec![],
    };
    add_state_transition(&mut tx, ai_agent_account);
    tx.input.push(fees_tx.input[0].clone());

    let tx_to_sign = TransactionToSign {
        tx_bytes: &consensus::serialize(&tx),
        inputs_to_sign: &[InputToSign {
            index: 0,
            signer: *signer_account.key,
        }]
    };
    set_transaction_to_sign(accounts, tx_to_sign)?;

    let offset = 4u32.to_le_bytes();
    instruction_data.extend_from_slice(&offset);

    let instruction = Instruction {
        program_id: *signer_account.key,
        accounts: vec![
            AccountMeta {
                pubkey: *ai_agent_account.key,
                is_signer: false,
                is_writable: true,
            },
        ],
        data: instruction_data,
    };
    set_transaction_to_sign(accounts, instruction)?;

    Ok(())
}