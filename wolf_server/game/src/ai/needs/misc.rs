use super::*;

impl Needs {
    pub fn require_stockpile(&mut self) {
        self.needs
            .entry(NeedsKey::Stockpile)
            .or_insert(Need::IsTrue(false));
    }
    pub fn add_stockpile(&mut self) {
        self.needs.insert(NeedsKey::Stockpile, Need::IsTrue(true));
    }
    pub fn needs_stockpile(&self) -> bool {
        self.key_is_needed(NeedsKey::Stockpile)
    }
    pub fn require_house(&mut self) {
        self.needs
            .entry(NeedsKey::House)
            .or_insert(Need::IsTrue(false));
    }
    pub fn add_house(&mut self) {
        self.needs.insert(NeedsKey::House, Need::IsTrue(true));
    }
    pub fn needs_house(&self) -> bool {
        self.key_is_needed(NeedsKey::House)
    }
}
