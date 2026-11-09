const CHUNK_SIZE: isize = 16;

#[derive(Hash, PartialEq, Eq, Clone, Copy)]
pub struct BlockbucketBlockPos {
    pub x: isize,
    pub y: isize,
    pub z: isize,
}

#[derive(Hash, PartialEq, Eq, Clone, Copy)]
pub struct BlockbucketChunkPos {
    pub x: isize,
    pub z: isize,
}

impl BlockbucketBlockPos {
    /// Converts this position into the chunk it's in.
    pub fn chunk(&self) -> BlockbucketChunkPos {
        BlockbucketChunkPos {
            x: self.x.div_euclid(CHUNK_SIZE),
            z: self.z.div_euclid(CHUNK_SIZE),
        }
    }
}

impl BlockbucketChunkPos {
    /// Gets the center block of this chunk at y = 0.
    pub fn center(&self) -> BlockbucketBlockPos {
        BlockbucketBlockPos {
            x: self.x * CHUNK_SIZE + CHUNK_SIZE / 2,
            y: 0,
            z: self.z * CHUNK_SIZE + CHUNK_SIZE / 2,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{BlockbucketBlockPos, BlockbucketChunkPos};
    use test_case::test_case;

    #[test_case(0, 0, 0, 0; "origin")]
    #[test_case(15, 15, 0, 0; "upper edge of origin chunk")]
    #[test_case(16, 16, 1, 1; "positive chunk boundary")]
    #[test_case(-1, -1, -1, -1; "negative coordinate")]
    #[test_case(-16, -16, -1, -1; "negative chunk boundary")]
    #[test_case(-17, -17, -2, -2; "negative chunk boundary minus one")]
    #[test_case(31, -33, 1, -3; "mixed coordinates")]
    fn block_position_maps_to_chunk_with_flooring_for_negative_coordinates(
        x: isize,
        z: isize,
        chunk_x: isize,
        chunk_z: isize,
    ) {
        let pos = BlockbucketBlockPos { x, y: 42, z };
        let chunk = pos.chunk();

        assert_eq!((chunk.x, chunk.z), (chunk_x, chunk_z));
    }

    #[test_case(2, -3, 40, 0, -40; "positive x and negative z")]
    #[test_case(0, 0, 8, 0, 8; "origin chunk")]
    #[test_case(-1, -1, -8, 0, -8; "negative chunk")]
    fn chunk_center_is_the_middle_horizontal_block_at_y_zero(
        chunk_x: isize,
        chunk_z: isize,
        center_x: isize,
        center_y: isize,
        center_z: isize,
    ) {
        let chunk = BlockbucketChunkPos {
            x: chunk_x,
            z: chunk_z,
        };
        let center = chunk.center();

        assert_eq!(
            (center.x, center.y, center.z),
            (center_x, center_y, center_z)
        );
    }
}
