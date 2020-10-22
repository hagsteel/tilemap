use std::ops::{Add, Div, Mul, Sub};
use std::cmp::{Ord, PartialOrd};
pub trait Scalar :
      Sized
      + PartialEq
      + Copy
      + PartialOrd
      + Ord
      + Eq
      + Add<Self, Output = Self>
      + Sub<Self, Output = Self>
      + Mul<Self, Output = Self>
      + Div<Self, Output = Self>
      + Into<usize>
      + From<usize>
{}

impl Scalar for usize {}
impl Scalar for u32 {}
impl Scalar for u16 {}
impl Scalar for u8 {}

pub struct TileMap<T, U: Scalar> {
    tiles: Vec<T>,
    pub width: U,
    pub height: U,
}

impl<T, U: Scalar> TileMap<T, U> {
    pub fn to_index(&self, p: (U, U)) -> U {
        p.0 + p.1 * self.width
    }

    pub fn new(width: U, height: U) -> Self {
        Self {
            tiles: Vec::with_capacity(width.into() * height.into()),
            width,
            height,
        }
    }

    pub fn push(&mut self, tile: T) {
        self.tiles.push(tile);
    }

    pub fn append(&mut self, mut tiles: Vec<T>) {
        self.tiles.append(&mut tiles);
    }

    pub fn get(&self, index: U) -> &T {
        &self.tiles[index.into()]
    }

    pub fn by_coords(&self, p: (U, U)) -> Option<&T> {
        let index = self.to_index(p);
        if index.into()  > self.tiles.len() {
            None
        } else {
            Some(self.get(index))
        }
    }

    /// x1 and y1 has to be smaller than x2 and y2
    pub fn coords_in_area(&self, p1: (U, U), p2: (U, U)) -> impl Iterator<Item = (U, U)> {
        debug_assert!(p1.0 <= p2.0);
        debug_assert!(p1.1 <= p2.1);
        (p1.1.into()..=p2.1.into()).flat_map(move |y| (p1.0.into()..=p2.0.into()).map(move |x| (x.into(), y.into())))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_coords_to_index() {
        let map = TileMap::<usize, usize>::new(4, 2);
        let index = map.to_index((0, 1));
        assert_eq!(index, 4);
    }

    #[test]
    fn by_coords() {
        let mut map = TileMap::new(4, 2);
        map.append(vec![0, 1, 2, 3, 4, 5, 6, 7]);
        let t = map.by_coords((2, 1));
        assert_eq!(Some(&6), t);
    }

    #[test]
    fn by_index() {
        let mut map = TileMap::new(2, 2);
        map.append(vec![0, 1, 2, 3]);
        let t = map.get(2);
        assert_eq!(&2, t);
    }

    #[test]
    fn by_area() {
        let mut map = TileMap::new(4, 2);
        map.append(vec![0, 1, 2, 3, 4, 5, 6, 7]);
        let tiles = map.coords_in_area((0, 0), (1, 1)).map(|p| map.to_index(p));
        assert_eq!(tiles.collect::<Vec<_>>(), vec![0, 1, 4, 5]);
    }

    #[test]
    fn using_u16() {
        let mut map = TileMap::new(4, 2);
        map.append(vec![0u16, 1, 2, 3, 4, 5, 6, 7]);
    }
}
