use std::{collections::HashMap, marker::PhantomData};

use crate::{
    items::{BlockbucketItemCategory, BlockbucketItemStack},
    storage::{Container, StorageRegion},
};

#[derive(Hash, PartialEq, Eq, Clone, Copy)]
enum StorageType {
    Bulk,
    Pickface,
    Pickup,
    BulkPickup,
    Dropoff,
}

/// Overarching struct that contains all information about the warehouse
pub struct BlockbucketWarehouse<
    Category: BlockbucketItemCategory,
    ItemStack: BlockbucketItemStack<Category>,
    C: Container<Category, ItemStack>,
    SR: StorageRegion<Category, ItemStack, C>,
> {
    storages: HashMap<StorageType, SR>,

    _cat: PhantomData<Category>,
    _itemstack: PhantomData<ItemStack>,
    _container: PhantomData<C>,
}
