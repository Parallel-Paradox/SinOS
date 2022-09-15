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
use crate::service::*;

pub type Admin = Service<Manager, Context, Command, Response>;
pub type AdminMsg = ServiceMsg<Command, Response>;
pub type AdminCmd = CustomCommand<Command, Response>;

pub type AdminExt = Extension<Arc<mpsc::Sender<AdminMsg>>>;

#[derive(Debug, Default)]
pub struct Manager { }
impl ServiceManager<Context, Command, Response> for Manager {
    fn cmd_handler(&self, context: &mut Context, command: Command) -> Response {
        tracing::debug!("{context:?} {command:?}");
        Response::OK
    }
}

#[derive(Debug, Default)]
pub struct Context {
    sender_map: HashMap<String, ServiceSender>
}
impl ServiceContext for Context { }

#[derive(Debug)]
pub enum Command { Echo }
impl ServiceCommand for Command { }

#[derive(Debug)]
pub enum Response { OK }
impl ServiceResponse for Response { }

#[derive(Debug)]
enum ServiceSender {
    Admin(Arc<mpsc::Sender<AdminMsg>>)
}
