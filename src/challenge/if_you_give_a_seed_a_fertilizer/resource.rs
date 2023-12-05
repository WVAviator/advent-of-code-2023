use std::fmt::Display;

#[derive(Eq, PartialEq, Hash, Debug, Clone, Copy)]
pub enum Resource {
    Seed,
    Soil,
    Fertilizer,
    Water,
    Light,
    Temperature,
    Humidity,
    Location,
}

impl Display for Resource {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Resource::Seed => "seed",
                Resource::Soil => "soil",
                Resource::Fertilizer => "fertilizer",
                Resource::Water => "water",
                Resource::Light => "light",
                Resource::Temperature => "temperature",
                Resource::Humidity => "humidity",
                Resource::Location => "location",
            }
        )
    }
}

impl Resource {
    pub fn from(resource_str: &str) -> Self {
        match resource_str {
            "seed" => Resource::Seed,
            "soil" => Resource::Soil,
            "fertilizer" => Resource::Fertilizer,
            "water" => Resource::Water,
            "light" => Resource::Light,
            "temperature" => Resource::Temperature,
            "humidity" => Resource::Humidity,
            "location" => Resource::Location,
            _ => panic!("Attempted to parse an invalid resource: {}", resource_str),
        }
    }
}
