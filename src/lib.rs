use std::collections::HashSet;

mod random;

pub type Position = (usize, usize);

pub enum OpenResult {
    Mine,
    NoMine(u8),
}

#[derive(Debug)]
pub struct Minesweeper {
    width: usize,
    height: usize,
    open_field: HashSet<Position>,
    mines: HashSet<Position>,
    flagged_fields: HashSet<Position>,
}

impl Minesweeper {
    // add code here
    pub fn new(width: usize, height: usize, mine_count: usize) -> Minesweeper {
        Minesweeper {
            width,
            height,
            open_field: HashSet::new(),
            mines: {
                let mut mines = HashSet::new();

                while mines.len() < mine_count {
                    mines.insert((
                        random::random_range(0, width),
                        random::random_range(0, height),
                    ));
                }

                mines
            },
            flagged_fields: HashSet::new(),
        }
    }
    pub fn neighbours(&self, (x, y): Position) -> Vec<Position> {
        (x-1 ..= x+1).flat_map(|i| (y-1 ..= y+1).map(move |j| (i, j))).collect()
    }

    pub fn open(&mut self, position: Position) -> OpenResult {
        self.open_field.insert(position);

        let is_mine = self.mines.contains(&position);

        if is_mine {
            OpenResult::Mine
        } else {
            OpenResult::NoMine(0)
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::Minesweeper;

    #[test]
    fn test() {
        let ms = Minesweeper::new(9, 9, 5);

        println!("{:?}", ms);
    }
}
