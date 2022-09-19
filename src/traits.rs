use std::time::Duration;

use async_trait::async_trait;
use bastion::prelude::*;

use crate::{actor::Actor, state::WeakState};

#[async_trait]
pub trait TActor {
  type State: Sized;

  fn default() -> Actor<Self>
  where
    Self: Sized;

  fn with_supervisor(supervisor: SupervisorRef) -> Self;

  fn executor(children: Children, weak: &WeakState<Self::State>) -> Children;

  // For supervisor
  fn with_supervisor_callbacks() -> Option<Callbacks>;

  fn with_restart_strategy() -> Option<RestartStrategy>;

  fn with_strategy() -> Option<SupervisionStrategy>;

  // For children
  fn with_children_callbacks() -> Option<Callbacks>;

  fn with_dispatcher() -> Option<Dispatcher>;

  fn with_distributor() -> Option<Distributor>;

  fn with_heartbeat_tick() -> Option<Duration>;

  fn with_name() -> Option<String>;

  fn with_redundancy() -> Option<usize>;

  fn with_resizer() -> Option<OptimalSizeExploringResizer>;
}

#[async_trait]
pub trait TState {}
