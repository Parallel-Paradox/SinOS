pub mod admin;
mod action_code;

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
    fn cmd_handler(&self, context: &mut E, command: CustomCommand<C, R>);
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

// TODO: Add 'Pause' and 'ForceClose' command
//  'ForceClose': Close service channel immediately, even there's still sender exist.
#[derive(Debug)]
pub enum ServiceMsg<C:ServiceCommand, R: ServiceResponse> {
    Do(CustomCommand<C, R>),
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
                }
            }
        });

        _sender
    }

    fn handle_msg(&mut self, command: CustomCommand<C, R>) {
        self.manager.cmd_handler(&mut self.context, command);
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

pub fn send_response<R>(response: Option<R>, rsp_sender: Option<oneshot::Sender<R>>) where
    R: ServiceResponse
{
    if let Some(rsp_sender) = rsp_sender {
        if let Some(rsp) = response {
            // send rsp
            if let Err(err) = rsp_sender.send(rsp) {
                tracing::error!("Response {err:?} send fail!");
            }
        } else {
            // drop the sender and warn
            tracing::warn!("Send a receiver but no response is generated.");
        }
    } else {
        if let Some(_) = response {
            // drop the rsp and warn
            tracing::warn!("Generate a response but no receiver.");
        }
    }
}
