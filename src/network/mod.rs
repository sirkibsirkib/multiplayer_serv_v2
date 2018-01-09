use messaging::{ServerStub, ClientStub, WrappedClientStub, WrappedServerStub};

pub fn start_client_messenger(stub: ServerStub) {

}

pub fn start_server_messenger(stub: ClientStub) {

}

pub fn start_single_coupler(clientside_serverstub: ServerStub, serverside_clientstub: WrappedClientStub) {
    let handle1 = thread::spawn(|| {
        //serverward
        match clientside_serverstub.pop() {
            Ok(msg) => {

            },
            Err(err) => {

            },
        }
    }).join();

    let handle2 = thread::spawn || {
        //clientward
        match serverside_clientstub.pop() {
            Ok(msg) => {
                
            },
            Err(err) => {

            },
        }
    }).join();
}