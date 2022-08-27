mod any_command;
mod any_request;
mod any_result;
mod command;
mod command_sender;

pub(crate) use any_command::AnyCommand;
pub(crate) use any_request::AnyRequest;
pub(crate) use any_result::AnyResult;
pub use command::Command;
pub use command_sender::CommandSender;

#[macro_export]
macro_rules! request {
    ($sender:ident, |$x:ident: &mut $xt:ty| $expression:expr) => {
        $sender
            .request(move |$x: &mut $xt| $expression.boxed())
            .await
    };
}
