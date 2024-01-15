macro_rules! define_global_command {
    ($vis:vis, $ident:ident, $name:literal, $summary:literal, $exec:expr) => {
        paste::paste! {
            #[allow(non_snake_case)]
            mod [<_ $ident _MOD>] {
                use crate::commands::{Command, CommandContext, CommandError, GLOBAL_COMMANDS};

                #[derive(Clone, PartialEq, Eq, serde::Deserialize, serde::Serialize)]
                pub struct $ident;

                #[typetag::serde]
                impl Command for $ident {
                    fn name(&self) -> &'static str {
                        $name
                    }
                    fn summary(&self) -> &'static str {
                        $summary
                    }

                    fn execute(
                        &self,
                        context: &CommandContext,
                        args: String,
                        world: &mut bevy::prelude::World,
                    ) -> Result<(), CommandError> {
                        ($exec)(self, context, args, world)
                    }
                }

                #[linkme::distributed_slice(GLOBAL_COMMANDS)]
                static _STATIC_REF: &'static dyn Command = &$ident;
            }
            $vis use [<_ $ident _MOD>]::$ident;
        }
    };
    ($ident:ident, $name:literal, $summary:literal, $exec:expr) => {
        define_global_command!(pub, $ident, $name, $summary, $exec);
    };
}
pub(crate) use define_global_command;
