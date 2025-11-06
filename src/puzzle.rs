
use rand::Rng;

#[derive(Clone, Copy, PartialEq)]
pub enum CellState {
    Unknown,
    X,
    O,
}

pub struct Puzzle {
    pub height: u32,
    pub width: u32,
    pub state: Vec<Vec<CellState>>,
    pub solution: Vec<Vec<bool>>,
    pub row_hints: Vec<Vec<u32>>,
    pub col_hints: Vec<Vec<u32>>,
    pub cursor: (u32, u32),
}

pub fn create_puzzle(width: u32, height: u32) -> Puzzle {
    let mut rng = rand::thread_rng();
    let mut solution = vec![vec![false; width as usize]; height as usize];
    for y in 0..height {
        for x in 0..width {
            solution[y as usize][x as usize] = rng.gen_range(0..=1) == 1;
        }
    }

    let (row_hints, col_hints) = calculate_hints(&solution);

    Puzzle {
        width,
        height,
        state: vec![vec![CellState::Unknown; width as usize]; height as usize],
        solution,
        row_hints,
        col_hints,
        cursor: (0, 0),
    }
}

pub fn calculate_hint_for_line(line: &[bool]) -> Vec<u32> {
    let mut hints = vec![];
    let mut count = 0;
    for &cell in line {
        if cell {
            count += 1;
        } else {
            if count > 0 {
                hints.push(count);
            }
            count = 0;
        }
    }
    if count > 0 {
        hints.push(count);
    }
    if hints.is_empty() {
        hints.push(0);
    }
    hints
}

pub fn calculate_hints(solution: &Vec<Vec<bool>>) -> (Vec<Vec<u32>>, Vec<Vec<u32>>) {
    let height = solution.len();
    if height == 0 {
        return (vec![], vec![]);
    }
    let width = solution[0].len();
    let row_hints = solution
        .iter()
        .map(|row| calculate_hint_for_line(row))
        .collect();

    let mut col_hints = vec![];
    for x in 0..width {
        let col: Vec<bool> = (0..height).map(|y| solution[y][x]).collect();
        col_hints.push(calculate_hint_for_line(&col));
    }

    (row_hints, col_hints)
}
