use rand::Rng;
use rand::rngs::ThreadRng;

pub struct SnakeGame {
    board: Vec<u8>,
    snake: Vec<(u8, u8)>,
    apple: (u8, u8),
    rng: ThreadRng,
    init_len: u8,
    board_size: u8,
    move_stack: Vec<u8>,
    cnt: u8
}

impl SnakeGame {
    pub fn init(board_size: u8, snake_size: u8) -> Self {
        let total_size: u8 = board_size * board_size;
        let mut board: Vec<u8> = vec![0; total_size as usize];
        let snake: Vec<(u8, u8)> = vec![(board_size / 2, board_size / 2); snake_size as usize];
        
        board[(board_size / 2 * board_size + board_size / 2) as usize] = 2;

        let mut rng = rand::thread_rng();

        let rand_x: u8 = rng.gen_range(0..board_size);
        let rand_y: u8 = rng.gen_range(0..board_size);

        let apple: (u8, u8) = (rand_x, rand_y);

        let mut move_stack: Vec<u8> = Vec::new();

        board[(rand_y * board_size + rand_x) as usize] = 1;

        Self {
            board,
            snake, // 2
            apple, // 1
            rng,
            init_len: snake_size,
            board_size,
            move_stack,
            cnt: 0
        }
    }

    pub fn update(&mut self, dir: u8) {
        self.move_snake(dir);
        self.update_board();
        self.check_apple(self.snake[0]);

        if self.cnt > 32 {
            self.cnt = 0;
        }
    }

    fn update_board(&mut self) {
        self.board = vec![0; (self.board_size * self.board_size) as usize];
        
        self.board[(self.apple.1 * self.board_size + self.apple.0) as usize] = 1;

        for seg in self.snake.iter() {
            let (sx, sy) = *seg;
            self.board[(sy * self.board_size + sx) as usize] = 2;
        }
    }

    fn move_snake(&mut self, dir: u8) {
        let updated_head = self.update_dir(dir, self.snake[0]);

        let new_head = (updated_head.0 as u8, updated_head.1 as u8);

        self.snake.insert(0, new_head);

        if self.snake.len() > self.init_len as usize {
            self.snake.pop();
        }
    }

    fn check_apple(&mut self, pt: (u8, u8)) {
        if self.apple == pt {
            let mut empty_cells = Vec::new();

            self.cnt += 1;
    
            for y in 0..self.board_size {
                for x in 0..self.board_size {
                    if !self.snake.contains(&(x, y)) {
                        empty_cells.push((x, y));
                    }
                }
            }
    
            if let Some(&(rand_x, rand_y)) = empty_cells.get(self.rng.gen_range(0..empty_cells.len())) {
                self.apple = (rand_x, rand_y);
            }
        }
    }

    fn is_out_of_bounds(&self, pt: (i16, i16)) -> bool {
        pt.0 < 0 || pt.0 >= self.board_size as i16 || pt.1 < 0 || pt.1 >= self.board_size as i16
    }

    fn update_dir(&self, dir: u8, pt: (u8, u8)) -> (i16, i16) {
        let pt_x: i16 = pt.0 as i16;
        let pt_y: i16 = pt.1 as i16;

        match dir {
            1 => (pt_x, pt_y.saturating_sub(1)), // north
            2 => (pt_x.saturating_add(1), pt_y), // east
            3 => (pt_x, pt_y.saturating_add(1)), // south
            4 => (pt_x.saturating_sub(1), pt_y), // west
            _ => (pt_x, pt_y)
        }
    }

    pub fn get_dir(&mut self) -> u8 {
        if (self.move_stack.is_empty()) {
            self.fill_move_stack();
        }

        let next_move = self.move_stack.pop();
        next_move.expect("REASON")
        
    }

    fn fill_move_stack(&mut self) {
        let apple_x = self.apple.0 as i16;
        let apple_y = self.apple.1 as i16;
        let head_x = self.snake[0].0 as i16;
        let head_y = self.snake[0].1 as i16;

        let dx = head_x - apple_x;
        let dy = head_y - apple_y;

        let x_mov = dx.abs();
        let y_mov = dy.abs();

        // fill stack

        for x in 1..=x_mov {
            if dx > 0 {
                self.move_stack.push(4);
            } else if dx < 0 {
                self.move_stack.push(2);
            }
        }

        for x in 1..=y_mov {
            if dy > 0 {
                self.move_stack.push(1);
            } else if dy < 0 {
                self.move_stack.push(3);
            }
        }

    }

    pub fn get_board(&self) -> Vec<u8>{
        self.board.clone()
    }

    pub fn get_cnt(&self) -> u8 {
        self.cnt.clone()
    }
}