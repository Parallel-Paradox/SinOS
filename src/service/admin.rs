use crate::service::*;

pub type Admin = Service<Manager, Context, Command, Response>;
pub type AdminMsg = ServiceMsg<Command, Response>;

#[derive(Debug, Default)]
struct Manager { }
impl ServiceManager<Context, Command, Response> for Manager {
    fn msg_handler(&self, context: &mut Context, msg: ServiceMsg<Command, Response>) {
        println!("{:?}", context);
        println!("{:?}", msg);
    }
}

#[derive(Debug, Default)]
struct Context { }
impl ServiceContext for Context { }

#[derive(Debug)]
pub enum Command { }
impl ServiceCommand for Context { }

#[derive(Debug)]
pub enum Response { }
impl ServiceResponse for Response { }
