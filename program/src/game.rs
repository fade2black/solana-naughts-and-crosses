use borsh::{BorshDeserialize, BorshSerialize};
use solana_program::pubkey::Pubkey;
use std::fmt;

const GRID_SIZE: usize = 3;

#[derive(BorshSerialize, BorshDeserialize, Debug, Clone, Copy, PartialEq)]
pub enum Mark {
    E = 0,
    X = 1,
    O = 2,
}
impl fmt::Display for Mark {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let ch = match self {
            Mark::E => "_|",
            Mark::X => "x|",
            Mark::O => "o|",
        };
        write!(f, "{}", ch)
    }
}

impl Default for Mark {
    fn default() -> Self {
        Mark::E
    }
}

#[derive(BorshSerialize, BorshDeserialize, Debug)]
pub struct Move {
    pub row: usize,
    pub col: usize,
    pub mark: Mark,
}

#[derive(BorshSerialize, BorshDeserialize, Debug)]
pub struct Game {
    player_1: Pubkey,
    player_2: Pubkey,
    most_recent: Option<Pubkey>,
    winner: Option<Pubkey>,
    grid: [Mark; GRID_SIZE * GRID_SIZE],
}

impl Game {
    pub fn new(player_1: Pubkey, player_2: Pubkey) -> Game {
        let most_recent: Option<Pubkey> = None;
        let winner: Option<Pubkey> = None;
        let grid = [Mark::E; GRID_SIZE * GRID_SIZE];

        Game {
            player_1,
            player_2,
            most_recent,
            winner,
            grid,
        }
    }

    pub fn play(&mut self, player: Pubkey, row: usize, col: usize, mark: Mark) {
        if self.is_over() {
            return;
        }

        let i = row * GRID_SIZE + col;
        self.grid[i] = mark;

        self.most_recent.replace(player);
        if self.is_win() {
            self.winner.replace(player);
        }
    }

    pub fn get_grid(&self) -> Vec<Vec<Mark>> {
        let mut rows = vec![];

        for i in 0..GRID_SIZE {
            let mut row = vec![];
            for j in 0..GRID_SIZE {
                row.push(self.grid[i * GRID_SIZE + j]);
            }
            rows.push(row);
        }
        rows
    }

    pub fn is_over(&self) -> bool {
        self.winner.is_some() || self.is_fully_marked()
    }

    pub fn get_winner(&self) -> Option<Pubkey> {
        self.winner
    }

    pub fn get_most_recent(&self) -> Option<Pubkey> {
        self.most_recent
    }

    fn is_win(&self) -> bool {
        // Check rows
        for i in 0..GRID_SIZE {
            let j = i * GRID_SIZE;

            if self.grid[j] != Mark::E
                && self.grid[j] == self.grid[j + 1]
                && self.grid[j + 1] == self.grid[j + 2]
            {
                return true;
            }
        }

        // Check cols
        for i in 0..GRID_SIZE {
            if self.grid[i] != Mark::E
                && self.grid[i] == self.grid[i + GRID_SIZE]
                && self.grid[i + GRID_SIZE] == self.grid[i + 2 * GRID_SIZE]
            {
                return true;
            }
        }

        // Check main diagonal
        if self.grid[0] != Mark::E && self.grid[0] == self.grid[4] && self.grid[4] == self.grid[8] {
            return true;
        }

        if self.grid[2] != Mark::E && self.grid[2] == self.grid[4] && self.grid[4] == self.grid[6] {
            return true;
        }

        false
    }

    fn is_fully_marked(&self) -> bool {
        for i in 0..GRID_SIZE {
            for j in 0..GRID_SIZE {
                if let Mark::E = self.grid[i * GRID_SIZE + j] {
                    return false;
                }
            }
        }
        true
    }
}

#[cfg(test)]
mod tests {
    const X: Mark = Mark::X;
    const O: Mark = Mark::O;
    const E: Mark = Mark::E;

    use super::*;

    fn create_game_with_grid(grid: [Mark; 9]) -> Game {
        Game {
            player_1: Pubkey::new_unique(),
            player_2: Pubkey::new_unique(),
            most_recent: None,
            winner: None,
            grid: grid,
        }
    }
    #[test]
    fn it_returns_true_when_aligned_along_rows() {
        let grid1 = [X, X, X, O, O, E, O, E, O];
        let grid2 = [E, O, O, X, X, X, O, E, O];
        let grid3 = [E, E, O, O, O, O, X, X, X];
        let grids = [grid1, grid2, grid3];

        for grid in grids {
            let game = create_game_with_grid(grid);
            assert!(game.is_win());
        }
    }

    #[test]
    fn it_returns_true_when_aligned_along_cols() {
        let grid1 = [O, X, E, O, E, X, O, X, E];
        let grid2 = [O, O, E, X, O, X, X, O, X];
        let grid3 = [X, O, O, E, E, O, X, X, O];
        let grids = [grid1, grid2, grid3];

        for grid in grids {
            let game = create_game_with_grid(grid);
            assert!(game.is_win());
        }
    }

    #[test]
    fn it_returns_true_when_aligned_along_diagonals() {
        let grid1 = [O, X, E, E, O, X, E, X, O];
        let grid2 = [O, E, X, E, X, O, X, O, E];
        let grids = [grid1, grid2];

        for grid in grids {
            let game = create_game_with_grid(grid);
            assert!(game.is_win());
        }
    }

    #[test]
    fn it_sets_winner() {
        let grid = [X, X, E, E, E, O, E, O, E];
        let mut game = create_game_with_grid(grid);
        let player = game.player_1;

        game.play(player, 0, 2, X);
        assert_eq!(game.winner, Some(player));
    }

    #[test]
    fn it_does_not_set_winner() {
        let grid = [E, X, E, E, E, O, E, O, E];
        let mut game = create_game_with_grid(grid);
        let player = game.player_1;

        game.play(player, 0, 0, X);
        assert_eq!(game.winner, None);
    }

    #[test]
    fn it_sets_most_recent_palyer() {
        let grid = [X, E, E, E, E, O, E, O, E];
        let mut game = create_game_with_grid(grid);
        let player = game.player_1;

        game.play(player, 0, 2, X);
        assert_eq!(game.most_recent, Some(player));
    }

    #[test]
    fn it_does_not_play() {
        let grid = [X, X, E, E, E, O, E, O, E];
        let mut game = create_game_with_grid(grid);
        let player_1 = game.player_1;

        game.play(player_1, 0, 2, X);
        game.play(game.player_2, 1, 0, X);
        assert_eq!(game.most_recent, Some(player_1));
    }
}
