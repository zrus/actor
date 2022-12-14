use anyhow::Result;
use async_trait::async_trait;
use bastion::prelude::*;
use cqrs_es::Aggregate;
use log::{info, warn};

use crate::{
  actors::error::ActorError,
  aggregates::client::{aggregate::ClientState, commands::ClientCommand, events::ClientEvent},
};

use super::{
  state::{State, WeakState},
  traits::TActor,
  Actor,
};

pub struct ClientActor {
  supervisor: SupervisorRef,
}

impl ClientActor {
  pub fn stop(&self) -> Result<()> {
    self.supervisor.stop().map_err(|_| ActorError::StopError)?;
    info!("Stop successfully!");
    Ok(())
  }

  pub fn kill(&self) -> Result<()> {
    self.supervisor.kill().map_err(|_| ActorError::KillError)?;
    info!("Kill successfully!");
    Ok(())
  }
}

#[async_trait]
impl TActor for ClientActor {
  type State = ClientState;
  fn default() -> Actor<Self> {
    Actor::<Self>::default()
  }

  fn with_supervisor(supervisor: SupervisorRef) -> Self {
    Self { supervisor }
  }

  fn executor(children: Children, weak: &WeakState<Self::State>) -> Children {
    let state_weak = weak.clone();
    children.with_exec(move |ctx| {
      let state_weak = state_weak.clone();
      async move {
        let state: State<ClientState> = State::upgrade(state_weak);
        loop {
          MessageHandler::new(ctx.recv().await?)
            .on_tell(|cmd: ClientCommand, _| {
              run!(async {
                let read = state.read().await;
                let events = read.handle(cmd, &()).await.unwrap();
                Distributor::named("client").tell_one(events).unwrap();
              });
            })
            .on_tell(|events: Vec<ClientEvent>, _| {
              run!(async {
                let mut write = state.write().await;
                for evt in events {
                  write.apply(evt);
                }
                drop(write);
              })
            })
            .on_fallback(|unknown, _| warn!("Unknown message: {unknown:?}"));
        }
      }
    })
  }

  fn with_supervisor_callbacks() -> Option<Callbacks> {
    None
  }

  fn with_restart_strategy() -> Option<RestartStrategy> {
    None
  }

  fn with_strategy() -> Option<SupervisionStrategy> {
    None
  }

  fn with_children_callbacks() -> Option<Callbacks> {
    None
  }

  fn with_dispatcher() -> Option<Dispatcher> {
    None
  }

  fn with_distributor() -> Option<Distributor> {
    Some(Distributor::named("client"))
  }

  fn with_heartbeat_tick() -> Option<std::time::Duration> {
    None
  }

  fn with_name() -> Option<String> {
    None
  }

  fn with_redundancy() -> Option<usize> {
    None
  }

  fn with_resizer() -> Option<OptimalSizeExploringResizer> {
    None
  }
}
