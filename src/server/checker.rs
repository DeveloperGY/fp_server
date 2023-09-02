// Used for Doc Comments
#![allow(unused_imports)]
use super::Receiver;
use super::Handler;
use super::Responder;

/// An [`ErrorChecker`] handles errors that erupt from receiving or handling a
/// request
/// 
/// `ReqErr` is an error type that is handled by an [`ErrorChecker`] when 
/// [`Receiver::next_request()`] fails to receive a request.
/// It must match the type [`Receiver::next_request()`] returns.
/// 
/// `HanErr` is an error type that is handled by an [`ErrorChecker`] when 
/// [`Handler::handle_request()`] fails to handle a request.
/// It must match the type [`Handler::handle_request()`] returns.
/// 
/// `ResErr` is an error type that is handled by an [`ErrorChecker`] when
/// [`Responder::send_response()`] fails to send a response.
/// It must match the type [`Responder::send_response()`] returns.
pub trait ErrorChecker<ReqErr, HanErr, ResErr>: Send {
    fn handle_receiver_error(&mut self, err: ReqErr);
    fn handle_handler_error(&mut self, err: HanErr);
    fn handle_responder_error(&mut self, err: ResErr);
}