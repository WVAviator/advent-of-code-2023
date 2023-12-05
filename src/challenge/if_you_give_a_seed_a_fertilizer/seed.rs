use std::collections::HashMap;

use super::resource::Resource;

pub struct Seed {
    seed_id: u32,
    resources: HashMap<Resource, u32>,
}

impl Seed {
    pub fn new(seed_id: u32) -> Self {
        Seed {
            seed_id,
            resources: HashMap::from([(Resource::Seed, seed_id)]),
        }
    }

    pub fn get_resource(&self, resource: &Resource) -> u32 {
        return self
            .resources
            .get(resource)
            .expect("Could not find resource.")
            .clone();
    }

    pub fn add_resource(&mut self, resource: Resource, id: u32) {
        self.resources.insert(resource, id).unwrap();
    }
}
