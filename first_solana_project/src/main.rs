use solana_program::{
    account_info::AccountInfo,
    entrypoint,
    entrypoint::ProgramResult,
    msg,
    pubkey::PubKey
}

entrypoint!(process_instruction);

pub fn process_instruction(
    _accounts:&[AccountInfo],
    _program_id:PubKey,
    _instruction_data:&[u8]) -> ProgramResult{
    msg!("Hello World");
    Ok(())
}

//ProgramResult is a enum. There are two outcomes.Ok(T)/Err(E)
 
