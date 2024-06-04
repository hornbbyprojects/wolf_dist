use std::collections::BTreeMap;

#[derive(PartialEq, Eq, Hash, Debug, Clone, Copy, PartialOrd, Ord)]
pub struct ResourceAmount(pub i32);

impl std::ops::Add for ResourceAmount {
    type Output = Self;
    fn add(self, rhs: Self) -> Self {
        ResourceAmount(self.0 + rhs.0)
    }
}
impl std::ops::Sub for ResourceAmount {
    type Output = Self;
    fn sub(self, rhs: Self) -> Self {
        ResourceAmount(self.0 - rhs.0)
    }
}

#[derive(PartialEq, Eq, Hash, Debug, Copy, Clone, PartialOrd, Ord)]
pub enum ResourceType {
    Wood,
    Food,
}

#[derive(Clone, Debug, Hash, PartialEq, Eq)]
pub struct Resources {
    //BTreeMap used so we can hash resources
    pub resource_amounts: BTreeMap<ResourceType, ResourceAmount>,
}

impl std::ops::Add for Resources {
    type Output = Self;
    fn add(mut self, rhs: Self) -> Self::Output {
        self += &rhs;
        self
    }
}

impl std::ops::SubAssign<&Resources> for Resources {
    fn sub_assign(&mut self, rhs: &Resources) {
        for (key, value) in rhs.resource_amounts.iter() {
            let our_value = self
                .resource_amounts
                .get(key)
                .map(|x| *x)
                .unwrap_or(ResourceAmount(0));
            let total = our_value - *value;
            self.set_resource_amount(*key, total);
        }
    }
}
impl std::ops::AddAssign<&Resources> for Resources {
    fn add_assign(&mut self, rhs: &Resources) {
        for (key, value) in rhs.resource_amounts.iter() {
            let our_value = self
                .resource_amounts
                .get(key)
                .map(|x| *x)
                .unwrap_or(ResourceAmount(0));
            let total = our_value + *value;
            self.set_resource_amount(*key, total);
        }
    }
}

impl Resources {
    pub fn new() -> Self {
        Resources {
            resource_amounts: BTreeMap::new(),
        }
    }
    pub fn is_empty(&self) -> bool {
        self.resource_amounts.is_empty()
    }
    pub fn contains(&self, other: &Self) -> bool {
        for (key, value) in other.resource_amounts.iter() {
            if let Some(our_value) = self.resource_amounts.get(key) {
                if our_value < value {
                    return false;
                }
            } else {
                return false;
            }
        }
        true
    }
    pub fn get_resource_amount(&self, resource_type: ResourceType) -> ResourceAmount {
        self.resource_amounts
            .get(&resource_type)
            .map(|x| *x)
            .unwrap_or(ResourceAmount(0))
    }
    pub fn set_resource_amount(&mut self, resource_type: ResourceType, amount: ResourceAmount) {
        if amount == ResourceAmount(0) {
            self.resource_amounts.remove(&resource_type);
        } else {
            self.resource_amounts.insert(resource_type, amount);
        }
    }
    pub fn wood(wood_amount: ResourceAmount) -> Self {
        let mut resource_amounts = BTreeMap::new();
        resource_amounts.insert(ResourceType::Wood, wood_amount);
        Resources { resource_amounts }
    }
    pub fn food(food_amount: ResourceAmount) -> Self {
        let mut resource_amounts = BTreeMap::new();
        resource_amounts.insert(ResourceType::Food, food_amount);
        Resources { resource_amounts }
    }
    pub fn take_all_from(&mut self, other: &mut Self) {
        let other_resources = std::mem::replace(&mut other.resource_amounts, BTreeMap::new());
        for (key, their_value) in other_resources {
            let our_value = self
                .resource_amounts
                .get(&key)
                .map(|x| *x)
                .unwrap_or(ResourceAmount(0));
            let total = our_value + their_value;
            self.set_resource_amount(key, total);
        }
    }
    pub fn spend(&mut self, amount_to_spend: &mut Resources) {
        let mut to_spend = Vec::new();
        for (resource_type, resource_amount) in amount_to_spend.resource_amounts.iter() {
            to_spend.push((resource_type.clone(), resource_amount.clone()));
        }
        for (resource_type, their_amount) in to_spend {
            let our_amount = match self.resource_amounts.get(&resource_type) {
                Some(our_amount) => our_amount.clone(),
                None => continue,
            };
            if our_amount >= their_amount {
                amount_to_spend.resource_amounts.remove(&resource_type);
                let total = our_amount - their_amount;
                self.set_resource_amount(resource_type, total);
            } else {
                self.resource_amounts.remove(&resource_type);
                let total = their_amount - our_amount;
                amount_to_spend.set_resource_amount(resource_type, total);
            }
        }
    }
    //efficiency is what % of self divisor will satisfy, with all resource_types unweighted by amount
    //count is the number of times this efficiency will apply
    //e.g: if a resource type will give 50% food and 25% wood, then efficiency is 37.5% and count is 2.
    pub fn get_efficiency_and_count(&self, divisor: &Resources) -> (f64, usize) {
        let mut total_efficiency = 0.0;
        let mut max_count = 0;
        for (resource_type, resource_amount) in self.resource_amounts.iter() {
            let other_resource_amount = divisor.get_resource_amount(*resource_type);
            if other_resource_amount == ResourceAmount(0) {
                continue;
            }
            let count = resource_amount.0 as f64 / other_resource_amount.0 as f64;
            let count_ceil = count.ceil() as usize; //guaranteed to be at least 1, as resource_amount >= 0
            max_count = max_count.max(count_ceil);
            let type_efficiency = (1.0 / count).min(1.0);
            total_efficiency += type_efficiency;
        }
        let efficiency = total_efficiency / self.resource_amounts.len() as f64;
        (efficiency, max_count)
    }
}
