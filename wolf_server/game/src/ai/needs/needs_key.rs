#[derive(Eq, PartialEq, Clone, Copy, Hash, Debug)]
pub enum NeedsKey {
    Resources,
    TownResources,
    House,
    Stockpile,
}
