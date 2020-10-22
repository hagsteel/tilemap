pub struct TileMap<T> {
    tiles: Vec<T>,
    pub width: u16,
    pub height: u16,
}

impl<T> TileMap<T> {
    pub fn to_index(&self, p: (u16, u16)) -> u16 {
        p.0 + p.1 * self.width
    }

    pub fn new(width: u16, height: u16) -> Self {
        let cap = (width * height) as usize;
        debug_assert!(cap <= u16::MAX as usize);

        Self {
            tiles: Vec::with_capacity(cap),
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

    pub fn get(&self, index: u16) -> &T {
        &self.tiles[index as usize]
    }

    pub fn by_coords(&self, p: (u16, u16)) -> Option<&T> {
        let index = self.to_index(p);
        if index as usize > self.tiles.len() {
            None
        } else {
            Some(self.get(index))
        }
    }

    /// x1 and y1 has to be smaller than x2 and y2
    pub fn coords_in_area(&self, p1: (u16, u16), p2: (u16, u16)) -> impl Iterator<Item = (u16, u16)> {
        debug_assert!(p1.0 <= p2.0);
        debug_assert!(p1.1 <= p2.1);
        (p1.1..=p2.1).flat_map(move |y| (p1.0..=p2.0).map(move |x| (x, y)))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_coords_to_index() {
        let map = TileMap::<usize>::new(4, 2);
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
        let mut map = TileMap::new(4u16, 2);
        map.append(vec![0u16, 1, 2, 3, 4, 5, 6, 7]);
    }
}
