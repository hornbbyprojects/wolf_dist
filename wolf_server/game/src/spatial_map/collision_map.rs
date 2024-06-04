use crate::game::*;
use crate::spatial_map::*;
use fixed::traits::ToFixed;

#[derive(Clone)]
pub struct HitBox {
    pub coords: PixelCoords, //coords of center
    pub width: PixelNum,     //from center to edge, technically half width
    pub height: PixelNum,
}
impl std::fmt::Debug for HitBox {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&format!(
            "hitbox[{},{}:{},{}]",
            self.coords.get_x().0,
            self.coords.get_y().0,
            self.width.0,
            self.height.0
        ))?;
        Ok(())
    }
}
impl HitBox {
    pub fn default() -> Self {
        Self::new_at_zero(10, 10)
    }
    pub fn new_at_zero<T: ToFixed>(width: T, height: T) -> HitBox {
        Self::new(PixelCoords::new_at_zero(), width, height)
    }
    pub fn new<T: ToFixed>(coords: PixelCoords, width: T, height: T) -> HitBox {
        HitBox {
            coords,
            width: PixelNum::from_num(width),
            height: PixelNum::from_num(height),
        }
    }
    pub fn collides_with(&self, other: &HitBox) -> bool {
        let x = self.coords.get_x();
        let y = self.coords.get_y();
        let other_x = other.coords.get_x();
        let other_y = other.coords.get_y();
        //two squares collide iff both their x range and y range intercept.
        //overlap of [x1, y1] and [x2, y2] is [x1.max(x2), y1.min(y2)]
        //so to check for overlap we need to check if x1.max(x2) <= y1.min(y2)
        let max_leftmost = (x - self.width).max(other_x - other.width);
        let min_rightmost = (x + self.width).min(other_x + other.width);
        if max_leftmost > min_rightmost {
            return false;
        }
        let max_bottommost = (y - self.height).max(other_y - other.height);
        let min_topmost = (y + self.height).min(other_y + other.height);
        if max_bottommost > min_topmost {
            return false;
        }
        true
    }
    pub fn rotate(&self, rotation: Angle) -> Self {
        //TODO: this method stretches the hitbox, bad for beams etc
        let width: f64 = self.width.0.to_num::<f64>() * rotation.cos()
            + self.height.0.to_num::<f64>() * rotation.sin();
        let height: f64 = self.width.0.to_num::<f64>() * rotation.sin()
            + self.height.0.to_num::<f64>() * rotation.cos();
        HitBox::new(self.coords, width.abs(), height.abs())
    }
    pub fn translate(&self, offset: PixelCoords) -> Self {
        HitBox {
            coords: self.coords.set_plane(offset.get_plane()) + offset,
            width: self.width,
            height: self.height,
        }
    }
    pub fn move_to(&self, location: PixelCoords) -> Self {
        let mut ret = self.clone();
        ret.coords = location;
        ret
    }
    pub fn get_overlapping_squares(&self) -> WolfHashSet<SquareCoords> {
        let bottom_left = self.coords.translate_fixed(-self.width, -self.height);
        let top_right = self.coords.translate_fixed(self.width, self.height);
        square_of_coords::<i64, SquareCoords>(bottom_left.into(), top_right.into())
    }
}
pub trait HitBoxed {
    fn get_hit_box(&self, game: &Game) -> HitBox;
}
impl<T: HitBoxed> SpatialBox for T {
    fn get_coords(&self, game: &Game) -> SquareCoords {
        self.get_hit_box(game).coords.into()
    }
    fn get_box_dimensions(&self, game: &Game) -> (u32, u32) {
        let hit_box = self.get_hit_box(game);
        let width = (hit_box.width.0.to_num::<f64>() / SQUARE_SIZE_PIXELS as f64).ceil() as u32;
        let height = (hit_box.height.0.to_num::<f64>() / SQUARE_SIZE_PIXELS as f64).ceil() as u32;
        (width, height)
    }
}

pub struct CollisionMap<ItemType>(pub SpatialMap<ItemType>);

impl<ItemType: HitBoxed + Eq + Hash + Clone> CollisionMap<ItemType> {
    pub fn new() -> Self {
        CollisionMap(SpatialMap::<ItemType>::new())
    }
    pub fn add(&mut self, game: &Game, new_item: ItemType) {
        self.0.add(game, new_item);
    }
    pub fn remove(&mut self, item: ItemType) {
        self.0.remove(item);
    }
    pub fn move_item(&mut self, game: &Game, item: ItemType) {
        self.0.move_item(game, item);
    }
    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }
}

impl<ItemType: HitBoxed + Eq + Hash + Clone> CollisionMap<ItemType> {
    pub fn get_colliding_game(&self, game: &Game, hit_box: HitBox) -> Vec<ItemType> {
        let candidates = self.0.get_within_box(
            hit_box.coords.into(),
            hit_box.width.0.to_num::<i64>(),
            hit_box.height.0.to_num::<i64>(),
        );
        candidates
            .into_iter()
            .filter(|candidate| candidate.get_hit_box(game).collides_with(&hit_box))
            .collect::<Vec<ItemType>>()
    }
}
