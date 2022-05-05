mod kick;

use poise::{Command, Context};

use crate::{error::AppError, state::State};

use self::kick::kick;

pub type CommandContext<'a> = Context<'a, State, AppError>;

pub struct CommandState;

pub fn get_commands() -> Vec<Command<State, AppError>> {
    vec![kick()]
}
