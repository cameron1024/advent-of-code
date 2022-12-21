const INPUT: &str = include_str!("../inputs/day_8");

fn main() {
    let result = part_1(INPUT);
    println!("{result}");
}

fn part_1(s: &str) -> usize {
    let grid = Grid::from_str(s);
    println!("{grid:?}");
    grid.count_visible()
}

#[derive(Debug)]
struct Grid {
    width: usize,
    height: usize,
    trees: Vec<u8>,
}

impl Grid {
    fn from_str(s: &str) -> Self {
        let mut width = 0;
        let mut height = 0;
        let mut trees = vec![];

        for line in s.trim().lines() {
            height += 1;
            width = line.len();

            for c in line.chars() {
                trees.push(c.to_digit(10).unwrap() as u8);
            }
        }

        Grid {
            width,
            height,
            trees,
        }
    }

    fn get(&self, x: usize, y: usize) -> u8 {
        self.trees[x + y * self.width]
    }


    fn count_visible(&self) -> usize {
        let mut count = 0;

        for x in 0..self.width {
            for y in 0..self.height {
                count += if self.is_visible(x, y) { 1 } else { 0 };
            }
        }

        count
    }

    fn is_visible(&self, x: usize, y: usize) -> bool {
        if x == 0 || x == self.width - 1 {
            return true;
        }

        if y == 0 || y == self.height - 1 {
            return true;
        }

        self.is_visible_from_top(x, y)
            || self.is_visible_from_left(x, y)
            || self.is_visible_from_right(x, y)
            || self.is_visible_from_bottom(x, y)
    }

    fn is_visible_from_left(&self, x: usize, y: usize) -> bool {
        (0..x).all(|x_2| self.get(x, y) > self.get(x_2, y))
    }

    fn is_visible_from_top(&self, x: usize, y: usize) -> bool {
        (0..y).all(|y_2| self.get(x, y) > self.get(x, y_2))
    }

    fn is_visible_from_right(&self, x: usize, y: usize) -> bool {
        ((x + 1)..self.width).all(|x_2| self.get(x, y) > self.get(x_2, y))
    }

    fn is_visible_from_bottom(&self, x: usize, y: usize) -> bool {
        ((y + 1)..self.height).all(|y_2| self.get(x, y) > self.get(x, y_2))
    }
}
