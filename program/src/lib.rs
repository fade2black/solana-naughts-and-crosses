pub mod game;
use borsh::{BorshDeserialize, BorshSerialize};
use game::{Game, Move};
use solana_program::{
    account_info::{next_account_info, AccountInfo},
    entrypoint,
    entrypoint::ProgramResult,
    program_error::ProgramError,
    pubkey::Pubkey,
};

entrypoint!(process_instruction);

pub fn process_instruction(
    program_pk: &Pubkey,
    accounts: &[AccountInfo],
    data: &[u8],
) -> ProgramResult {
    let accounts_iter = &mut accounts.iter();
    let account = next_account_info(accounts_iter)?;

    if account.owner != program_pk {
        return Err(ProgramError::IncorrectProgramId);
    }

    let player = next_account_info(accounts_iter)?;

    let mv = Move::try_from_slice(data)?;
    let mut game = Game::try_from_slice(&account.data.borrow())?;

    game.play(*player.key, mv.row, mv.col, mv.mark);
    account.realloc(game.try_to_vec()?.len(), true)?;
    game.serialize(&mut &mut account.data.borrow_mut()[..])?;

    Ok(())
}
