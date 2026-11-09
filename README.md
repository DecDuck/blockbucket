# blockbucket

`blockbucket` is a way to bring modded Minecraft style inventory and item management to vanilla Minecraft, using botted clients.

## storages

`blockbucket` maintains the following storages within your warehouse:

- Bulk. Bulk storage is a massive array of containers that `blockbucket` fully manages. It's used to storage everything and the system keep tracks of where everything is.
- Pickface. A pickface is a selective slice of the bulk storage that `blockbucket` auto-maintains by keeping it fully (where possible), useful for quick access to common materials
- Pickup. A pickup is for specific item orders. When a user of the warehouse needs something specific, either crafted or just fetched, the workers will put it here.
- Bulk Pickup. When a larger order is put in (i.e. for a schematic or other material list), the workers will pack it into available shulker boxes and deposit them here.
- Dropoff. Workers will pick up anything here (including in shulkers), and sort it back into bulk storage.
