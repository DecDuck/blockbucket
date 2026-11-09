use std::collections::HashMap;

use crate::{
    coords::{BlockbucketBlockPos, BlockbucketChunkPos},
    items::{BlockbucketItemCategory, BlockbucketItemStack},
};

pub struct Capacity {
    pub filled: usize,
    pub total: usize,
}

/// A single container in the world that has a completely separate inventory to other containers.
pub trait Container<Cat: BlockbucketItemCategory, ItemStack: BlockbucketItemStack<Cat>> {
    /// Called on every container while creating them, to see if they share a neighbour. If so, we skip creating a container for its neighbour.
    fn shares_neighbour(&self) -> Option<BlockbucketBlockPos>;

    fn update_contents(
        &mut self,
        contents: Vec<ItemStack>,
    ) -> impl Future<Output = Result<(), anyhow::Error>>;
}

pub trait StorageRegion<
    Cat: BlockbucketItemCategory,
    ItemStack: BlockbucketItemStack<Cat>,
    C: Container<Cat, ItemStack>,
>
{
    /// Syncs the cached storage with the database (not the world)
    /// Returns Result<(), anyhow::Error> because blockbucket doesn't have to handle database errors
    fn sync(&mut self) -> impl Future<Output = Result<(), anyhow::Error>>;

    /// Updates a particular chunks' containers. Should update cache and database.
    fn update_chunk_containers(
        &mut self,
        chunk: BlockbucketChunkPos,
        containers: HashMap<BlockbucketBlockPos, C>,
    ) -> impl Future<Output = Result<(), anyhow::Error>>;

    /// Fetches a mutable container at a position
    fn container_view(&mut self, container: BlockbucketBlockPos) -> Option<&mut C>;

    /// How many slots are filled out of total
    fn slot_capacity(&self) -> Capacity;

    /// How much theoretical item storage there is
    fn item_capacity(&self) -> Capacity;
}
