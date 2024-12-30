use borsh::{BorshDeserialize, BorshSerialize};
use solana_program::{
    account_info::AccountInfo, entrypoint,entrypoint::{ProgramResult}, msg, pubkey::{self, Pubkey}
};

entrypoint!(ProcessInstruction);

pub fn ProcessInstruction(
    program_id:&Pubkey,
    accounts: &[AccountInfo],
    instruction_data: &[u8],
) -> ProgramResult {
    msg!("Hello Solana!!!");
    Ok(())
}
