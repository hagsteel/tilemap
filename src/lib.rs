pub fn coords_to_index(x: usize, y: usize, width: usize) -> usize {
    x + y * width
}

pub struct Tile {
}

pub struct TileMap<T> {
    tiles: Vec<T>,
    pub width: usize,
    pub height: usize,
}

impl<T> TileMap<T> {
    pub fn new(width: usize, height: usize) -> Self {
        Self {
            tiles: Vec::with_capacity(width * height),
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

    pub fn get(&self, index: usize) -> &T {
        &self.tiles[index]
    }

    pub fn by_coords(&self, x: usize, y: usize) -> Option<&T> {
        let index = coords_to_index(x, y, self.width);
        Some(self.get(index))
    }

    /// x1 and y1 has to be smaller than x2 and y2
    pub fn indices_by_area(&self, p1: (usize, usize), p2: (usize, usize)) -> impl Iterator<Item=(usize, usize)> {
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
        let coords = coords_to_index(0, 1, 4);
        eprintln!("{:?}", coords);
    }

    #[test]
    fn by_coords() {
        let mut map = TileMap::new(4, 2);
        map.append(vec![
            0, 1, 2, 3, 
            4, 5, 6, 7
        ]);
        let t = map.by_coords(2, 1);
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
        map.append(vec![
            0, 1, 2, 3, 
            4, 5, 6, 7
        ]);
        let tiles = map.indices_by_area(0, 0, 1, 1);
        assert_eq!(tiles, vec![0, 1, 4, 5]);
    }
}
