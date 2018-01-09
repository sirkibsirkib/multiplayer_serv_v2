
#[derive(Hash,Eq,PartialEq,Copy,Clone)]
pub struct LocationId(u32);
#[derive(Hash,Eq,PartialEq,Copy,Clone)]
pub struct PlayerId(u8);
impl PlayerId {
    pub fn new(raw_pid: u8) -> Self {
        assert!(raw_pid < 32);
        PlayerId(raw_pid)
    }
}

#[derive(Hash,Eq,PartialEq,Copy,Clone)]
pub struct ClientId(u64);
#[derive(Hash,Eq,PartialEq,Copy,Clone)]
pub struct EntityId(u64);


pub struct ClientIdBits {
    bits: u32,
}

impl ClientIdBits {
    pub fn new() -> ClientIdBits {
        ClientIdBits {bits: 0}
    }

    pub fn new_with_only(pid: PlayerId) -> ClientIdBits {
        let mut x = ClientIdBits::new();
        x.set(pid, true);
        x
    }

    pub fn set(&mut self, pid: PlayerId, value: bool) {
        assert!(pid.0 < 32);
        if value {
            self.bits = self.bits | (1 << pid.0);
        } else {
            self.bits = self.bits & ((0xff_ff_ff_ff) - (1 << pid.0));
        }
    }

    pub fn get(&self, pid: PlayerId) -> bool {
        self.bits & (1 << pid.0) != 0
    }

    pub fn no_subs(&self) -> bool {
        self.bits == 0
    }
}