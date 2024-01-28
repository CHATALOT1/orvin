//! Commands that are **always** available to run, regardless of any potential limitation.
//! These can be run by both admin and player clients.
use super::*;

pub struct GlobalCommand {
    command: &'static dyn Command,
    name: &'static str,
}

#[distributed_slice]
static GLOBAL_COMMANDS: [GlobalCommand];

pub fn setup_global_commands(mut commands: Commands) {
    for cmd in GLOBAL_COMMANDS {
        commands.spawn((Name::new(cmd.name), AvailableCommand::Static(cmd.command)));
    }
}

macro_rules! define_global_command {
    ($vis:vis, $ident:ident, $name:literal, $exec:expr) => {
        paste::paste! {
            #[allow(non_snake_case)]
            mod [<_ $ident _MOD>] {
                use super::*;

                #[derive(Clone, PartialEq, Eq, serde::Deserialize, serde::Serialize)]
                pub struct $ident;

                #[typetag::serde]
                impl Command for $ident {
                    fn execute(&self, context: &CommandContext) -> Result<(), CommandError> {
                        ($exec)(context)
                    }
                }

                #[linkme::distributed_slice(GLOBAL_COMMANDS)]
                static _STATIC_REF: GlobalCommand = GlobalCommand { command: &$ident, name: $name };
            }
            #[allow(unused_imports)]
            $vis use [<_ $ident _MOD>]::$ident;
        }
    };
    ($ident:ident, $name:literal, $exec:expr) => {
        define_global_command!(pub, $ident, $name, $exec);
    };
}

#[cfg(feature = "test-command")]
define_global_command!(TestCommand, "test", |ctx: &CommandContext| {
    ctx.output_append(&format!("Hello {}", ctx.args));
    Ok(())
});
