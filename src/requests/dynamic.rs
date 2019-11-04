use serde::de::DeserializeOwned;
use reqwest::multipart;

use crate::{Bot, network};
use super::{ResponseResult, Method};

pub enum Kind {
    Simple,
    Json(String),
    Multipart(multipart::Form),
}

pub trait Payload {
    // NOTE: This payload doesn't use `Method` and reinvent `type Output`
    //  because the trait `Method` cannot be made into an object.
    type Output;

    fn method(&self) -> &str;

    fn kind(&self) -> Kind;
}

/// Dynamic ready-to-send telegram request.
///
/// This type is useful for storing different requests in one place, however
/// this type has _little_ overhead, so prefer using [json], [multipart] or
/// [simple] requests when possible.
///
/// [json]: crate::requests::json::Request
/// [multipart]: crate::requests::multipart::Request
/// [simple]: crate::requests::simple::Request
#[must_use = "requests do nothing until sent"]
pub struct Request<'b, P> {
    bot: &'b Bot,
    pub(crate) payload: P,
}

impl<'b, P> Request<'b, P>
where
    P: Payload,
    P::Output: DeserializeOwned,
{
    pub fn new(bot: &'b Bot, payload: P) -> Self {
        Self { bot, payload }
    }

    /// Send request to telegram
    pub async fn send(&self) -> ResponseResult<P::Output> {
        network::request_dynamic(
            self.bot.client(),
            self.bot.token(),
            self.payload.method(),
            self.payload.kind(),
        ).await
    }
}
