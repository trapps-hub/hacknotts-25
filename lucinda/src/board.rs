use std::collections::{HashMap, HashSet};
use std::marker::PhantomData;
use std::ops::Deref;
use nalgebra::SMatrix;
use rand::prelude::*;
use rayon::prelude::*;
use crate::board::build_state::Empty;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn place_test() {
        let test_board = BoardBuilder::new();
        let test_board = test_board.place_queens();

        println!("{}", test_board.board.map(|x| x.has_queen).map(|x| if x { 1 } else {0}));
    }

    #[test]
    fn fill_test() {
        let test_board = BoardBuilder::new();
        let test_board = test_board.place_queens();
        
        println!("{}", test_board.board.map(|x| x.has_queen).map(|x| if x { 1 } else {0}));

        let test_board = test_board.flood_fill();

        println!("{}", test_board.board.map(|x| x.region).map(|x| {
            x as u8
        }));
    }
    #[test]
    fn comp_test() {
        let test_board = BoardBuilder::new()
            .place_queens()
            .flood_fill()
            .validate_unique().unwrap();

        let mut x = SMatrix::default();
        x[(thread_rng().gen_range(0..8),thread_rng().gen_range(0..8))] = true;
        x[(thread_rng().gen_range(0..8),thread_rng().gen_range(0..8))] = true;

        println!("{}", test_board.board.map(|x| x.region).map(|x| {
            x as u8
        }));

        println!("{}", validate_grid(*test_board, x));
    }
}

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
#[repr(u8)]
pub enum Regions {
    Unclaimed = 0,
    LAVA,
    Hinterlands,
    Farms,
    MySwamp,
    Lake,
    Ocean,
    Burpl,
    Castle
}

impl Regions {
    pub const fn as_color(&self) -> &'static str {
        match self {
            Regions::Unclaimed => "#dadada",
            Regions::LAVA => "#de3030",
            Regions::Hinterlands => "#e07a34",
            Regions::Farms => "#e2c337",
            Regions::MySwamp => "#9dbd3d",
            Regions::Lake => "#40d6cf",
            Regions::Ocean => "#3d9dbd",
            Regions::Burpl => "#ab567e",
            Regions::Castle => "#db2b7d"
        }
    }
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub struct Slot {
    pub has_queen: bool,
    pub region: Regions
}

pub mod build_state {
    pub struct Empty();
    pub struct QueensPlaced();
    pub struct RegionsFilled();
}

#[derive(Debug)]
pub struct BoardBuilder<BuildState, Rng = StdRng> {
    board: SMatrix<Slot, 8, 8>,
    rng: Rng,
    phantom_data: PhantomData<BuildState>
}

impl BoardBuilder<Empty, ThreadRng> {
    pub fn new() -> Self {
        Self::new_with_rng(thread_rng())
    }
}

impl<Rng: rand::Rng> BoardBuilder<Empty, Rng> {
    pub fn new_with_rng(rng: Rng) -> Self {
        Self {
            board: SMatrix::from_element(Slot {
                has_queen: false,
                region: Regions::Unclaimed
            }),
            rng,
            phantom_data: PhantomData
        }
    }
}

fn queen_place_helper<const N : usize>(
    previous_row: usize,
    current_filled_columns : [usize; N],
    rng: &mut impl Rng
) -> Option<[usize; N]> {
    if previous_row + 1 >= N {
        return Some(current_filled_columns);
    }

    let last_filled_column : isize = current_filled_columns[previous_row] as isize;

    let mut possible_col : Vec<_> = (0..N)
        .filter(|x| !current_filled_columns[0..(previous_row + 1)].contains(x))
        .filter(|x| *x as isize != last_filled_column - 1 && *x as isize != last_filled_column + 1)
        .collect();

    possible_col.shuffle(rng);

    for i in possible_col {
        let mut current_cols_copy = current_filled_columns.clone();

        current_cols_copy[previous_row + 1] = i;

        match queen_place_helper(previous_row + 1, current_cols_copy, rng) {
            None => continue,
            Some(stuff) => return Some(stuff)
        }
    }

    None
}

impl <Rng: rand::Rng> BoardBuilder<Empty, Rng> {

    pub fn place_queens(mut self) -> BoardBuilder<build_state::QueensPlaced, Rng> {
        let mut board = self.board;
        let mut populated_cols  = [0usize; 8];
        populated_cols[0] = self.rng.gen_range(0..8);

        let queen_placements = queen_place_helper(0, populated_cols, &mut self.rng)
            .expect("There should exist a valid random board for all boards");

        for idx in queen_placements.into_iter().enumerate() {
            board[idx].has_queen = true;
        }

        BoardBuilder::<build_state::QueensPlaced, Rng> { board, rng: self.rng, phantom_data: PhantomData }
    }
}

impl <Rng: rand::Rng> BoardBuilder<build_state::QueensPlaced, Rng> {
    pub fn flood_fill(mut self) -> BoardBuilder<build_state::RegionsFilled, Rng> {
        let mut remaining_colors = {
            let mut tmp_vec = vec![
                Regions::LAVA,
                Regions::Hinterlands,
                Regions::Farms,
                Regions::MySwamp,
                Regions::Lake,
                Regions::Ocean,
                Regions::Burpl,
                Regions::Castle
            ];

            tmp_vec.shuffle(&mut self.rng);

            tmp_vec
        };

        let mut board = self.board;

        for b in board.iter_mut() {
            if b.has_queen {
                b.region = remaining_colors.pop().expect("There should be at least one color for a queen");
            }
        }

        loop {
            // flood fill each region
            let (x, y) = board.shape();
            for i in 0..x {
                for j in 0..y {
                    if !matches!(board[(i,j)].region, Regions::Unclaimed) {
                    let mut x_mod : isize = 0;
                    let mut y_mod : isize = 0;
                    *(if self.rng.gen_bool(0.5) { &mut x_mod } else { &mut y_mod }) = if self.rng.gen_bool(0.5) { -1 } else { 1 };

                    if let Some(modifiied) = (i as isize).checked_add(x_mod).map(|x| x as usize) {
                        if modifiied < x {
                            if let Some(modijied) = (j as isize).checked_add(y_mod).map(|x| x as usize) {
                                if modijied < y && matches!(board[(modifiied, modijied)].region, Regions::Unclaimed)
                                {
                                    board[(modifiied, modijied)].region = board[(i, j)].region;
                                }
                            }
                        }
                    }
                    }
                }
            }

            // assert all
            if board.iter().all(|x| !matches!(x.region, Regions::Unclaimed)) {
                break BoardBuilder::<build_state::RegionsFilled, Rng> { board, phantom_data: PhantomData, rng: self.rng };
            }
        }
    }
}

impl <Rng: rand::Rng> BoardBuilder<build_state::RegionsFilled, Rng> {
    pub fn validate_unique(self) -> Option<ValidBoard> {
        let board = self.board;

        // TODO validate this board has a unique_solution
        if queen_place_solver_from_start(&self.board) {
            Some(ValidBoard { board })
        } else { None }
    }
}

pub struct ValidBoard {
    board: SMatrix<Slot, 8, 8>
}

impl Deref for ValidBoard {
    type Target = SMatrix<Slot, 8, 8>;

    fn deref(&self) -> &Self::Target {
        &self.board
    }
}

pub fn validate_grid<const N : usize>(board: SMatrix<Slot, N, N>, user: SMatrix<bool, N, N>) -> SMatrix<bool, N, N> {
    let column_invalids : Vec<_> = user.column_iter().map(|x| x.iter().map(|x| if *x { 1 } else { 0 }).sum::<usize>()).map(|x| x > 1).collect();
    let row_invalids : Vec<_> = user.row_iter().map(|x| x.iter().map(|x| if *x { 1 } else { 0 }).sum::<usize>()).map(|x| x > 1).collect();

    let region_invalids: HashSet<Regions> = user
        .iter()
        .zip(board.iter())
        .filter_map(|(x, y)| x.then_some(y.region))
        .fold(HashMap::new(), |mut map, r| { *map.entry(r).or_insert(0) += 1; map })
        .into_iter().filter_map(|(k,v)| (v > 1).then_some(k) ).collect();

    let queen_invalids: SMatrix<bool, N, N> = SMatrix::from_fn(|i, j| {
        if user[(i,j)] {
            let locality_top@(lt_x, lt_y) = (
                i.saturating_sub(1),
                j.saturating_sub(1)
            );
            let (lb_x, lb_y) = (
                if (i + 1) < N { i + 1 } else { N - 1 },
                if (j + 1) < N { j + 1 } else { N - 1 }
            );

            let locality_size = (
                lb_x - lt_x + 1,
                lb_y - lt_y + 1
            );

            user.view(locality_top, locality_size).iter().cloned().map(usize::from).sum::<usize>() > 1
        } else {
            false
        }
    });

    SMatrix::from_fn(|i, j| {
        row_invalids[i] || column_invalids[j] || queen_invalids[(i,j)] || region_invalids.contains(&board[(i, j)].region)
    })
}

fn queen_place_solver<const N : usize>(
    previous_row: usize,
    current_filled_columns : [(usize, Regions); N],
    board_ref: &SMatrix<Slot, N, N>
) -> Vec<[(usize, Regions); N]> {
    if previous_row + 1 >= N {
        return vec![current_filled_columns];
    }

    let last_filled_column : isize = current_filled_columns[previous_row].0 as isize;

    let possible_col : Vec<_> = (0..N)
        .filter(|&x| !current_filled_columns[0..(previous_row + 1)].iter().map(|(y,_)| y).any(|&y| y == x))
        .filter(|&x| x as isize != last_filled_column - 1 && x as isize != last_filled_column + 1)
        .filter(|&x| !current_filled_columns[0..(previous_row + 1)].iter().map(|&(_,y)| y).any(|y| y == board_ref[(x, previous_row + 1)].region))
        .collect();

    let mut total_result = vec![];

    for i in possible_col {
        let mut current_cols_copy = current_filled_columns;

        current_cols_copy[previous_row + 1] = (i, board_ref[(i, previous_row + 1)].region);

        let mut inner_result = queen_place_solver(previous_row + 1, current_cols_copy, board_ref);

        total_result.append(&mut inner_result);

        if total_result.len() > 1 {
            // Short circuit
            break;
        }
    }

    total_result
}

fn queen_place_solver_from_start<const N : usize>(
    board_ref: &SMatrix<Slot, N, N>
) -> bool {

    let x : usize = (0..N).into_par_iter().map(|i| {
        let mut root = [(0usize, Regions::Unclaimed); N];
        root[0] = (i, board_ref[(i, 0)].region);

        let inter = queen_place_solver(
            0,
            root,
            board_ref
        );

        inter.len()
    }).sum();

    x == 1
}
