
use borsh::{BorshDeserialize,BorshSerialize};
use solana_program::{
    account_info::{next_account_info, AccountInfo},
    entrypoint::ProgramResult,
    msg,
    pubkey::Pubkey,
    entrypoint
};

#[derive(BorshDeserialize,BorshSerialize)]
enum InstructionType{
    Increament(u32),
    Decreament(u32)
}

#[derive(BorshDeserialize,BorshSerialize)]
struct Coutner{
    count:u32
}

entrypoint!(program_counter);


pub fn program_counter(
    _program_id:&Pubkey,
    accounts:&[AccountInfo],
    instruction_data:&[u8]

)->ProgramResult{

    let acc=next_account_info(& mut accounts.iter())?;
    let instruction_type=InstructionType::try_from_slice(instruction_data)?;
    let mut counter_data=Coutner::try_from_slice(&acc.data.borrow())?;

    match instruction_type{
        InstructionType::Increament(value)=>{
            counter_data.count +=value;

        },
        InstructionType::Decreament(value)=>{
            counter_data.count -=value;
        }
    } 
    counter_data.serialize(&mut *acc.data.borrow_mut())?;
    msg!("counter updated {}",counter_data.count);
    
    Ok(())

}