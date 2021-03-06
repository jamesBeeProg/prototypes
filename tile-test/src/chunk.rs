use crate::assets::{Assets, FlatWorldGenerator, NoiseWorldGenerator, WorldGenerator};
use anyhow::Result;
use cgmath::Vector2;
use noise::{NoiseFn, Perlin, Seedable};
use rand::{prelude::SliceRandom, thread_rng};
use std::iter;

#[derive(Clone)]
pub struct Chunk {
    tiles: [u8; Self::SIZE * Self::SIZE],
    // DEBUG: Only public for debug controls
    pub position: Vector2<isize>,
}

impl Chunk {
    pub const SIZE: usize = 32;

    pub fn new(position: Vector2<isize>) -> Self {
        Self {
            position,
            tiles: [0; Self::SIZE * Self::SIZE],
        }
    }

    pub fn tiles(&self) -> &[u8] {
        &self.tiles
    }

    pub fn set_tile(&mut self, position: Vector2<usize>, name: &str, assets: &Assets) {
        let mut rng = thread_rng();

        // Retrieve the data for the tile
        let tile = assets.tile_data.tiles.get(name).unwrap();

        // Y index, starts from wrong corner so do some math
        let index = Chunk::SIZE - 1 - position.y;
        // Account for X index
        let index = index * Chunk::SIZE + position.x;

        // Pick a random sprite
        self.tiles[index] = *tile.sprites.choose(&mut rng).unwrap();
    }

    pub fn generate(&mut self, assets: &Assets) -> Result<()> {
        match &assets.world_data {
            WorldGenerator::Flat(gen) => self.generate_flat(gen, assets),
            WorldGenerator::Noise(gen) => self.generate_noise(gen, assets),
        }
    }

    fn generate_flat(&mut self, gen: &FlatWorldGenerator, assets: &Assets) -> Result<()> {
        let mut rng = thread_rng();

        // Retrieve the data for the tile
        let flat_tile = assets.tile_data.tiles.get(&gen.tile).unwrap();

        // For every tile
        for tile in self.tiles.iter_mut() {
            // Pick a random sprite
            *tile = *flat_tile.sprites.choose(&mut rng).unwrap();
        }

        Ok(())
    }

    fn generate_noise(&mut self, gen: &NoiseWorldGenerator, assets: &Assets) -> Result<()> {
        let mut rng = thread_rng();

        // Create noise from seed
        let noise = Perlin::new().set_seed(gen.seed);

        // Map each tile id to their data
        let tiles = gen
            .tiles
            .iter()
            .flat_map(|it| {
                let tile = assets.tile_data.tiles.get(&it.name);
                iter::repeat(tile).take(it.bias)
            })
            .collect::<Option<Vec<_>>>()
            .unwrap();

        // For every tile
        for (index, tile) in self.tiles.iter_mut().enumerate() {
            // Map chunk index into (X, Y) pair
            let index = Vector2::new(index % Self::SIZE, index / Self::SIZE);

            // Relate to global grid
            let index = index.cast().unwrap() + self.position * Self::SIZE as isize;

            // Scale by world gen settings
            let index = index.cast().unwrap() / gen.scale;

            // Get noise value for position
            let index: [f64; 2] = index.into();
            let output = noise.get(index);

            // Map from `-1.0..1.0` to `0..tile.len()`
            let output = output * 0.5 + 0.5;
            let output = output * tiles.len() as f64;
            let output = output.trunc() as usize;
            let output = output.min(tiles.len() - 1);

            // Retrieve one of the tile's sprites
            let output = tiles[output].sprites.choose(&mut rng).unwrap();

            // Update buffer with new sprite id
            *tile = *output;
        }

        Ok(())
    }
}

impl std::fmt::Debug for Chunk {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for (i, tiles) in self.tiles.chunks(Chunk::SIZE).enumerate() {
            // Write current index as sidebar
            write!(f, "{}  ", i)?;

            // Write all sprite ids in row
            for tile in tiles {
                write!(f, "{}", tile)?;
            }

            // Newline
            writeln!(f)?;
        }

        // Print position last so don't have to scroll up
        writeln!(f, "({}, {})", self.position.x, self.position.y)
    }
}
