use std::marker::PhantomData;
use std::ops::Deref;
use nalgebra::{SMatrix, SVector};
use rand::prelude::*;

#[cfg(test)]
mod tests {
    use nalgebra::SMatrix;
    use rand::{thread_rng, Rng};
    use crate::board::{validate_grid, BoardBuilder};

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

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
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
pub struct BoardBuilder<BuildState> {
    board: SMatrix<Slot, 8, 8>,
    phantom_data: PhantomData<BuildState>
}

fn queen_place_helper<const N : usize>(
    previous_row: usize,
    current_filled_columns : [usize; N]
) -> Option<[usize; N]> {
    if previous_row + 1 >= N {
        return Some(current_filled_columns);
    }

    let last_filled_column : isize = current_filled_columns[previous_row] as isize;

    let mut possible_col : Vec<_> = (0..N)
        .filter(|x| !current_filled_columns[0..(previous_row + 1)].contains(x))
        .filter(|x| *x as isize != last_filled_column - 1 && *x as isize != last_filled_column + 1)
        .collect();

    possible_col.shuffle(&mut thread_rng());

    for i in possible_col {
        let mut current_cols_copy = current_filled_columns.clone();

        current_cols_copy[previous_row + 1] = i;

        match queen_place_helper(previous_row + 1, current_cols_copy) {
            None => continue,
            Some(stuff) => return Some(stuff)
        }
    }

    None
}

impl BoardBuilder<build_state::Empty> {
    pub fn new() -> Self {
        Self {
            board: SMatrix::from_element(Slot {
                has_queen: false,
                region: Regions::Unclaimed
            }),
            phantom_data: PhantomData
        }
    }

    pub fn place_queens(self) -> BoardBuilder<build_state::QueensPlaced> {
        let mut board = self.board;
        let mut populated_cols  = [0usize; 8];
        populated_cols[0] = thread_rng().gen_range(0..8);

        let queen_placements = queen_place_helper(0, populated_cols)
            .expect("There should exist a valid random board for all boards");

        for idx in queen_placements.into_iter().enumerate() {
            board[idx].has_queen = true;
        }

        BoardBuilder::<build_state::QueensPlaced> { board, phantom_data: PhantomData }
    }
}

impl BoardBuilder<build_state::QueensPlaced> {
    pub fn flood_fill(self) -> BoardBuilder<build_state::RegionsFilled> {
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

            tmp_vec.shuffle(&mut thread_rng());

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
                    *(if random() { &mut x_mod } else { &mut y_mod }) = if random() { -1 } else { 1 };

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
                break BoardBuilder::<build_state::RegionsFilled> { board, phantom_data: PhantomData };
            }
        }
    }
}

impl BoardBuilder<build_state::RegionsFilled> {
    pub fn validate_unique(self) -> Option<ValidBoard> {
        let mut board = self.board;

        // TODO validate this board has a unique_solution


        Some(ValidBoard { board })
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

    // let region_invalids: Vec<usize> = user.iter().zip(board.iter()).map(|(x, y)| {
    //     let mut z : SVector<usize, 8> = SVector::default();
    //     if *x {
    //         z[y.region as usize] = 1
    //     }
    //     z
    // }).sum::<SVector<usize, 8>>()
    //     .into_iter().cloned().enumerate().filter_map(|(x, y)| (x > 1).then_some(y)).collect();

    let queen_invalids: SMatrix<bool, N, N> = SMatrix::from_fn(|i, j| {
        if user[(i,j)] {
            let locality_top = (
                i.saturating_sub(1),
                j.saturating_sub(1)
            );
            let locality_size = (
                if (i + 1) < N { 3 } else { 2 },
                if (j + 1) < N { 3 } else { 2 }
            );

            user.view(locality_top, locality_size).iter().cloned().map(usize::from).sum::<usize>() > 1
        } else {
            false
        }
    });

    SMatrix::from_fn(|i, j| {
        row_invalids[i] || column_invalids[j] || queen_invalids[(i,j)]
    })
}
