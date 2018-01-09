use identity::*;
use resources::server_resources::*;
use messaging::*;
use magnetic::TryPopError;
use std::time::{Duration,Instant};
struct ServerEngine {
    stub: ServerStub,
    portal_manager: PortalManager,
    subscription_manager: SubscriptionManager,
}

impl ServerEngine {
    pub fn new(stub: ServerStub) -> Self {
        ServerEngine {
            stub: stub,
            portal_manager: PortalManager::new(),
            subscription_manager: SubscriptionManager::new(),
        }
    }

    pub fn start() {
        loop {
            //todo sleep
            tick();
        }
    }

    pub fn tick(&mut self) {
        loop {
            match stub.try_pop() {
                TryPopError => return, //no messages waiting
                x => {

                },
            }
        }
    }

    pub fn handle_serverward_message(&mut self, msg: ServerwardMessage) {
        use ServerwardMessage::*;
        match msg {
            ReqDiff(diff) => {

            },
            Subscribe(lid) => {
                self.subscription_manager.set_player_sub(lid, pid, value)
            },
            Unsubscribe(lid) => {

            },
        }
    }
}