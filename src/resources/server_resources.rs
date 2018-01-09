use identity::*;
use identity::ClientIdBits;
use std::collections::HashMap;
use game_state::Point2;

//doens't ever need to be saved
pub struct SubscriptionManager {
    subs: HashMap<LocationId, ClientIdBits>,
}

impl SubscriptionManager {
    pub fn new() -> Self {
        SubscriptionManager {
            subs: HashMap::new(),
        }
    }

    pub fn set_player_sub(&mut self, lid: LocationId, pid: PlayerId, value: bool) {
        if self.subs.contains_key(&lid) {
            let mut rm = false;
            {
                let mut sub = self.subs.get_mut(&lid).unwrap();
                sub.set(pid, value);
                if !value && sub.no_subs() {
                    rm = true;                
                }
            }
            if rm {
                self.subs.remove(&lid);
            }
        } else {
            if value {
                self.subs.insert(lid, ClientIdBits::new_with_only(pid));
            } // else: nothing to do here
        }
    }
}


pub struct PortalManager {
    portals: HashMap<EntityId, Point2>
}
