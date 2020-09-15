pub enum Background {
    Empty,
    GravityRift,
    Nebula,
    AsteroidField,
    Supernova,
}

pub enum PlanetTrait {
    Hazardous,
    Cultural,
    Industrial,
}

pub enum TechColor {
    Red,
    Green,
    Yellow,
    Blue,
}

pub enum WormholeType {
    Alpha,
    Beta,
    Delta,
}

pub enum Faction {
    Sol,
    Mentak,
    Yin,
    Muatt,
    Arborec,
    L1Z1X,
    Winnu,
    Nekro,
    Naalu,
    Barony,
    Saar,
    JolNar,
    Sardak,
    Xxcha,
    Yssaril,
    Hacan,
    Ghosts,
}

pub enum Feature {
    Planet {
        name: &'static str,
        ptrait: Option<PlanetTrait>,
        tech_skip: Option<TechColor>,
        resources: usize,
        influence: usize,
    },
    Wormhole(WormholeType),
}

pub struct System {
    pub id: usize,
    pub background: Background,
    pub features: &'static [Feature],
    pub home: Option<Faction>,
}

pub const NUM_SYSTEMS: usize = 51;
pub static SYSTEMS: [System; NUM_SYSTEMS] = [
    System {
        id: 1,
        home: Some(Faction::Sol),
        background: Background::Empty,
        features: &[Feature::Planet {
            name: "Jord",
            ptrait: None,
            tech_skip: None,
            resources: 4,
            influence: 2,
        }],
    },
    System {
        id: 2,
        home: Some(Faction::Mentak),
        background: Background::Empty,
        features: &[Feature::Planet {
            name: "Moll Primus",
            ptrait: None,
            tech_skip: None,
            resources: 4,
            influence: 1,
        }],
    },
    System {
        id: 3,
        home: Some(Faction::Yin),
        background: Background::Empty,
        features: &[Feature::Planet {
            name: "Darien",
            ptrait: None,
            tech_skip: None,
            resources: 4,
            influence: 4,
        }],
    },
    System {
        id: 4,
        home: Some(Faction::Muatt),
        background: Background::Empty,
        features: &[Feature::Planet {
            name: "Muatt",
            ptrait: None,
            tech_skip: None,
            resources: 4,
            influence: 1,
        }],
    },
    System {
        id: 5,
        home: Some(Faction::Arborec),
        background: Background::Empty,
        features: &[Feature::Planet {
            name: "Nestphar",
            ptrait: None,
            tech_skip: None,
            resources: 3,
            influence: 2,
        }],
    },
    System {
        id: 6,
        home: Some(Faction::L1Z1X),
        background: Background::Empty,
        features: &[Feature::Planet {
            name: "[0.0.0]",
            ptrait: None,
            tech_skip: None,
            resources: 5,
            influence: 0,
        }],
    },
    System {
        id: 7,
        home: Some(Faction::Winnu),
        background: Background::Empty,
        features: &[Feature::Planet {
            name: "Winnu",
            ptrait: None,
            tech_skip: None,
            resources: 3,
            influence: 4,
        }],
    },
    System {
        id: 8,
        home: Some(Faction::Nekro),
        background: Background::Empty,
        features: &[Feature::Planet {
            name: "Mordai II",
            ptrait: None,
            tech_skip: None,
            resources: 4,
            influence: 0,
        }],
    },
    System {
        id: 9,
        home: Some(Faction::Naalu),
        background: Background::Empty,
        features: &[
            Feature::Planet {
                name: "Maaluuk",
                ptrait: None,
                tech_skip: None,
                resources: 0,
                influence: 2,
            },
            Feature::Planet {
                name: "Druaa",
                ptrait: None,
                tech_skip: None,
                resources: 3,
                influence: 1,
            },
        ],
    },
    System {
        id: 10,
        home: Some(Faction::Barony),
        background: Background::Empty,
        features: &[
            Feature::Planet {
                name: "Arc Prime",
                ptrait: None,
                tech_skip: None,
                resources: 4,
                influence: 0,
            },
            Feature::Planet {
                name: "Wren Terra",
                ptrait: None,
                tech_skip: None,
                resources: 2,
                influence: 1,
            },
        ],
    },
    System {
        id: 11,
        home: Some(Faction::Saar),
        background: Background::Empty,
        features: &[
            Feature::Planet {
                name: "Lisis II",
                ptrait: None,
                tech_skip: None,
                resources: 1,
                influence: 0,
            },
            Feature::Planet {
                name: "Ragh",
                ptrait: None,
                tech_skip: None,
                resources: 2,
                influence: 1,
            },
        ],
    },
    System {
        id: 12,
        home: Some(Faction::JolNar),
        background: Background::Empty,
        features: &[
            Feature::Planet {
                name: "Nar",
                ptrait: None,
                tech_skip: None,
                resources: 2,
                influence: 3,
            },
            Feature::Planet {
                name: "Jol",
                ptrait: None,
                tech_skip: None,
                resources: 1,
                influence: 2,
            },
        ],
    },
    System {
        id: 13,
        home: Some(Faction::Sardak),
        background: Background::Empty,
        features: &[
            Feature::Planet {
                name: "Tren'Lak",
                ptrait: None,
                tech_skip: None,
                resources: 1,
                influence: 0,
            },
            Feature::Planet {
                name: "Quinarra",
                ptrait: None,
                tech_skip: None,
                resources: 3,
                influence: 1,
            },
        ],
    },
    System {
        id: 14,
        home: Some(Faction::Xxcha),
        background: Background::Empty,
        features: &[
            Feature::Planet {
                name: "Archon Ren",
                ptrait: None,
                tech_skip: None,
                resources: 2,
                influence: 3,
            },
            Feature::Planet {
                name: "Archon Tau",
                ptrait: None,
                tech_skip: None,
                resources: 1,
                influence: 1,
            },
        ],
    },
    System {
        id: 15,
        home: Some(Faction::Yssaril),
        background: Background::Empty,
        features: &[
            Feature::Planet {
                name: "Retillion",
                ptrait: None,
                tech_skip: None,
                resources: 2,
                influence: 3,
            },
            Feature::Planet {
                name: "Shalloq",
                ptrait: None,
                tech_skip: None,
                resources: 1,
                influence: 2,
            },
        ],
    },
    System {
        id: 16,
        home: Some(Faction::Hacan),
        background: Background::Empty,
        features: &[
            Feature::Planet {
                name: "Hercant",
                ptrait: None,
                tech_skip: None,
                resources: 1,
                influence: 1,
            },
            Feature::Planet {
                name: "Arretze",
                ptrait: None,
                tech_skip: None,
                resources: 2,
                influence: 0,
            },
            Feature::Planet {
                name: "Kamdorn",
                ptrait: None,
                tech_skip: None,
                resources: 0,
                influence: 1,
            },
        ],
    },
    System {
        id: 17,
        home: Some(Faction::Ghosts),
        background: Background::Empty,
        features: &[Feature::Wormhole(WormholeType::Delta)],
    },
    System {
        id: 18,
        home: None,
        background: Background::Empty,
        features: &[Feature::Planet {
            name: "Mecatol Rex",
            ptrait: None,
            tech_skip: None,
            resources: 1,
            influence: 6,
        }],
    },
    System {
        id: 19,
        home: None,
        background: Background::Empty,
        features: &[Feature::Planet {
            name: "Wellon",
            ptrait: Some(PlanetTrait::Industrial),
            tech_skip: Some(TechColor::Yellow),
            resources: 1,
            influence: 2,
        }],
    },
    System {
        id: 20,
        home: None,
        background: Background::Empty,
        features: &[Feature::Planet {
            name: "Vefut II",
            ptrait: Some(PlanetTrait::Hazardous),
            tech_skip: None,
            resources: 1,
            influence: 2,
        }],
    },
    System {
        id: 21,
        home: None,
        background: Background::Empty,
        features: &[Feature::Planet {
            name: "Thibah",
            ptrait: Some(PlanetTrait::Industrial),
            tech_skip: Some(TechColor::Blue),
            resources: 1,
            influence: 1,
        }],
    },
    System {
        id: 22,
        home: None,
        background: Background::Empty,
        features: &[Feature::Planet {
            name: "Tar'Mann",
            ptrait: Some(PlanetTrait::Industrial),
            tech_skip: Some(TechColor::Green),
            resources: 1,
            influence: 1,
        }],
    },
    System {
        id: 23,
        home: None,
        background: Background::Empty,
        features: &[Feature::Planet {
            name: "Saudor",
            ptrait: Some(PlanetTrait::Industrial),
            tech_skip: None,
            resources: 2,
            influence: 2,
        }],
    },
    System {
        id: 24,
        home: None,
        background: Background::Empty,
        features: &[Feature::Planet {
            name: "Mehar Xull",
            ptrait: Some(PlanetTrait::Hazardous),
            tech_skip: Some(TechColor::Red),
            resources: 1,
            influence: 3,
        }],
    },
    System {
        id: 25,
        home: None,
        background: Background::Empty,
        features: &[
            Feature::Planet {
                name: "Quann",
                ptrait: Some(PlanetTrait::Cultural),
                tech_skip: None,
                resources: 1,
                influence: 2,
            },
            Feature::Wormhole(WormholeType::Beta),
        ],
    },
    System {
        id: 26,
        home: None,
        background: Background::Empty,
        features: &[
            Feature::Planet {
                name: "Lodor",
                ptrait: Some(PlanetTrait::Cultural),
                tech_skip: None,
                resources: 3,
                influence: 1,
            },
            Feature::Wormhole(WormholeType::Alpha),
        ],
    },
    System {
        id: 27,
        home: None,
        background: Background::Empty,
        features: &[
            Feature::Planet {
                name: "New Albion",
                ptrait: Some(PlanetTrait::Industrial),
                tech_skip: Some(TechColor::Green),
                resources: 1,
                influence: 1,
            },
            Feature::Planet {
                name: "Starpoint",
                ptrait: Some(PlanetTrait::Hazardous),
                tech_skip: None,
                resources: 3,
                influence: 1,
            },
        ],
    },
    System {
        id: 28,
        home: None,
        background: Background::Empty,
        features: &[
            Feature::Planet {
                name: "Tequ'Ran",
                ptrait: Some(PlanetTrait::Hazardous),
                tech_skip: None,
                resources: 2,
                influence: 0,
            },
            Feature::Planet {
                name: "Torkan",
                ptrait: Some(PlanetTrait::Cultural),
                tech_skip: None,
                resources: 0,
                influence: 3,
            },
        ],
    },
    System {
        id: 29,
        home: None,
        background: Background::Empty,
        features: &[
            Feature::Planet {
                name: "Qucen'N",
                ptrait: Some(PlanetTrait::Industrial),
                tech_skip: None,
                resources: 1,
                influence: 2,
            },
            Feature::Planet {
                name: "Rarron",
                ptrait: Some(PlanetTrait::Cultural),
                tech_skip: None,
                resources: 0,
                influence: 3,
            },
        ],
    },
    System {
        id: 30,
        home: None,
        background: Background::Empty,
        features: &[
            Feature::Planet {
                name: "Mellon",
                ptrait: Some(PlanetTrait::Cultural),
                tech_skip: None,
                resources: 0,
                influence: 2,
            },
            Feature::Planet {
                name: "Zohbat",
                ptrait: Some(PlanetTrait::Hazardous),
                tech_skip: None,
                resources: 3,
                influence: 1,
            },
        ],
    },
    System {
        id: 31,
        home: None,
        background: Background::Empty,
        features: &[
            Feature::Planet {
                name: "Lazar",
                ptrait: Some(PlanetTrait::Industrial),
                tech_skip: Some(TechColor::Yellow),
                resources: 1,
                influence: 0,
            },
            Feature::Planet {
                name: "Sakulag",
                ptrait: Some(PlanetTrait::Hazardous),
                tech_skip: None,
                resources: 2,
                influence: 1,
            },
        ],
    },
    System {
        id: 32,
        home: None,
        background: Background::Empty,
        features: &[
            Feature::Planet {
                name: "Tequ'Ran",
                ptrait: Some(PlanetTrait::Cultural),
                tech_skip: None,
                resources: 0,
                influence: 2,
            },
            Feature::Planet {
                name: "Torkan",
                ptrait: Some(PlanetTrait::Cultural),
                tech_skip: None,
                resources: 1,
                influence: 1,
            },
        ],
    },
    System {
        id: 33,
        home: None,
        background: Background::Empty,
        features: &[
            Feature::Planet {
                name: "Corneeq",
                ptrait: Some(PlanetTrait::Cultural),
                tech_skip: None,
                resources: 1,
                influence: 2,
            },
            Feature::Planet {
                name: "Resculon",
                ptrait: Some(PlanetTrait::Cultural),
                tech_skip: None,
                resources: 2,
                influence: 0,
            },
        ],
    },
    System {
        id: 34,
        home: None,
        background: Background::Empty,
        features: &[
            Feature::Planet {
                name: "Centauri",
                ptrait: Some(PlanetTrait::Cultural),
                tech_skip: None,
                resources: 1,
                influence: 3,
            },
            Feature::Planet {
                name: "Gral",
                ptrait: Some(PlanetTrait::Industrial),
                tech_skip: Some(TechColor::Blue),
                resources: 1,
                influence: 1,
            },
        ],
    },
    System {
        id: 35,
        home: None,
        background: Background::Empty,
        features: &[
            Feature::Planet {
                name: "Bereg",
                ptrait: Some(PlanetTrait::Hazardous),
                tech_skip: None,
                resources: 3,
                influence: 1,
            },
            Feature::Planet {
                name: "Lirta IV",
                ptrait: Some(PlanetTrait::Hazardous),
                tech_skip: None,
                resources: 2,
                influence: 3,
            },
        ],
    },
    System {
        id: 36,
        home: None,
        background: Background::Empty,
        features: &[
            Feature::Planet {
                name: "Arnor",
                ptrait: Some(PlanetTrait::Industrial),
                tech_skip: None,
                resources: 2,
                influence: 1,
            },
            Feature::Planet {
                name: "Lor",
                ptrait: Some(PlanetTrait::Industrial),
                tech_skip: None,
                resources: 1,
                influence: 2,
            },
        ],
    },
    System {
        id: 37,
        home: None,
        background: Background::Empty,
        features: &[
            Feature::Planet {
                name: "Arinam",
                ptrait: Some(PlanetTrait::Industrial),
                tech_skip: None,
                resources: 1,
                influence: 2,
            },
            Feature::Planet {
                name: "Meer",
                ptrait: Some(PlanetTrait::Hazardous),
                tech_skip: Some(TechColor::Red),
                resources: 0,
                influence: 4,
            },
        ],
    },
    System {
        id: 38,
        home: None,
        background: Background::Empty,
        features: &[
            Feature::Planet {
                name: "Abyz",
                ptrait: Some(PlanetTrait::Hazardous),
                tech_skip: None,
                resources: 3,
                influence: 0,
            },
            Feature::Planet {
                name: "Fria",
                ptrait: Some(PlanetTrait::Hazardous),
                tech_skip: None,
                resources: 2,
                influence: 0,
            },
        ],
    },
    System {
        id: 39,
        home: None,
        background: Background::Empty,
        features: &[Feature::Wormhole(WormholeType::Alpha)],
    },
    System {
        id: 40,
        home: None,
        background: Background::Empty,
        features: &[Feature::Wormhole(WormholeType::Beta)],
    },
    System {
        id: 41,
        home: None,
        background: Background::GravityRift,
        features: &[],
    },
    System {
        id: 42,
        home: None,
        background: Background::Nebula,
        features: &[],
    },
    System {
        id: 43,
        home: None,
        background: Background::Supernova,
        features: &[],
    },
    System {
        id: 44,
        home: None,
        background: Background::AsteroidField,
        features: &[],
    },
    System {
        id: 45,
        home: None,
        background: Background::AsteroidField,
        features: &[],
    },
    System {
        id: 46,
        home: None,
        background: Background::Empty,
        features: &[],
    },
    System {
        id: 47,
        home: None,
        background: Background::Empty,
        features: &[],
    },
    System {
        id: 48,
        home: None,
        background: Background::Empty,
        features: &[],
    },
    System {
        id: 49,
        home: None,
        background: Background::Empty,
        features: &[],
    },
    System {
        id: 50,
        home: None,
        background: Background::Empty,
        features: &[],
    },
    System {
        id: 51,
        home: Some(Faction::Ghosts),
        background: Background::Empty,
        features: &[
            Feature::Planet {
                name: "Creuss",
                ptrait: None,
                tech_skip: None,
                resources: 4,
                influence: 2,
            },
            Feature::Wormhole(WormholeType::Delta),
        ],
    },
];

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn system_ids() {
        for (i, v) in SYSTEMS.iter().enumerate() {
            assert_eq!(i + 1, v.id as usize);
        }
    }
}
