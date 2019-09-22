use crate::observable::{from_future::DEFAULT_RUNTIME, interval::SpawnHandle};
use crate::prelude::*;
use futures::prelude::*;
use futures::task::SpawnExt;

pub(crate) fn thread_pool_schedule<T: Send + Sync + 'static>(
  task: impl FnOnce(SharedSubscription, Option<T>) + Send + 'static,
  state: Option<T>,
) -> SharedSubscription {
  let mut subscription = SharedSubscription::default();
  let c_proxy = subscription.clone();
  let f = future::lazy(move |_| task(c_proxy, state));
  let handle = DEFAULT_RUNTIME
    .lock()
    .unwrap()
    .spawn_with_handle(f)
    .expect("spawn task to thread pool failed.");

  subscription.add(Box::new(SpawnHandle(Some(handle))));
  subscription
}