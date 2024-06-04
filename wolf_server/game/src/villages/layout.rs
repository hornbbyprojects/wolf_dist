use crate::terrain::TerrainSpriteComponent;

use super::*;
/*
Basic layout of villages.
Starting idea: Randomly place buildings, with roads between them
*/

use coords::SquareCoords;

use super::create_tavern;

pub fn create_road_square(game: &mut Game, coords: SquareCoords) {
    let owner_id = GameObject::create_game(game, coords.center_pixel());
    TerrainSpriteComponent::add_to(game, owner_id, coords, FLOOR_SPRITE);
}
pub fn make_road(game: &mut Game, start: SquareCoords, end: SquareCoords) {
    let mut current = start;
    loop {
        create_road_square(game, current);
        let dx = end.get_x() - current.get_x();
        let dy = end.get_y() - current.get_y();
        if dx == 0 && dy == 0 {
            return;
        }
        if dx.abs() > dy.abs() {
            current = current.translate(dx.signum(), 0);
        } else {
            current = current.translate(0, dy.signum());
        }
    }
}
pub fn place_village(game: &mut Game) {
    let mut tavern_coords = Vec::new();
    for _ in 0..3 {
        let sx = rand::thread_rng().gen_range(-100..100);
        let sy = rand::thread_rng().gen_range(-100..100);
        let coords = SquareCoords::new(Plane(0), sx, sy);
        tavern_coords.push(coords);
    }
    for x in 0..tavern_coords.len() - 1 {
        for y in x + 1..tavern_coords.len() {
            make_road(game, tavern_coords[x], tavern_coords[y]);
        }
    }
    for coords in tavern_coords {
        create_tavern(game, coords);
    }
}
