macro_rules! define_global_command {
    ($vis:vis, $ident:ident, $name:literal, $exec:expr) => {
        paste::paste! {
            #[allow(non_snake_case)]
            mod [<_ $ident _MOD>] {
                use crate::commands::{Command, CommandContext, CommandError, GLOBAL_COMMANDS, GlobalCommand};

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
            $vis use [<_ $ident _MOD>]::$ident;
        }
    };
    ($ident:ident, $name:literal, $exec:expr) => {
        define_global_command!(pub, $ident, $name, $exec);
    };
}
pub(super) use define_global_command;
