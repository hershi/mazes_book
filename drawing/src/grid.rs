use image::{GenericImage, ImageBuffer, GrayImage, RgbImage, Rgb};
use imageproc::drawing::*;

use std::collections::HashSet;
use std::iter;
use std::fmt;
use std::path::Path;

#[derive(Hash, Eq, PartialEq, Debug, Clone)]
pub struct Coordinates {
    pub row: isize,
    pub col: isize,
}

impl Coordinates {
    pub fn new(row: isize, col: isize) -> Coordinates {
        Coordinates{row, col}
    }
}

#[derive(Debug)]
pub struct Cell {
    pub coordinates: Coordinates,
    pub links: HashSet<Coordinates>,
    pub north: Option<Coordinates>,
    pub south: Option<Coordinates>,
    pub east: Option<Coordinates>,
    pub west: Option<Coordinates>,
}

fn validate_coordinates(pos: Coordinates, rows: isize, cols: isize) -> Option<Coordinates> {
    if pos.row >= 0 && pos.row < rows && pos.col >=0 && pos.col < cols {
        Some(pos)
    } else {
        None
    }
}

impl Cell {
    pub fn new(row: isize, col: isize, rows: isize, cols: isize) -> Cell {
        let coordinates = Coordinates{row, col};
        Cell {
            coordinates,
            links: HashSet::new(),
            north: validate_coordinates(Coordinates{row: row-1, col}, rows, cols),
            south: validate_coordinates(Coordinates{row: row+1, col}, rows, cols),
            east:  validate_coordinates(Coordinates{row , col: col+1}, rows, cols),
            west:  validate_coordinates(Coordinates{row , col: col-1}, rows, cols)}
    }

    pub fn is_linked(&self, other_coordinates: &Option<Coordinates>) -> bool {
        match other_coordinates {
            Some(coordinates) => self.links.contains(coordinates),
            None => false
        }
    }

    pub fn unlink(&mut self, cell: &mut Cell) {
        self.links.remove(&cell.coordinates);
        cell.links.remove(&self.coordinates);
    }

    pub fn neighbors(&self) -> Vec<Coordinates> {
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
pub struct Grid {
    pub grid: Vec<Cell>,
    pub rows: isize,
    pub cols: isize,
}

impl Grid {
    pub fn new(rows: isize, cols: isize) -> Grid {
        let num_cells = (rows * cols) as usize;
        let mut grid = Vec::with_capacity(num_cells);
        for row in 0..rows {
            for col in 0..cols {
                grid.push(Cell::new(row, col, rows, cols));
            }
        }
        Grid {grid,  rows, cols}
    }

    pub fn link(&mut self, pos1: Coordinates, pos2: Coordinates) {
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

    pub fn get(&self, pos: &Coordinates) -> Option<&Cell> {
        self.get_index(pos).map(|i| &self.grid[i])
    }

    pub fn get_mut(&mut self, pos: Coordinates) -> Option<&mut Cell> {
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

pub fn draw(grid : &Grid, path: &Path) {
    let side_length = 30;
    let white = Rgb([255u8, 255u8, 255u8]);

    let width_in_pixels = (grid.cols * (side_length + 1)) as u32;
    let height_in_pixels = (grid.rows * (side_length + 1)) as u32;

    let mut img = RgbImage::new(width_in_pixels+1, height_in_pixels+1);
    let top_left = (0f32, 0f32);
    let top_right = (0f32, (width_in_pixels-1) as f32);
    let bottom_left = ((height_in_pixels-1) as f32, 0f32);
    let bottom_right = ((height_in_pixels-1) as f32, (width_in_pixels-1) as f32);
    draw_line_segment_mut(&mut img, top_left, top_right, white);
    draw_line_segment_mut(&mut img, top_right, bottom_right, white);
    draw_line_segment_mut(&mut img, bottom_right, bottom_left, white);
    draw_line_segment_mut(&mut img, bottom_left, top_left, white);

    for row in 0..grid.rows {
        for col in 0..grid.cols {
            let cell_pos = Coordinates::new(row, col);
            let cell = grid.get(&cell_pos).unwrap();

            let left = (1 + col * (side_length + 1)) as f32;
            let right = left + side_length as f32 + 1f32;
            let top = (1 + row * (side_length + 1)) as f32;
            let bottom = top + side_length as f32 + 1f32;

            if !cell.is_linked(&cell.south) {
                draw_line_segment_mut(&mut img, (left, bottom), (right, bottom), white);
            }

            if !cell.is_linked(&cell.west) {
                draw_line_segment_mut(&mut img, (left, bottom), (left, top), white);
            }
        }
    }

    img.save(path).unwrap();
}

