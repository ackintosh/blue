enum State {
    Init,
    Standby,
    ConnectedToNetwork,
    ShuttingDown,
}

struct Core {
    state: State,
}

impl Core {
    fn new() -> Self {
        println!("Initializing core node...");
        Self { state: State::Init }
    }

    fn start(&mut self) {
        self.state = State::Standby;
    }

    fn join_network(&mut self) {
        self.state = State::ConnectedToNetwork;
    }

    fn shutdown(&mut self) {
        self.state = State::ShuttingDown;
        println!("Shutdown core node...");
    }

    fn get_state(&self) -> State {
        *self.state
    }
}
