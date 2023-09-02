// Used for Doc Comments
#![allow(unused_imports)]
use super::Receiver;
use super::Responder;
use super::ErrorChecker;

/// Handles client requests and generates a response
/// 
/// `Req` is a request type that is handled by a [`Handler`].
/// It must match the type that [`Receiver::next_request()`] returns.
/// 
/// `Res` is a response type that is returned when a [`Handler`] successfully
/// handles a request.
/// It must match the type that [`Responder::send_response()`] expects.
/// 
/// `HanErr` is an error type that is returned when a [`Handler`] failed to
/// handle a request.
/// It must match the type that [`ErrorChecker::handle_handler_error()`] expects.
pub trait Handler<Req, Res, HanErr>: Send {
    fn handle_request(&mut self, request: Req) -> Result<Res, HanErr>;
}