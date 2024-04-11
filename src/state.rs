use std::sync::{Mutex, MutexGuard, OnceLock};

use crate::service::Songs;

pub(crate) struct ServerState {
    songs: Mutex<Box<dyn Songs + 'static>>,
}

static SERVER_STATE: OnceLock<ServerState> = OnceLock::new();

pub(crate) fn init_server_state(state: ServerState) -> Result<(), String> {
    if SERVER_STATE.get().is_some() {
        return Err("Server state was already initialized!".to_string());
    }
    SERVER_STATE.get_or_init(|| state);
    Ok(())
}

pub(crate) fn get_server_state_or_fail() -> &'static ServerState {
    match SERVER_STATE.get() {
        Some(value) => value,
        None => panic!("Server state was not initialized"),
    }
}

impl ServerState {
    pub(crate) fn new<S: Songs + 'static>(songs: S) -> Self {
        Self {
            songs: Mutex::new(Box::new(songs)),
        }
    }

    pub(crate) fn songs(&self) -> MutexGuard<Box<dyn Songs>> {
        ServerState::lock_clearing(&self.songs, "Songs")
    }

    fn lock_clearing<'a, T>(mutex: &'a Mutex<T>, name: &'static str) -> MutexGuard<'a, T> {
        mutex.lock().unwrap_or_else(|err| {
            log::info!("Clearing poison for {}", name);
            mutex.clear_poison();
            err.into_inner()
        })
    }
}
