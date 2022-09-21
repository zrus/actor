use std::{
  ops::Deref,
  sync::{Arc, Weak},
};

use tokio::sync::RwLock;

type Inner<S> = Arc<RwLock<S>>;
type InnerWeak<S> = Weak<RwLock<S>>;

#[derive(Debug)]
pub struct State<S>(Inner<S>);

impl<S> Default for State<S>
where
  S: Default,
{
  fn default() -> Self {
    Self(Arc::new(RwLock::new(S::default())))
  }
}

impl<S> Deref for State<S> {
  type Target = Inner<S>;

  fn deref(&self) -> &Self::Target {
    &self.0
  }
}

impl<S> State<S> {
  pub fn downgrade(&self) -> WeakState<S> {
    WeakState(Arc::downgrade(self))
  }

  pub fn upgrade(weak: WeakState<S>) -> Self {
    match weak.0.upgrade() {
      Some(state) => Self(state),
      None => panic!("Upgrade state failed!"),
    }
  }
}

#[derive(Debug)]
pub struct WeakState<S>(Weak<RwLock<S>>);

impl<S> Deref for WeakState<S> {
  type Target = InnerWeak<S>;

  fn deref(&self) -> &Self::Target {
    &self.0
  }
}

impl<S> Clone for WeakState<S> {
  fn clone(&self) -> Self {
    Self(self.0.clone())
  }
}
