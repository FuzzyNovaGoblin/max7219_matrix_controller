#[derive(Debug, Clone)]
pub struct Letter {
    bits: [Vec<bool>; 8],
    width: u8,
    u8_grid: [u8; 8],
}

impl Letter {
    pub fn new(bits: [Vec<bool>; 8], width: u8) -> Letter {
        let mut u8_grid = [0; 8];

        for (i, r) in bits.iter().enumerate() {
            let mut num = 0;
            for j in 0..8 {
                num <<= 1;
                if let Some(true) = r.get(j) {
                    num += 1;
                }
            }
            u8_grid[i] = num;
        }

        Letter {
            bits,
            width,
            u8_grid,
        }
    }

    pub fn get(&self, x: usize, y: usize) -> bool {
        self.bits[y][x]
    }

    pub fn get_u8_grid(&self) -> &[u8; 8] {
        &self.u8_grid
    }

    pub fn get_width(&self) -> &u8 {
        &self.width
    }
}
