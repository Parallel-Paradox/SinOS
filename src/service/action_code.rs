// TODO register router into app
// TODO test create action code

use crate::service::*;

pub type ActionCode = Service<Manager, Context, Command, Response>;
pub type ActionCodeMsg = ServiceMsg<Command, Response>;
pub type ActionCodeCmd = CustomCommand<Command, Response>;

#[derive(Debug, Default)]
pub struct Manager { }
impl ServiceManager<Context, Command, Response> for Manager {
    fn cmd_handler(&self, context: &mut Context, command: ActionCodeCmd) {
        tracing::debug!("{context:?} {command:?}");
    }
}

#[derive(Debug, Default)]
pub struct Context { }
impl ServiceContext for Context { }

#[derive(Debug)]
pub enum Command { Echo }
impl ServiceCommand for Command { }

#[derive(Debug)]
pub enum Response { OK }
impl ServiceResponse for Response { }
