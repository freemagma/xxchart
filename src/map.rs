use crate::system;
use std::collections::HashMap;
use std::ops::*;

#[derive(PartialEq, Eq, Hash, Clone, Copy, Debug)]
pub struct Coord {
    x: isize,
    y: isize,
}
impl Add for Coord {
    type Output = Self;
    fn add(self, other: Self) -> Self {
        Self {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}
impl Mul<isize> for Coord {
    type Output = Self;
    fn mul(self, rhs: isize) -> Self::Output {
        Coord {
            x: self.x * rhs,
            y: self.y * rhs,
        }
    }
}
static DIRECTIONS: [Coord; 6] = [
    Coord { x: 1, y: 0 },
    Coord { x: 0, y: 1 },
    Coord { x: -1, y: 1 },
    Coord { x: -1, y: 0 },
    Coord { x: 0, y: -1 },
    Coord { x: 1, y: -1 },
];

pub struct CoordRings {
    radius: usize,
    current: Coord,
    direction: usize,
    progress: usize,
    start: bool,
}
impl CoordRings {
    pub fn new() -> Self {
        CoordRings {
            radius: 1,
            current: DIRECTIONS[4],
            direction: 0,
            progress: 0,
            start: true,
        }
    }
}
impl Iterator for CoordRings {
    type Item = Coord;
    fn next(&mut self) -> Option<Self::Item> {
        if self.start {
            self.start = false;
        } else {
            self.current = self.current.in_direction(self.direction);
            self.progress += 1;
            if self.progress == self.radius {
                self.progress = 0;
                self.direction += 1;
            }
            if self.direction == 6 {
                self.direction = 0;
                self.radius += 1;
                self.current = DIRECTIONS[4] * (self.radius as isize);
            }
        }
        Some(self.current)
    }
}

impl Coord {
    pub fn new(x: isize, y: isize, z: isize) -> Self {
        assert_eq!(x + y + z, 0);
        Coord { x, y }
    }
    pub fn x(&self) -> isize {
        self.x
    }
    pub fn y(&self) -> isize {
        self.y
    }
    pub fn z(&self) -> isize {
        -(self.x + self.y)
    }
    pub fn neighbors(&self) -> impl Iterator + '_ {
        let self_copy = self.clone();
        DIRECTIONS.iter().map(move |x| self_copy + *x)
    }
    fn in_direction(&self, index: usize) -> Self {
        DIRECTIONS[index] + *self
    }
}

#[derive(Debug)]
pub struct Map(HashMap<Coord, &'static system::System>);
impl Deref for Map {
    type Target = HashMap<Coord, &'static system::System>;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl DerefMut for Map {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl Map {
    pub fn new() -> Self {
        Map(HashMap::new())
    }
    pub fn from_map_string(map_string: &str) -> Self {
        let mut h = HashMap::new();
        let id_iter = map_string
            .split(' ')
            .into_iter()
            .map(|s| s.parse::<usize>().unwrap());
        for (id, coord) in id_iter.zip(CoordRings::new()) {
            if id > 0 {
                h.insert(coord, system::id_system(id));
            }
        }
        Map(h)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn coord_sum_one() {
        let c1 = Coord { x: 3, y: -7 };
        let c2 = Coord { x: 0, y: -1 };
        let c3 = Coord::new(100, -99, -1);
        assert_eq!(c1.x() + c1.y() + c1.z(), 0);
        assert_eq!(c2.x() + c2.y() + c2.z(), 0);
        assert_eq!(c3.x() + c3.y() + c3.z(), 0);
    }
}
