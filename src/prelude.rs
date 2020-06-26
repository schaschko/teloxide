//! Commonly used items.

pub use crate::{
    dispatch,
    dispatching::{
        dialogue::{
            exit, next, DialogueDispatcher, DialogueStage, DialogueWithCx,
            GetChatId, TransitionIn, TransitionOut,
        },
        Dispatcher, DispatcherHandlerRx, DispatcherHandlerRxExt, UpdateWithCx,
    },
    error_handlers::{LoggingErrorHandler, OnError},
    requests::{Request, ResponseResult},
    types::{Message, Update},
    up, Bot, RequestError,
};

pub use frunk::{Coprod, Coproduct};
pub use tokio::sync::mpsc::UnboundedReceiver;

pub use futures::StreamExt;
