pub mod action_code;
pub mod admin;

use std::fmt::Debug;
use tokio::sync::mpsc;
use tokio::sync::oneshot;

/* ---------- Mark trait ---------- */

/// Service manager, handle messages on context.
pub trait ServiceManager
    <E: ServiceContext, C: ServiceCommand, R: ServiceResponse>: Debug + Send + Default
{
    fn msg_handler(&self, context: &mut E, msg: ServiceMsg<C, R>);
}

/// The context that manager works on.
pub trait ServiceContext: Debug + Send + Default { }

/// The command that send to manager in message.
pub trait ServiceCommand: Debug + Send { }

/// The response of command from service.
pub trait ServiceResponse: Debug + Send { }


/* ---------- Define ---------- */

#[derive(Debug)]
pub struct Service
    <T: ServiceManager<E, C, R>, E: ServiceContext, C: ServiceCommand, R: ServiceResponse>
{
    manager: T,
    context: E,
    /// [`None`] if service is not started. Remember to drop it if you borrowed one.
    msg_sender: Option<mpsc::Sender<ServiceMsg<C, R>>>,
}

impl<T, E, C, R> Default for Service<T, E, C, R> where
    T: ServiceManager<E, C, R>, E: ServiceContext, C: ServiceCommand, R: ServiceResponse
{
    fn default() -> Self { Self { manager: T::default(), context: E::default(), msg_sender: None } }
}


#[derive(Debug)]
pub struct ServiceMsg<C: ServiceCommand, R: ServiceResponse> {
    command: C,
    rsp_sender: oneshot::Sender<R>,
}


/* ---------- Interface ----------- */

impl<T, E, C, R> Service<T, E, C, R> where
    T: ServiceManager<E, C, R>,
    E: ServiceContext,
    C: ServiceCommand + 'static,
    R: ServiceResponse + 'static
{
    fn start(buffer: usize) -> mpsc::Sender<ServiceMsg<C, R>> {
        let (sender, mut channel)
            = mpsc::channel(buffer);
        let _sender = sender.clone();

        tokio::spawn(async move {
            let mut service = Self::default();
            service.msg_sender = Some(sender);

            while let Some(msg) = channel.recv().await {
                println!("{:?}, {:?}", msg, service);
            }
        });

        _sender
    }
}
