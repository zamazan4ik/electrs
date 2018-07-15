use bitcoin::util::hash::Sha256dHash;
use std::sync::{Arc, Mutex};

use {daemon, index, signal::Waiter, store};

use errors::*;

pub struct App {
    store: store::DBStore,
    index: index::Index,
    daemon: daemon::Daemon,
    tip: Mutex<Sha256dHash>,
}

impl App {
    pub fn new(
        store: store::DBStore,
        index: index::Index,
        daemon: daemon::Daemon,
    ) -> Result<Arc<App>> {
        Ok(Arc::new(App {
            store,
            index,
            daemon: daemon.reconnect()?,
            tip: Mutex::new(Sha256dHash::default()),
        }))
    }

    pub fn write_store(&self) -> &store::WriteStore {
        &self.store
    }
    pub fn read_store(&self) -> &store::ReadStore {
        &self.store
    }
    pub fn index(&self) -> &index::Index {
        &self.index
    }
    pub fn daemon(&self) -> &daemon::Daemon {
        &self.daemon
    }

    pub fn update(&self, signal: &Waiter) -> Result<bool> {
        let mut tip = self.tip.lock().expect("failed to lock tip");
        let new_block = *tip != self.daemon().getbestblockhash()?;
        if new_block {
            *tip = self.index().update(self.write_store(), &signal)?;
        }
        Ok(new_block)
    }
}
