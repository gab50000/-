pub fn solve_a() {
    let result = include_str!("../data/05.txt")
        .lines()
        .map(BoardingPass::from_str)
        .map(|pass| pass.get_id())
        .max();
    for r in result {
        println!("{:?}", r);
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
struct BoardingPass {
    row: u32,
    col: u32,
}

impl BoardingPass {
    fn from_str(input: &str) -> Self {
        let (mut min_row, mut min_col) = (0u32, 0u32);
        let (mut max_row, mut max_col) = (127u32, 7u32);

        for char in input.bytes() {
            match char {
                b'F' => max_row = min_row + (max_row - min_row) / 2,
                b'B' => min_row = (min_row + max_row) / 2 + 1,
                b'L' => max_col = min_col + (max_col - min_col) / 2,
                b'R' => min_col = (min_col + max_col) / 2 + 1,
                _ => panic!("Unexpected input"),
            }
        }

        Self {
            row: min_row,
            col: min_col,
        }
    }

    fn get_id(&self) -> u32 {
        self.row * 8 + self.col
    }
}

#[test]
fn test_boarding_pass() {
    assert_eq!(
        BoardingPass::from_str("FBFBBFFRLR"),
        BoardingPass { row: 44, col: 5 }
    );
}
