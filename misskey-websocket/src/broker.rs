use std::sync::Arc;

use crate::channel::WebSocketReceiver;
use crate::error::Result;

#[cfg(all(not(feature = "tokio-runtime"), feature = "async-std-runtime"))]
use async_std::sync::RwLock;
#[cfg(all(not(feature = "tokio-runtime"), feature = "async-std-runtime"))]
use async_std::task;
use futures::stream::StreamExt;
use log::{debug, info, warn};
#[cfg(all(feature = "tokio-runtime", not(feature = "async-std-runtime")))]
use tokio::sync::RwLock;
#[cfg(all(feature = "tokio-runtime", not(feature = "async-std-runtime")))]
use tokio::task;

pub mod channel;
pub mod handler;
pub mod model;

use channel::{control_channel, ControlReceiver, ControlSender};
use handler::Handler;
use model::{BrokerState, SharedBrokerState};

#[derive(Debug)]
pub(crate) struct Broker {
    websocket_rx: WebSocketReceiver,
    broker_rx: ControlReceiver,
    handler: Handler,
}

impl Broker {
    pub fn spawn(websocket_rx: WebSocketReceiver) -> (ControlSender, SharedBrokerState) {
        let state = Arc::new(RwLock::new(BrokerState::Working));
        let shared_state = Arc::clone(&state);

        let (broker_tx, broker_rx) = control_channel(Arc::clone(&state));

        let mut broker = Broker {
            websocket_rx,
            broker_rx,
            handler: Handler::new(),
        };

        task::spawn(async move {
            match broker.task().await {
                Ok(()) => {
                    info!("broker: exited normally");

                    let mut state = state.write().await;
                    *state = BrokerState::Exited;
                }
                Err(e) => {
                    warn!("broker: exited with error: {:?}", e);

                    let mut state = state.write().await;
                    *state = BrokerState::Dead(e);
                }
            }

            // This ensures that broker (and communication channels on broker side)
            // is dropped after `state` is surely set to `Dead` or `Exited`, thus asserts that the
            // state must be set to `Dead` or `Exited` when these channels are found out to be closed.
            std::mem::drop(broker);
        });

        (broker_tx, shared_state)
    }

    async fn clean_handler(&mut self) -> Result<()> {
        if self.handler.is_empty() {
            return Ok(());
        }

        info!("broker: handler is not empty, enter receiving loop");
        while !self.handler.is_empty() {
            let msg = self.websocket_rx.recv_json().await?;
            self.handler.handle(msg).await?;
        }

        Ok(())
    }

    async fn task(&mut self) -> Result<()> {
        use futures::future::{self, Either};

        info!("broker: started");

        loop {
            let t1 = self.websocket_rx.recv_json();
            let t2 = self.broker_rx.next();

            futures::pin_mut!(t1, t2);

            match future::select(t1, t2).await {
                Either::Left((msg, _)) => {
                    while let Some(ctrl) = self.broker_rx.try_recv() {
                        debug!("broker: received control {:?}", ctrl);
                        self.handler.update(ctrl);
                    }

                    self.handler.handle(msg?).await?;
                }
                Either::Right((Some(ctrl), _)) => {
                    debug!("broker: received control {:?}", ctrl);
                    self.handler.update(ctrl);
                }
                Either::Right((None, _)) => {
                    info!("broker: all controls terminated, exiting gracefully");
                    return self.clean_handler().await;
                }
            }
        }
    }
}
