use std::fmt::Debug;
use std::ops::{Add, AddAssign, Sub};
use std::str::FromStr;

#[derive(Debug)]
enum ParsePointError {}

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Hash)]
pub struct Point<T> {
    pub x: T,
    pub y: T,
}

impl<T> Point<T> {
    pub fn new(x: T, y: T) -> Self {
        Self { x, y }
    }

    pub fn neighbors(&self) -> Vec<Self>
    where
        T: Copy + Add<T, Output = T> + Sub<T, Output = T> + From<i8>,
    {
        let mut neighbors = Vec::new();
        for x in -1..=1 {
            for y in -1..=1 {
                if x == 0 && y == 0 {
                    continue;
                }
                neighbors.push(Self {
                    x: self.x + T::from(x),
                    y: self.y + T::from(y),
                });
            }
        }
        neighbors
    }

    pub fn neighbors_without_diagonals(&self) -> Vec<Self>
    where
        T: Copy + Add<T, Output = T> + Sub<T, Output = T> + From<i8>,
    {
        let mut neighbors = Vec::new();
        let neighbor_indices = [(-1, 0), (0, -1), (1, 0), (0, 1)];
        for (x, y) in neighbor_indices.iter() {
            neighbors.push(Self {
                x: self.x + T::from(*x),
                y: self.y + T::from(*y),
            });
        }
        neighbors
    }
}

impl<T: AddAssign> AddAssign for Point<T> {
    fn add_assign(&mut self, other: Self) {
        self.x += other.x;
        self.y += other.y;
    }
}

impl<T: FromStr> Point<T> {
    pub fn parse(s: &str, separator: &str) -> Self
    where
        <T as FromStr>::Err: Debug,
    {
        let mut value = s.split(separator);
        let x = value.next().unwrap().parse::<T>().unwrap();
        let y = value.next().unwrap().parse::<T>().unwrap();

        Self { x, y }
    }
}

impl<T: FromStr> FromStr for Point<T> {
    type Err = String;

    fn from_str(p: &str) -> Result<Self, Self::Err> {
        let (x, y) = p
            .split_once(',')
            .or(p.split_once(", "))
            .ok_or("input file corrupt")?;
        let (x, y) = (x.trim(), y.trim());

        Ok(Self {
            x: x.parse().map_err(|_| format!("x value '{x}' corrupt"))?,
            y: y.parse().map_err(|_| format!("y value '{y}' corrupt"))?,
        })
    }
}

impl<T: Copy + Ord + Add<T, Output = T> + Sub<T, Output = T>> Point<T> {
    pub fn min(&self, other: &Self) -> Self {
        Self {
            x: Ord::min(self.x, other.x),
            y: Ord::min(self.y, other.y),
        }
    }

    pub fn max(&self, other: &Self) -> Self {
        Self {
            x: Ord::max(self.x, other.x),
            y: Ord::max(self.y, other.y),
        }
    }

    pub fn dist(&self, x: T, y: T) -> T
    where
        <T as Sub>::Output: Add<T>,
    {
        let delta_x = if self.x > x { self.x - x } else { x - self.x };
        let delta_y = if self.y > y { self.y - y } else { y - self.y };

        delta_x + delta_y
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn comma_space_separated() {
        let p = "1, 2".parse::<Point<usize>>().unwrap();
        assert_eq!(Point { x: 1usize, y: 2 }, p);
    }
}
