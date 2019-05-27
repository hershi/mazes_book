extern crate rand;

use std::collections::HashSet;
use std::iter;
use std::fmt;

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

fn validate_coordinates(pos: Coordinates, rows: isize, cols: isize) -> Option<Coordinates> {
    if pos.row >= 0 && pos.row < rows && pos.col >=0 && pos.col < cols {
        Some(pos)
    } else {
        None
    }
}

impl Cell {
    fn new(row: isize, col: isize, rows: isize, cols: isize) -> Cell {
        let coordinates = Coordinates{row, col};
        Cell {
            coordinates,
            links: HashSet::new(),
            north: validate_coordinates(Coordinates{row: row-1, col}, rows, cols),
            south: validate_coordinates(Coordinates{row: row+1, col}, rows, cols),
            east:  validate_coordinates(Coordinates{row , col: col+1}, rows, cols),
            west:  validate_coordinates(Coordinates{row , col: col-1}, rows, cols)}
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
                grid.push(Cell::new(row, col, rows, cols));
            }
        }
        Grid {grid,  rows, cols}
    }

    fn link(&mut self, pos1: Coordinates, pos2: Coordinates) {
        let cell1 = self.get_index(&pos1).unwrap();
        let cell2 = self.get_index(&pos2).unwrap();
        self.grid[cell1].links.insert(pos2.clone());
        self.grid[cell2].links.insert(pos1.clone());
    }

    fn get_index(&self, pos: &Coordinates) -> Option<usize> {
        if pos.row >= 0 && pos.row < self.rows && pos.col >=0 && pos.col < self.cols {
            Some((pos.row * self.cols + pos.col) as usize)
        } else {
            None
        }
    }

    fn get(&self, pos: &Coordinates) -> Option<&Cell> {
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
                let cell = self.get(&Coordinates::new(row,col)).unwrap();
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

fn binary_tree_decide_link(cell: Option<&Cell>) -> Option<Coordinates> {
    cell.map_or(None, |cell| {
        let options = vec![cell.north.clone(), cell.east.clone()]
            .into_iter()
            .flat_map(|x| x)
            .collect::<Vec<Coordinates>>();


        if options.len() == 0 {
            None
        } else {
            let idx = rand::random::<usize>() % options.len();
            Some(options[idx].clone()) // why can't move?
        }
    })
}

fn binary_tree(grid: &mut Grid) {
    for row in 0..grid.rows {
        for col in 0..grid.cols {
            let cell_pos = Coordinates::new(row, col);

            let linked = binary_tree_decide_link(grid.get(&cell_pos));

            if linked.is_some() {
                grid.link(cell_pos, linked.unwrap());
            }
        }
    }
}

fn main() {
    let mut grid = Grid::new(10,10);
    binary_tree(&mut grid);
    println!("{}", grid);
}
