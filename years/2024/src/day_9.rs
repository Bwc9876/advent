use advent_core::{day_stuff, ex_for_day, Day};

pub struct Day9;

#[derive(Clone, Debug)]
pub enum Block {
    File(usize),
    Blank,
}

#[derive(Debug, Clone)]
pub struct Disk {
    data: Vec<(Block, usize)>,
}

impl Disk {
    pub fn parse(input: &str) -> Self {
        let data = input
            .chars()
            .enumerate()
            .map(|(i, c)| {
                let size = c.to_string().parse::<usize>().unwrap();
                if i % 2 == 0 {
                    (Block::File(i / 2), size)
                } else {
                    (Block::Blank, size)
                }
            })
            .collect::<Vec<_>>();
        Self { data }
    }

    pub fn checksum(&self) -> usize {
        self.data
            .iter()
            .enumerate()
            .fold(
                (0, 0_usize),
                |(current_index, total), (_, (block, size))| match block {
                    Block::Blank => (current_index + size, total),
                    Block::File(id) => (
                        current_index + size,
                        total
                            + (current_index..current_index + size)
                                .map(|i| *id * i)
                                .sum::<usize>(),
                    ),
                },
            )
            .1
    }

    #[allow(unused)]
    pub fn display(&self) -> String {
        self.data
            .iter()
            .map(|(b, s)| match b {
                Block::Blank => ".".repeat(*s),
                Block::File(id) => id.to_string().repeat(*s),
            })
            .collect::<String>()
    }

    /// (position, size)
    pub fn iter_blanks_before(&self, i: usize) -> impl Iterator<Item = (usize, usize)> + use<'_> {
        self.data.iter().enumerate().filter_map(move |(j, b)| {
            if let (Block::Blank, b_amnt) = b
                && j < i
            {
                Some((j, *b_amnt))
            } else {
                None
            }
        })
    }

    /// If we inserted once
    /// Assumes i > j, j describes [Block::Blank], i describes [Block::File].
    pub fn swap_blocks(&mut self, i: usize, j: usize) -> bool {
        let a = self.data[i].clone();
        let b = self.data[j].clone();

        if a.1 == b.1 {
            self.data.swap(i, j);
            false
        } else {
            if a.1 > b.1 {
                self.data[i].1 -= b.1;
                self.data[j].0 = a.0.clone();
                self.data.insert(i + 1, (b.0.clone(), a.1 - b.1));
            } else {
                self.data[j] = a.clone();
                self.data[i].0 = b.0.clone();
                self.data.insert(j + 1, (b.0.clone(), b.1 - a.1));
            }

            true
        }
    }
}

impl Day for Day9 {
    day_stuff!(9, "1928", "2858", Disk);

    fn part_1(mut input: Self::Input) -> Option<String> {
        let mut i = input.data.len() - 1;
        loop {
            if let (Block::File(_), _) = input.data[i] {
                let next_blank = input.iter_blanks_before(i).next();
                if let Some((j, _)) = next_blank {
                    if input.swap_blocks(i, j) {
                        i += 1;
                    }
                }
            }
            if i == 0 {
                break;
            } else {
                i -= 1;
            }
        }

        Some(input.checksum().to_string())
    }

    fn part_2(mut input: Self::Input) -> Option<String> {
        let mut i = input.data.len() - 1;

        loop {
            if let (Block::File(_), size) = input.data[i] {
                let next_blank = input
                    .iter_blanks_before(i)
                    .find(|(_, b_amnt)| *b_amnt >= size);
                if let Some((j, _)) = next_blank {
                    if input.swap_blocks(i, j) {
                        i += 1;
                    }
                }
            }
            if i == 0 {
                break;
            } else {
                i -= 1;
            }
        }

        Some(input.checksum().to_string())
    }

    fn parse_input(input: &str) -> Self::Input {
        Disk::parse(input.trim())
    }
}
