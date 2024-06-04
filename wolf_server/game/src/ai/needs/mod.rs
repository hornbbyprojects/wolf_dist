use wolf_hash_map::WolfHashMap;

mod resources;
pub use resources::*;

mod misc;
pub use misc::*;

mod needs_key;
pub use needs_key::*;

mod need;
pub use need::*;

#[derive(Debug)]
pub struct Needs {
    needs: WolfHashMap<NeedsKey, Need>,
}

impl Clone for Needs {
    fn clone(&self) -> Needs {
        let mut ret_needs = WolfHashMap::new();
        for (key, value) in self.needs.iter() {
            ret_needs.insert(key.clone(), value.clone());
        }
        Needs { needs: ret_needs }
    }
}

impl Needs {
    pub fn new() -> Needs {
        Needs {
            needs: WolfHashMap::new(),
        }
    }
    pub fn key_is_needed(&self, key: NeedsKey) -> bool {
        self.needs
            .get(&key)
            .map(|x| !x.is_satisfied())
            .unwrap_or(false)
    }
    pub fn is_satisfied(&self) -> bool {
        for (_key, value) in self.needs.iter() {
            if !value.is_satisfied() {
                return false;
            }
        }
        true
    }
    #[allow(dead_code)]
    pub fn debug_still_need(&self) {
        println!("the following needs are unsatisfied:");
        for (key, need) in self.needs.iter() {
            if !need.is_satisfied() {
                println!("Still need {:?} ({:?})", key, need);
            }
        }
    }
}
