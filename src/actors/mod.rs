pub mod client;
pub mod error;
pub mod state;
pub mod traits;

use std::marker::PhantomData;
use std::time::Duration;

use anyhow::Result;
use bastion::prelude::*;

use self::error::ActorError;
use self::state::WeakState;
use self::traits::TActor;

pub struct Actor<A> {
  // For supervisor
  supervisor_callbacks: Option<Callbacks>,
  restart_strategy: Option<RestartStrategy>,
  strategy: Option<SupervisionStrategy>,
  // For children
  children_callbacks: Option<Callbacks>,
  dispatcher: Option<Dispatcher>,
  distributor: Option<Distributor>,
  heartbeat_tick: Option<Duration>,
  name: Option<String>,
  redundancy: Option<usize>,
  resizer: Option<OptimalSizeExploringResizer>,
  // For marker
  phantom_actor: PhantomData<A>,
}

impl<A> Actor<A>
where
  A: TActor + 'static,
{
  pub fn default() -> Self {
    Self {
      supervisor_callbacks: None,
      restart_strategy: None,
      strategy: None,
      children_callbacks: None,
      dispatcher: None,
      distributor: None,
      heartbeat_tick: None,
      name: None,
      redundancy: None,
      resizer: None,
      phantom_actor: PhantomData,
    }
  }

  pub fn run_with_supervisor(
    self,
    supervisor: SupervisorRef,
    weak: &WeakState<A::State>,
  ) -> Result<A> {
    supervisor
      .children(|mut children| {
        if let Some(callbacks) = A::with_children_callbacks() {
          children = children.with_callbacks(callbacks);
        }
        if let Some(dispatcher) = A::with_dispatcher() {
          children = children.with_dispatcher(dispatcher);
        }
        if let Some(distributor) = A::with_distributor() {
          children = children.with_distributor(distributor);
        }
        if let Some(interval) = A::with_heartbeat_tick() {
          children = children.with_heartbeat_tick(interval);
        }
        if let Some(name) = A::with_name() {
          children = children.with_name(name);
        }
        if let Some(redundancy) = A::with_redundancy() {
          children = children.with_redundancy(redundancy);
        }
        if let Some(resizer) = A::with_resizer() {
          children = children.with_resizer(resizer);
        }

        if let Some(callbacks) = self.children_callbacks {
          children = children.with_callbacks(callbacks);
        }
        if let Some(dispatcher) = self.dispatcher {
          children = children.with_dispatcher(dispatcher);
        }
        if let Some(distributor) = self.distributor {
          children = children.with_distributor(distributor);
        }
        if let Some(interval) = self.heartbeat_tick {
          children = children.with_heartbeat_tick(interval);
        }
        if let Some(name) = self.name {
          children = children.with_name(name);
        }
        if let Some(redundancy) = self.redundancy {
          children = children.with_redundancy(redundancy);
        }
        if let Some(resizer) = self.resizer {
          children = children.with_resizer(resizer);
        }

        A::executor(children, weak)
      })
      .map_err(|_| ActorError::UnableToInitChildren)?;

    Ok(A::with_supervisor(supervisor))
  }

  pub fn run(self, weak: &WeakState<A::State>) -> Result<A> {
    let supervisor = Bastion::supervisor(|mut sp| {
      if let Some(ref callbacks) = self.supervisor_callbacks {
        sp = sp.with_callbacks(callbacks.clone());
      }
      if let Some(ref restart_strategy) = self.restart_strategy {
        sp = sp.with_restart_strategy(restart_strategy.clone());
      }
      if let Some(ref strategy) = self.strategy {
        sp = sp.with_strategy(strategy.clone());
      }
      sp
    })
    .map_err(|_| ActorError::UnableToInitSupervisor)?;

    self.run_with_supervisor(supervisor, weak)
  }

  // For supervisor
  pub fn with_supervisor_callbacks(mut self, callbacks: Callbacks) -> Self {
    self.supervisor_callbacks = Some(callbacks);
    self
  }

  pub fn with_restart_strategy(mut self, restart_strategy: RestartStrategy) -> Self {
    self.restart_strategy = Some(restart_strategy);
    self
  }

  pub fn with_strategy(mut self, strategy: SupervisionStrategy) -> Self {
    self.strategy = Some(strategy);
    self
  }

  // For children
  pub fn with_children_callbacks(mut self, callbacks: Callbacks) -> Self {
    self.children_callbacks = Some(callbacks);
    self
  }

  pub fn with_dispatcher(mut self, dispatcher: Dispatcher) -> Self {
    self.dispatcher = Some(dispatcher);
    self
  }

  pub fn with_distributor(mut self, distributor: Distributor) -> Self {
    self.distributor = Some(distributor);
    self
  }

  pub fn with_heartbeat_tick(mut self, interval: Duration) -> Self {
    self.heartbeat_tick = Some(interval);
    self
  }

  pub fn with_name(mut self, name: impl Into<String>) -> Self {
    self.name = Some(name.into());
    self
  }

  pub fn with_redundancy(mut self, redundancy: usize) -> Self {
    self.redundancy = Some(redundancy);
    self
  }

  pub fn with_resizer(mut self, resizer: OptimalSizeExploringResizer) -> Self {
    self.resizer = Some(resizer);
    self
  }
}
