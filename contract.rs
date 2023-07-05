use solana_program::{
    account_info::AccountInfo, entrypoint, entrypoint::ProgramResult, msg, pubkey::Pubkey,
};

// declare and export the program's entrypoint
entrypoint!(process_instruction);

// program entrypoint's implementation
pub fn process_instruction(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    instruction_data: &[u8],
) -> ProgramResult {
    // log a message to the blockchain
    let x = str("ABDAASDA".to_string());
    msg!(&x);
    // gracefully exit the program
    Ok(())
}

pub fn str(a:String) -> String {
    let b = "ABCDABCD";
    let result = [a,b.to_string()].join("\n");
    return result
}
