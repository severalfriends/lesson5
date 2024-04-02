use solana_program::program_error::ProgramError;
use thiserror::Error;

#[derive(Debug, Error)]

pub enum NoteError {
    #[error("Note title exceeds max length")]
    InvalidNoteTitleLength,
    #[error("Note body exceeds max length")]
    InvalidNoteBodyLength,
    #[error("Invalid update note authority")]
    InvalidNoteAuthority,
}

impl From<NoteError> for ProgramError {
    fn from(value: NoteError) -> Self {
        ProgramError::Custom(value as u32)
    }
}