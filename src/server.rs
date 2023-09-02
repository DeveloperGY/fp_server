mod receiver;
mod handler;
mod responder;
mod checker;

pub use receiver::Receiver;
pub use handler::Handler;
pub use responder::Responder;
pub use checker::ErrorChecker;

use std::sync::{Arc, Mutex};
use std::marker::Send;

pub struct Server<'a, 'b, 'c, 'd, Req, ReqErr, HanErr, Res, ResErr>
    where Req: Send, ReqErr: Send
{
    receiver: Arc<Mutex<&'a mut dyn Receiver<Req, ReqErr>>>,
    handler: Arc<Mutex<&'b mut dyn Handler<Req, Res, HanErr>>>,
    responder: Arc<Mutex<&'c mut dyn Responder<Res, ResErr>>>,
    error_checker: Arc<Mutex<&'d mut dyn ErrorChecker<ReqErr, HanErr, ResErr>>>
}

impl<'a, 'b, 'c, 'd, Req, ReqErr, HanErr, Res, ResErr> 
    Server<'a, 'b, 'c, 'd, Req, ReqErr, HanErr, Res, ResErr> 
        where Req: Send + 'static, ReqErr: Send
{
    pub fn new(
        receiver: &'a mut dyn Receiver<Req, ReqErr>,
        handler: &'b mut dyn Handler<Req, Res,HanErr>,
        responder: &'c mut dyn Responder<Res, ResErr>,
        error_checker: &'d mut dyn ErrorChecker<ReqErr, HanErr, ResErr>
    ) -> Self {
        let receiver = Arc::new(Mutex::new(receiver));
        let handler = Arc::new(Mutex::new(handler));
        let responder = Arc::new(Mutex::new(responder));
        let error_checker = Arc::new(Mutex::new(error_checker));
        
        Self {
            receiver,
            handler,
            responder,
            error_checker
        }
    }

    pub fn run(&mut self) {
        std::thread::scope(|s| {
            loop {
                let request = self.receiver.lock().unwrap().next_request();

                let handler = self.handler.clone();
                let responder = self.responder.clone();
                let error_checker = self.error_checker.clone();

                s.spawn(move || {
                    let request = match request {
                        Ok(r) => r,
                        Err(e) => {
                            error_checker.lock()
                                .unwrap()
                                .handle_receiver_error(e);
                            return;
                        }
                    };

                    let response = handler.lock()
                        .unwrap()
                        .handle_request(request);

                    let response = match response{
                        Ok(r) => r,
                        Err(e) => {
                            error_checker.lock()
                                .unwrap()
                                .handle_handler_error(e);
                            return;
                        }
                    };

                    let response_result = responder.lock()
                        .unwrap()
                        .send_response(response);
                    
                    match response_result {
                        Ok(_) => (),
                        Err(e) => {
                            error_checker.lock()
                                .unwrap()
                                .handle_responder_error(e);
                            return;
                        }
                    }
                });
            }
        });
    }
}