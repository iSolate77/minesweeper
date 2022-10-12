use std::collections::HashSet;

mod random;

type Position = (usize, usize);

#[derive(Debug)]
struct Minesweeper {
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
