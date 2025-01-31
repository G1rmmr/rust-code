use crate::game::EntityType;
use rand::Rng;

pub struct Map {
    pub size: usize,
    pub tiles: Vec<Vec<EntityType>>,
}

impl Map {
    pub fn new(size: usize) -> Self {
        let rng = rand::rng();
        let mut tiles = vec![vec![EntityType::Floor; size]; size];

        for y in 0..size {
            for x in 0..size {
                if y == 0 || y == size - 1 || x == 0 || x == size - 1 {
                    tiles[y][x] = EntityType::Wall;
                }
            }
        }
        Self { size, tiles }
    }

    pub fn generate(map: &Self) {
        let mut rng = rand::rng();
    }
}
