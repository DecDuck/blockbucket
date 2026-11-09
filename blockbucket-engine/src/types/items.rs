use std::hash::Hash;

use serde_json::Value;

pub trait BlockbucketItemCategory: Clone + Copy + Hash + PartialEq + Eq {
    fn name<N: AsRef<str>>(&self) -> N;
}

pub trait BlockbucketItemStack<Cat: BlockbucketItemCategory> {
    fn name<N: AsRef<str>>(&self) -> N;
    fn count(&self) -> usize;
    fn metadata(&self) -> Value;
    fn category(&self) -> Cat;
}
