use crate::connection_manager::ConnectionManager;

enum State {
    Init,
    Standby,
    ConnectedToNetwork,
    ShuttingDown,
}

pub struct Core {
    state: State,
    cm: ConnectionManager,
}

impl Core {
    pub fn new(host: String, port: String) -> Self {
        println!("Initializing core node...");
        Self {
            state: State::Init,
            cm: ConnectionManager::new(host.clone(), port.clone()),
        }
    }

    pub fn start(&mut self) {
        self.state = State::Standby;
        self.cm.start();
    }

    fn join_network(&mut self) {
        self.state = State::ConnectedToNetwork;
    }

    fn shutdown(&mut self) {
        self.state = State::ShuttingDown;
        println!("Shutdown core node...");
    }

    fn get_state(&self) -> &State {
        &self.state
    }
}
