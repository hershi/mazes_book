extern crate rand;

mod grid;

use crate::grid::*;

fn sidewinder(grid: &mut Grid) {
    for row in 0..grid.rows {
        let mut run_start : Option<isize> = None;
        for col in 0..grid.cols {
            run_start = Some(run_start.unwrap_or(col));
            let go_east = if col >= grid.cols-1 { false } else {row >= grid.rows - 1 || rand::random::<bool>()};

            let cell_pos = Coordinates::new(row, col);
            if go_east {
                let east_pos = grid.get(&cell_pos).map(|x|x.east.clone().unwrap()).unwrap();
                grid.link(cell_pos, east_pos);
                continue;
            }

            if row >= grid.rows - 1 { continue; }

            // Close the run
            let run_length = col as usize - run_start.unwrap() as usize + 1;
            let south_col = run_start.unwrap() + (rand::random::<usize>() % run_length) as isize;
            let from_pos = Coordinates::new(row, south_col);

            let to_pos = grid.get(&from_pos).map(|x|x.south.clone().unwrap()).unwrap();
            grid.link(from_pos, to_pos);
            run_start = None;
        }
    }
}

fn main() {
    let mut grid = Grid::new(15,15);
    sidewinder(&mut grid);
    println!("{}", grid);
}
