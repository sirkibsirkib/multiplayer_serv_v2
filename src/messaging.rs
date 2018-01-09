use identity::*;
use game_state::DiscretePoint2;
use magnetic::{Consumer,Producer,TryPopError,TryPushError,PopError,PushError};


use magnetic::spsc::{spsc_queue,SPSCConsumer,SPSCProducer};
use magnetic::buffer::dynamic::DynamicBuffer;

struct Authenticator {
    
}

impl Authenticator {
    pub fn try_authenticate(username: String, password: String) -> Result<PlayerId, AuthenticatorError> {
        unimplemented!()
    }
}

enum AuthenticatorError {
    UnknownUsername,
    PasswordMismatch,
    AlreadyLoggedIn,
}

struct WrappedServerwardMessage {
    sender: PlayerId,
    msg: ServerwardMessage,
}

struct WrappedClientwardMessage {
    destintations: ClientIdBits,
    msg: ClientwardMessage,
}



pub enum ServerwardMessage {
    ReqDiff(Diff),
    Subscribe(LocationId),
    Unsubscribe(LocationId),
    //TODO
}

pub enum ClientwardMessage {
    //TODO
}

enum Diff {
    MoveEntityLocally(EntityId, DiscretePoint2),
    MoveEntityGlobally(EntityId, LocationId, DiscretePoint2),
}

pub fn message_channel() -> (ClientStub, ServerStub) {
    let (serverward_p, serverward_c) = spsc_queue(DynamicBuffer::new(128).unwrap());
    let (clientward_p, clientward_c) = spsc_queue(DynamicBuffer::new(128).unwrap());
    let c_stub = ClientStub {
        input:  serverward_p,
        output: clientward_c,
    };
    let p_stub = ServerStub {
        input:  clientward_p,
        output: serverward_c,
    };
    (c_stub, p_stub)
}

pub fn wrapped_message_channel() {

}

// client side, representing client side
pub struct ClientStub {
    input:  SPSCProducer<ServerwardMessage, DynamicBuffer<ServerwardMessage>>,
    output: SPSCConsumer<ClientwardMessage, DynamicBuffer<ClientwardMessage>>,
}

// server side, representing server side
pub struct WrappedClientStub {
    input:  SPSCProducer<WrappedServerwardMessage, DynamicBuffer<WrappedServerwardMessage>>,
    output: SPSCConsumer<WrappedClientwardMessage, DynamicBuffer<WrappedClientwardMessage>>,
}

//client side, representing server side
pub struct ServerStub {
    input:  SPSCProducer<ClientwardMessage, DynamicBuffer<ClientwardMessage>>,
    output: SPSCConsumer<ServerwardMessage, DynamicBuffer<ServerwardMessage>>,
}

// server side, representing server side
pub struct WrappedServerStub {
    input:  SPSCProducer<WrappedClientwardMessage, DynamicBuffer<WrappedClientwardMessage>>,
    output: SPSCConsumer<WrappedServerwardMessage, DynamicBuffer<WrappedServerwardMessage>>,
}

impl ClientStub {
    pub fn pop(&self) -> Result<ClientwardMessage,PopError> {
        self.output.pop()
    }
    pub fn try_pop(&self) -> Result<ClientwardMessage,TryPopError> {
        self.output.try_pop()
    }
    pub fn try_push(&self, msg: ServerwardMessage) -> Result<(), TryPushError<ServerwardMessage>> {
        self.input.try_push(msg)
    }
}


impl WrappedClientStub {
    pub fn try_pop(&self) -> Result<WrappedClientwardMessage,TryPopError> {
        self.output.try_pop()
    }
    pub fn try_push(&self, msg: WrappedServerwardMessage) -> Result<(), TryPushError<WrappedServerwardMessage>> {
        self.input.try_push(msg)
    }
}

impl ServerStub {
    pub fn try_pop(&self) -> Result<ServerwardMessage,TryPopError> {
        self.output.try_pop()
    }
    pub fn try_push(&self, msg: ClientwardMessage) -> Result<(), TryPushError<ClientwardMessage>> {
        self.input.try_push(msg)
    }
}


impl WrappedServerStub {
    pub fn try_pop(&mut self) -> Result<WrappedServerwardMessage,TryPopError> {
        self.output.try_pop()
    }
    pub fn try_push(&mut self, msg: WrappedClientwardMessage) -> Result<(), TryPushError<WrappedClientwardMessage>> {
        self.input.try_push(msg)
    }
}

trait Serverside {
    fn push_wrapped(&self, wrapped: WrappedClientwardMessage);
    fn try_pop_wrapped(&self) -> Result<WrappedServerwardMessage, TryPopError>;
}

trait Clientside {
    fn push(&self, msg: ServerwardMessage);
    fn try_pop(&self) -> Result<ClientwardMessage, TryPopError>;
}