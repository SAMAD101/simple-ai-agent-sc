use arch_program::{
    account::{AccountInfo, AccountMeta},
    msg,
    entrypoint::ProgramResult,
    program::{
        next_account_info,
        get_clock,
        get_bitcoin_block_height,
        invoke
    },
    program_error::ProgramError,
    pubkey::Pubkey,
    system_instruction::{SystemInstruction},
    instruction::{Instruction, InstructionError},
    utxo::UtxoMeta,
    helper::add_state_transition,
    transaction_to_sign::TransactionToSign,
    input_to_sign::InputToSign
};
use arch_program::program::set_transaction_to_sign;
use bitcoin::{self, Transaction, transaction::Version, absolute::LockTime};

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
        return Err(ProgramError::InvalidAccountData);
    }

    let bitcoin_block_height = get_bitcoin_block_height();

    if ai_agent_account.utxo.clone() != UtxoMeta::from_slice(&[0; 36]) {
        msg!("UTXO {:?}", ai_agent_account.utxo.clone());
        return Err(ProgramError::Custom(502));
    }

    let clock = get_clock();
    let timestamp = clock.unix_timestamp;
    instruction_data.push(timestamp.to_le_bytes());

    let params: AIAgent = borsh::from_slice(instruction_data).unwrap();

    if ai_agent_account.is_writable {
        invoke(
            &SystemInstruction::new_create_account_instruction(
                params.utxo.txid().try_into().unwrap(),
                params.utxo.vout(),
                ai_agent_account.clone()
            ),
            &[
                signer_account.clone(),
                ai_agent_account.clone(),
            ],
        ).expect("Failed to create AI agent account");
    }

    let fees_tx: Transaction = bitcoin::consensus::deserialize(&params.tx_hex).unwrap();

    let mut tx = Transaction {
        version: Version::TWO,
        lock_time: LockTime::from_height(bitcoin_block_height),
        input: vec![],
        output: vec![],
    };
    add_state_transition(&mut tx, ai_agent_account);
    tx.input.push(fees_tx.input[0].clone());

    let tx_to_sign = TransactionToSign {
        tx_bytes: &bitcoin::consensus::serialize(&tx),
        inputs_to_sign: &[InputToSign {
            index: 0,
            signer: *signer_account.key,
        }]
    };

    set_transaction_to_sign(accounts, &tx_to_sign)?;

    Ok(())
}
