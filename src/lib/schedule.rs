// We will use this crate as event dispatcher.
use hey_listen::sync::ParallelDispatcher as Dispatcher;
// And this crate to schedule our tasks.
use serenity::model::prelude::*;
use serenity::prelude::*;
use std::{
    env,
    hash::{Hash, Hasher},
    sync::Arc,
};
use white_rabbit::Scheduler;

pub fn create_scheduler() -> Arc<RwLock<Scheduler>> {
    let scheduler = Scheduler::new(4);
    Arc::new(RwLock::new(scheduler))
}

pub fn create_dispatcher() -> Arc<RwLock<Dispatcher<DispatchEvent>>> {
    let mut dispatcher = Dispatcher::default();

    let thread_amount = env::var("SCHEDULE_THREADS")
        .expect("Expected the `SCHEDULE_THREADS` variable to be set in `.env`.")
        .parse::<usize>()
        .unwrap();
    dispatcher
        .num_threads(thread_amount)
        .expect("Could not construct threadpool");
    Arc::new(RwLock::new(dispatcher))
}

#[derive(Clone)]
pub enum DispatchEvent {
    // TODO(kyranet): Use this
    #[allow(dead_code)]
    ReactEvent(MessageId, UserId),
}

// We need to implement equality for our enum.
// One could test variants only. In this case, we want to know who reacted
// on which message.
impl PartialEq for DispatchEvent {
    fn eq(&self, other: &DispatchEvent) -> bool {
        match (self, other) {
            (
                DispatchEvent::ReactEvent(self_message_id, self_user_id),
                DispatchEvent::ReactEvent(other_message_id, other_user_id),
            ) => self_message_id == other_message_id && self_user_id == other_user_id,
        }
    }
}

impl Eq for DispatchEvent {}

impl Hash for DispatchEvent {
    fn hash<H: Hasher>(&self, state: &mut H) {
        match self {
            DispatchEvent::ReactEvent(msg_id, user_id) => {
                msg_id.hash(state);
                user_id.hash(state);
            }
        }
    }
}

pub struct DispatcherKey;
impl TypeMapKey for DispatcherKey {
    type Value = Arc<RwLock<Dispatcher<DispatchEvent>>>;
}

pub struct SchedulerKey;
impl TypeMapKey for SchedulerKey {
    type Value = Arc<RwLock<Scheduler>>;
}
