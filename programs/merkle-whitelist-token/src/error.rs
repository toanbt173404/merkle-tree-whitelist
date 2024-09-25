use anchor_lang::prelude::*;

#[error_code]
pub enum ProgramErrorCode {
    #[msg("The configuration account is already initialized.")]
    AlreadyInitialized,
    #[msg("You are not authorized to perform this action.")]
    Unauthorized,
   
}
