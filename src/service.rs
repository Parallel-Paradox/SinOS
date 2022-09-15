pub mod admin;

use std::fmt::Debug;
use std::marker::PhantomData;
use std::sync::Arc;
use tokio::sync::mpsc;
use tokio::sync::oneshot;

/* ---------- Mark trait ---------- */

/// Service manager, handle messages on context.
pub trait ServiceManager
    <E: ServiceContext, C: ServiceCommand, R: ServiceResponse>: Debug + Send + Default
{
    fn cmd_handler(&self, context: &mut E, command: C) -> R;
}

/// The context that manager works on.
pub trait ServiceContext: Debug + Send + Default { }

/// The command that send to manager in message.
pub trait ServiceCommand: Debug + Send { }

/// The response of command from service.
pub trait ServiceResponse: Debug + Send { }


/* ---------- Define ---------- */

#[derive(Debug)]
pub struct Service<T, E, C, R> where
    T: ServiceManager<E, C, R>,
    E: ServiceContext,
    C: ServiceCommand + 'static,
    R: ServiceResponse + 'static,
{
    manager: T,
    context: E,
    phant_c: PhantomData<C>,
    phant_r: PhantomData<R>,
}

impl<T, E, C, R> Default for Service<T, E, C, R> where
    T: ServiceManager<E, C, R>, E: ServiceContext, C: ServiceCommand, R: ServiceResponse
{
    fn default() -> Self {
        Self {
            manager: T::default(), context: E::default(),
            phant_c: PhantomData, phant_r: PhantomData
        }
    }
}

#[derive(Debug)]
pub struct CustomCommand<C:ServiceCommand, R: ServiceResponse> {
    pub command: C,
    pub rsp_sender: Option<oneshot::Sender<R>>,
}

#[derive(Debug)]
pub enum ServiceMsg<C:ServiceCommand, R: ServiceResponse> {
    Do(CustomCommand<C, R>), CloseService,
}


/* ---------- Interface ----------- */

impl<T, E, C, R> Service<T, E, C, R> where
    T: ServiceManager<E, C, R>,
    E: ServiceContext,
    C: ServiceCommand + 'static,
    R: ServiceResponse + 'static,
{
    pub fn start(buffer: usize) -> mpsc::Sender<ServiceMsg<C, R>> {
        let (sender, mut channel) =
            mpsc::channel(buffer);
        let _sender = sender.clone();

        tokio::spawn(async move {
            let mut service = Self::default();

            while let Some(msg) = channel.recv().await {
                match msg {
                    ServiceMsg::Do(command) => service.handle_msg(command),
                    ServiceMsg::CloseService => break,
                }
            }
        });

        _sender
    }

    fn handle_msg(&mut self, command: CustomCommand<C, R>) {
        let rsp = self.manager.cmd_handler(&mut self.context, command.command);
        if let None = command.rsp_sender { return }

        match command.rsp_sender.unwrap().send(rsp) {
            Err(err) => tracing::error!("{err:?}"),
            Ok(_) => { }
        }
    }
}

impl<C, R> ServiceMsg<C, R> where C:ServiceCommand, R: ServiceResponse {
    pub async fn send(self, sender: Arc<mpsc::Sender<Self>>) {
        match sender.send(self).await {
            Err(err) => tracing::error!("{err:?}"),
            Ok(_) => { },
        }
    }
}
