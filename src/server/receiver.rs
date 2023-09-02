// Used for Doc Comments
#![allow(unused_imports)]
use super::Handler;
use super::ErrorChecker;

use std::marker::Send;

/// Recieves client requests
/// 
/// `Req` is a request type that is returned when a [`Receiver`] successfully
/// receives a request. 
/// It must match the type that [`Handler::handle_request()`] expects.
/// 
/// `ReqErr` is an error type that is returned when a [`Receiver`] failed to
/// receive a request.
/// It must match the type that [`ErrorChecker::handle_receiver_error()`] expects.
pub trait Receiver<Req, ReqErr>: Send 
    where Req: Send, ReqErr: Send
{

    /// Returns the next request or an error if it cannot receive a request
    fn next_request(&mut self) -> Result<Req, ReqErr>;
}