use farmhash::FarmHasher;
use std::collections::{HashMap, HashSet};
use std::hash::BuildHasherDefault;

pub type FarmHashBuildHasher = BuildHasherDefault<FarmHasher>;

pub type FarmHashMap<K, V> = HashMap<K, V, FarmHashBuildHasher>;

pub type FarmHashSet<T> = HashSet<T, FarmHashBuildHasher>;
