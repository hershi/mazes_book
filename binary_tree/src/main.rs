extern crate rand;

mod grid;

use crate::grid::*;

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
