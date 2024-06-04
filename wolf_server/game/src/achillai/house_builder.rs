pub const BUILD_DISTANCE: f64 = 120.0;
const HOUSE_WIDTH: u32 = 10;


struct HouseBuilderFindLand {
}

impl HouseBuilderFindLand {
    fn step(&mut self, game: &mut Game, owner_id: GameObjectId) {
        let start_x = rand::thread_rng().gen_range(-100.. 100);
        let start_y = rand::thread_rng().gen_range(-100.. 100);
        let mut squares = Vec::new();
        //bottom
        for i in 0..HOUSE_WIDTH {
            squares.push(SquareCoords::new(start_x + i, start_y));
        }
        //left
        for i in 1..HOUSE_WIDTH{
            squares.push(SquareCoords::new(start_x, start_y + i));
        }
        //right
        for i in 1..HOUSE_WIDTH {
            squares.push(SquareCoords::new(start_x + HOUSE_WIDTH - 1, start_y + i));
        }
        //top
        for i in 1..(HOUSE_WIDTH - 1) {
            squares.push(SquareCoords::new(start_x + i, start_y + HOUSE_WIDTH - 1));
        }
        return ActionResult::ChangeTo(HouseBuilderBuildOnLand {
            current_square: squares.pop(),
            remaining_squares: squares,
        })
    }
}
struct HouseBuilderBuildOnLand {
    current_square: SquareCoords,
    remaining_squares: Vec<SquareCoords>,
}
impl HouseBuilderBuildOnLand {
    fn step(&mut self, game: &mut Game, owner_id: GameObjectId) {
        let current_position = owner_id.get_coords();
        if current_position.get_distance_to(&self.current_square.into()) < BUILD_DISTANCE {

        }
        else {
            owner_id.intend_move_to_point(game.movement_system.intend_move_system);
            return ActionResult::Continue
        }
        if self.remaining_squares.is_empty(){
            return ActionResult::Success
        }
        self.current_square = self.remaining_squares.pop()
    }
}
