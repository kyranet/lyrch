use serenity::framework::standard::StandardFramework;
use serenity::framework::Framework;
use serenity::model::prelude::Message;
use serenity::prelude::{Context, Mutex, TypeMapKey};
use std::sync::Arc;
use threadpool::ThreadPool;

pub struct LyrchFramework(pub Arc<Mutex<StandardFramework>>);

impl LyrchFramework {
    pub fn new(framework: StandardFramework) -> Self {
        Self(Arc::new(Mutex::new(framework)))
    }
}

impl Framework for LyrchFramework {
    #[inline]
    fn dispatch(&mut self, context: Context, message: Message, tp: &ThreadPool) {
        self.0.lock().dispatch(context, message, tp)
    }
}

impl From<StandardFramework> for LyrchFramework {
    fn from(framework: StandardFramework) -> Self {
        LyrchFramework::new(framework)
    }
}

impl Clone for LyrchFramework {
    fn clone(&self) -> Self {
        LyrchFramework(Arc::clone(&self.0))
    }
}

impl TypeMapKey for LyrchFramework {
    type Value = LyrchFramework;
}
