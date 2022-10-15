use std::{
    collections::HashSet,
    fmt::{Display, Write},
};

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

impl Display for Minesweeper {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for y in 0..self.height {
            for x in 0..self.width {
                let pos = (x, y);

                if !self.open_field.contains(&pos) {
                    if self.flagged_fields.contains(&pos) {
                        f.write_str("🚩 ")?;
                    } else {
                        f.write_str("⬜ ")?;
                    }
                } else if self.mines.contains(&pos) {
                    f.write_str("💣 ")?;
                } else {
                    write!(f, " {} ", self.neighbouring_mines(pos))?;
                }
            }
            f.write_char('\n')?;
        }
        Ok(())
    }
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
    pub fn iter_neighbours(&self, (x, y): Position) -> impl Iterator<Item = Position> {
        // add code here
        let width = self.width;
        let height = self.height;

        (x.max(1) - 1..=(x + 1).min(width - 1))
            .flat_map(move |i| (y.max(1) - 1..=(y + 1).min(height - 1)).map(move |j| (i, j)))
            .filter(move |&pos| pos != (x, y))
    }

    pub fn neighbouring_mines(&self, pos: Position) -> u8 {
        self.iter_neighbours(pos)
            .filter(|pos| self.mines.contains(pos))
            .count() as u8
    }

    pub fn open(&mut self, pos: Position) -> Option<OpenResult> {
        if self.flagged_fields.contains(&pos) {
            return None;
        }

        self.open_field.insert(pos);

        let is_mine = self.mines.contains(&pos);

        if is_mine {
            Some(OpenResult::Mine)
        } else {
            Some(OpenResult::NoMine(0))
        }
    }
    pub fn toogle_flag(&mut self, pos: Position) {
        if self.open_field.contains(&pos) {
            return;
        }

        if self.flagged_fields.contains(&pos) {
            self.flagged_fields.remove(&pos);
        } else {
            self.flagged_fields.insert(pos);
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::Minesweeper;

    #[test]
    fn test() {
        let mut ms = Minesweeper::new(9, 9, 5);
        ms.open((5, 5));
        ms.toogle_flag((6, 6));

        println!("{:?}", ms);
    }
}
