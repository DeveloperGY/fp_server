// Used for Doc Comments
#![allow(unused_imports)]
use super::Handler;
use super::ErrorChecker;

/// Responds to the client
/// 
/// `Res` is a response type that is sent by a [`Responder`].
/// It must match the type that [`Handler::handle_request()`] returns.
/// 
/// `ResErr` is an error type that is returned when a [`Responder`] fails to 
/// send a response.
/// It must match the type that [`ErrorChecker::handle_responder_error()`]
/// expects.
pub trait Responder<Res, ResErr>: Send {
    fn send_response(&mut self, response: Res) -> Result<(), ResErr>;
}