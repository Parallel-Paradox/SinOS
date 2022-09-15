//! Admin is the manager of all service, which is also a service. This service will be created when
//! server initializing. The server will hold a sender of Admin as an extension. This makes the
//! sender can be accessed by every router.
//!
//! Admin holds a map of <service_id, service_msg_sender>. Send the target service id and the
//! message to admin, admin will redirect it to the right place.
//!
//! An Admin service can hold a sub admin.

use std::collections::HashMap;
use std::sync::Arc;
use axum::Extension;
use nanoid::nanoid;
use crate::service::*;
use crate::service::action_code::*;
use crate::constant::*;

pub type Admin = Service<Manager, Context, Command, Response>;
pub type AdminMsg = ServiceMsg<Command, Response>;
pub type AdminCmd = CustomCommand<Command, Response>;

pub type AdminExt = Extension<Arc<mpsc::Sender<AdminMsg>>>;

#[derive(Debug, Default)]
pub struct Manager { }
impl ServiceManager<Context, Command, Response> for Manager {
    fn cmd_handler(&self, context: &mut Context, command: AdminCmd) {
        let mut response: Option<Response> = None;
        match command.command {
            Command::CreateActionCode => {
                let id = nanoid!();
                let sender = Arc::new(ActionCode::start(32));
                context.sender_map.insert(id.clone(), ServiceSender::ActionCode(sender));
                response = Some(Response::ServiceID(id));
            }
            Command::GetSenderWithID(id) => {
                let query = context.sender_map.get(&id);
                response = match query {
                    Some(sender) => Some(Response::ServiceSender(sender.clone())),
                    None => Some(Response::Err(SERVICE_ID_NOT_EXIST)),
                };
            }
            Command::CloseWithID(id) => {
                let query = context.sender_map.remove(&id);
                response = match query {
                    Some(_) => Some(Response::OK),
                    None => Some(Response::Err(SERVICE_ID_NOT_EXIST)),
                };
            }
            Command::Echo => { tracing::debug!("{context:?} {command:?}"); }
        }
        send_response(response, command.rsp_sender);
    }
}

#[derive(Debug, Default)]
pub struct Context {
    sender_map: HashMap<String, ServiceSender>
}
impl ServiceContext for Context { }

#[derive(Debug)]
pub enum Command {
    /// [`Response::ServiceID`]
    CreateActionCode,
    /// [`Response::ServiceSender`]
    GetSenderWithID(String),
    /// [`Response::OK`] or [`Response::Err`]
    ///
    /// This will drop the entry in the sender map, when all [`mpsc::Sender`] is dropped, the
    /// [`mpsc::Receiver`] channel will close automatically.
    CloseWithID(String),
    /// [`Response::OK`] or [`Response::Err`]
    Echo,
}
impl ServiceCommand for Command { }

#[derive(Debug)]
pub enum Response {
    ServiceID(String),
    ServiceSender(ServiceSender),
    OK,
    Err(usize),
}
impl ServiceResponse for Response { }

#[derive(Debug, Clone)]
pub enum ServiceSender {
    Admin(Arc<mpsc::Sender<AdminMsg>>),
    ActionCode(Arc<mpsc::Sender<ActionCodeMsg>>),
}
