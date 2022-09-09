use std::fmt::Debug;
use tokio::sync::mpsc;
use tokio::sync::oneshot;

/* ---------- Semantic trait ---------- */

pub trait CrossService: Debug + Send { }

/// Service manager, handle messages on context.
pub trait Manager<E: Context, M: Message>: CrossService + Default {
    fn msg_handler(&self, context: &mut E, msg: M);
}

/// The context that manager works on.
pub trait Context: CrossService + Default { }

/// The command that send to manager in message.
pub trait Command: CrossService + Default { }

/// Hold a [`oneshot::Sender`] for sending response to the command sender.
pub trait Response: CrossService { }

/// Message that contains Command and Response. See [`ServiceMsg`].
pub trait Message: CrossService { }


/* ---------- Define ---------- */

#[derive(Debug)]
pub struct Service<T: Manager<E, ServiceMsg<C, R>>, E: Context, C: Command, R: Response> {
    manager: T,
    context: E,
    /// [`None`] if service is not started. Remember to drop it if you borrowed one.
    msg_sender: Option<mpsc::Sender<ServiceMsg<C, R>>>,
}

unsafe impl<T, E, C, R> Send for Service<T, E, C, R> where
    T: Manager<E, ServiceMsg<C, R>>, E: Context, C: Command, R: Response { }

impl<T, E, C, R> Default for Service<T, E, C, R> where
    T: Manager<E, ServiceMsg<C, R>>, E: Context, C: Command, R: Response
{
    fn default() -> Self { Self { manager: T::default(), context: E::default(), msg_sender: None } }
}


#[derive(Debug)]
pub struct ServiceMsg<C: Command, R: Response> {
    command: C,
    rsp_sender: oneshot::Sender<R>,
}
unsafe impl<C, R> Send for ServiceMsg<C, R> where C: Command, R: Response { }
impl<C, R> CrossService for ServiceMsg<C, R> where C: Command, R: Response { }
impl<C, R> Message for ServiceMsg<C, R> where C: Command, R: Response { }


/* ---------- Interface ----------- */

impl<T, E, C, R> Service<T, E, C, R> where
    T: Manager<E, ServiceMsg<C, R>>, E: Context, C: Command, R: Response
{
    fn start(buffer: usize) -> mpsc::Sender<i32> {
        let (sender, mut channel)
            = mpsc::channel(buffer);
        let msg_sender = sender.clone();

        tokio::spawn(async move {
            let mut service = Self::default();

            while let Some(msg) = channel.recv().await {
                println!("{:?}, {:?}", msg, service);
            }
        });

        msg_sender
    }

    async fn cast(&self, command: C) -> R {
        let (rsp_sender, rsp_receiver) = oneshot::channel();
        let msg = ServiceMsg { command, rsp_sender };
        let msg_sender = self.msg_sender.clone();

        // TODO: Process uninit panic.
        msg_sender.unwrap().send(msg).await.unwrap();

        // TODO: Process out of time error.
        rsp_receiver.await.unwrap()
    }
}
