///
/// The persistence module provides a simple interface for storing and
/// retrieving data from a persistent storage (LocalDisk or Canister)

pub trait PersistentStorage {
    fn serialize(&self) -> Vec<u8>;
    fn deserialize(serialized:  Vec<u8>) -> Self where Self: Sized;
}

// impl PersistentStore {

// }