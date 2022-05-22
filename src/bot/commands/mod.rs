mod kick;

use poise::{Command, Context};

use crate::error::AppError;

use self::kick::kick;

use super::state::State;

pub type CommandContext<'a> = Context<'a, State, AppError>;

pub struct CommandState;

pub fn get_commands() -> Vec<Command<State, AppError>> {
    vec![kick()]
}
