extern crate rand;

use std::collections::HashSet;
use std::iter;
use std::fmt;
use std::io::Write;

#[derive(Hash, Eq, PartialEq, Debug, Clone)]
struct Coordinates {
    row: isize,
    col: isize,
}

impl Coordinates {
    fn new(row: isize, col: isize) -> Coordinates {
        Coordinates{row, col}
    }
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

    //fn link(&mut self, cell1: Coordinates, cell2: Coordinates) {
        //let cell1 = self.get_index(cell1);
        //let cell2 = self.get_index(cell2);
        //self.grid[cell1].insert(
        //self.links.insert(cell.coordinates.clone());
        //cell.links.insert(self.coordinates.clone());
    //}

    fn get_index(&self, pos: Coordinates) -> Option<usize> {
        if pos.row >= 0 && pos.row < self.rows && pos.col >=0 && pos.col < self.cols {
            Some((pos.row * self.cols + pos.col) as usize)
        } else {
            None
        }
    }

    fn get(&self, pos: Coordinates) -> Option<&Cell> {
        self.get_index(pos).map(|i| &self.grid[i])
    }

    fn get_mut(&mut self, pos: Coordinates) -> Option<&mut Cell> {
        if pos.row >= 0 && pos.row < self.rows && pos.col >=0 && pos.col < self.cols {
            Some(&mut self.grid[(pos.row * self.cols + pos.col) as usize])
        } else {
            None
        }
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
                let cell = self.get(Coordinates::new(row,col)).unwrap();
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

//fn binary_tree(grid: &mut Grid) {
    //for cell in grid.grid.iter_mut() {
        //if cell.north.is_none() && cell.east.is_none() { continue; }

        //if cell.north.is_none() {
            //let east = cell.east.unwrap();
            //cell.link(grid.get_mut((east.row, east.col)).unwrap());
            //continue;
        //}

        //if cell.east.is_none() {
            //let north = cell.north.unwrap();
            //cell.link(grid.get_mut((north.row, north.col)).unwrap());
            //continue;
        //}

        //if rand::random() {
            //let north = cell.north.unwrap();
            //cell.link(grid.get_mut((north.row, north.col)).unwrap());
        //} else {
            //let east = cell.east.unwrap();
            //cell.link(grid.get_mut((east.row, east.col)).unwrap());
        //}
    //}
//}

fn main() {
    let mut grid = Grid::new(10,10);
    println!("{:?}", grid.get(Coordinates::new(1,2)));
    println!("{}", grid);
}
