use std::collections::HashSet;
use std::ops::Index;
use std::ops::IndexMut;
use std::iter;
use std::fmt;
use std::io::Write;

#[derive(Hash, Eq, PartialEq, Debug, Clone)]
struct Coordinates {
    row: isize,
    col: isize,
}

#[derive(Debug)]
struct Cell {
    coordinates: Coordinates,
    links: HashSet<Coordinates>,
    north: Option<Coordinates>,
    south: Option<Coordinates>,
    east: Option<Coordinates>,
    west: Option<Coordinates>,
}

impl Cell {
    fn new(row: isize, col: isize) -> Cell {
        let coordinates = Coordinates{row, col};
        Cell {
            coordinates,
            links: HashSet::new(),
            north: Some(Coordinates{row: row-1, col}),
            south:Some(Coordinates{row: row+1, col}),
            east: Some(Coordinates{row , col: col+1}),
            west: Some(Coordinates{row , col: col-1})}
    }

    fn is_linked(&self, other_coordinates: &Option<Coordinates>) -> bool {
        match other_coordinates {
            Some(coordinates) => self.links.contains(coordinates),
            None => false
        }
    }

    fn link(&mut self, cell: &mut Cell) {
        self.links.insert(cell.coordinates.clone());
        cell.links.insert(self.coordinates.clone());
    }

    fn unlink(&mut self, cell: &mut Cell) {
        self.links.remove(&cell.coordinates);
        cell.links.remove(&self.coordinates);
    }

    fn neighbors(&self) -> Vec<Coordinates> {
        vec![self.north.clone(),
             self.south.clone(),
             self.east.clone(),
             self.west.clone()]
                 .into_iter()
                 .flat_map(|x|x)
                 .collect()
    }
}

#[derive(Debug)]
struct Grid {
    grid: Vec<Cell>,
    rows: isize,
    cols: isize,
}

impl Grid {
    fn new(rows: isize, cols: isize) -> Grid {
        let num_cells = (rows * cols) as usize;
        let mut grid = Vec::with_capacity(num_cells);
        for row in 0..rows {
            for col in 0..cols {
                grid.push(Cell::new(row, col));
            }
        }
        Grid {grid,  rows, cols}
    }
}

impl Index<(isize, isize)> for Grid {
    type Output = Cell;

    fn index(&self, idx: (isize, isize)) -> &Cell {
        &self.grid[(idx.0 * self.cols + idx.1) as usize]
    }
}

impl IndexMut<(isize, isize)> for Grid {
    fn index_mut(&mut self, idx: (isize, isize)) -> &mut Cell {
        &mut self.grid[(idx.0 * self.cols + idx.1) as usize]
    }
}

impl fmt::Display for Grid {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let line =
            iter::once("+")
                .chain(iter::repeat("---+").take(self.cols as usize))
                .collect::<Vec<_>>()
                .join("");
        writeln!(f, "{}", line);

        for row in 0..self.rows {
            let mut line1 = "|".to_string();
            let mut line2 = "+".to_string();
            for col in 0..self.cols {
                let cell = &self[(row,col)];
                if cell.is_linked(&cell.east) {
                    line1.push_str("    ");
                } else {
                    line1.push_str("   |")
                }

                if cell.is_linked(&cell.south) {
                    line2.push_str("   +")
                } else {
                    line2.push_str("---+")
                }
            }
            writeln!(f, "{}", line1);
            writeln!(f, "{}", line2);
        }

        Ok(())
    }
}

fn main() {
    let mut grid = Grid::new(10,10);
    println!("{:?}", grid[(1,2)]);
    println!("{}", grid);
}
