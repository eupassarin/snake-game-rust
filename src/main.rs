use rand::Rng;
use piston_window::{ *, types::Color };

#[derive(PartialEq)]
pub enum Direction {
    Up,
    Down,
    Left,
    Right,
}

const WINDOW_TITLE: &str = "SNAKE GAME";
const SPACE_TO_PAUSE: &str = "press SPACE to pause";
const PAUSED: &str = "PAUSED";
const SPACE_TO_CONTINUE: &str = "press SPACE to continue";
const GAMEOVER: &str = "GAME OVER!";
const ENTER_TO_RESTART: &str = "press ENTER to restart";

const BLOCK_SIZE: f64 = 20.0;
const BOARD_WIDTH: f64 = 25.0;
const BOARD_HEIGHT: f64 = 25.0;

const FIELD_COLOR: Color = [0.250, 0.066, 0.337, 1.0];
const FOOD_COLOR: Color = [0.9, 0.49, 0.13, 1.0];
const SNAKE_COLOR: Color = [1.0, 0.97, 0.0, 1.0];
const BORDER_COLOR: Color = [0.741, 0.765, 0.78, 1.0];
const GAMEOVER_COLOR: Color = [0.91, 0.3, 0.24, 0.5];
const MOVING_PERIOD: f64 = 0.2;

pub struct Game {
    snake: Snake,
    food: Option<Block>,
    is_game_over: bool,
    refresh_time: f64,
    is_stopped: bool,
    last_block_removed: Option<Block>,
    score: i32,
}
impl Game {
    pub fn new() -> Game {
        Game {
            snake: Snake::new(),
            food: Block::new(15.0, 15.0).into(),
            is_game_over: false,
            refresh_time: 0.0,
            is_stopped: false,
            last_block_removed: None,
            score: 0,
        }
    }
    pub fn handle_keypress(&mut self, event: &Event) {
        if let Some(Button::Keyboard(key)) = event.press_args() {
            match key {
                Key::Up if self.snake.direction != Direction::Down => {
                    self.snake.direction = Direction::Up;
                }
                Key::Down if self.snake.direction != Direction::Up => {
                    self.snake.direction = Direction::Down;
                }
                Key::Left if self.snake.direction != Direction::Right => {
                    self.snake.direction = Direction::Left;
                }
                Key::Right if self.snake.direction != Direction::Left => {
                    self.snake.direction = Direction::Right;
                }
                Key::Return if self.is_game_over => {
                    *self = Game::new();
                }
                Key::Space => {
                    self.is_stopped = !self.is_stopped;
                }
                _ => {}
            }
        }
    }
    pub fn generate_random_food(&mut self) {
        let new_food = Block::random();
        if self.snake.body.contains(&new_food) {
            self.generate_random_food();
        }
        self.food = new_food.into();
    }
    pub fn update(&mut self, delta_time: f64) {
        if self.is_game_over || self.is_stopped {
            return;
        }
        if self.refresh_time > MOVING_PERIOD {
            self.move_snake();
            if self.is_eating() {
                self.generate_random_food();
                self.restore_tail();
            }
            self.check_if_snake_alive();
            self.refresh_time = 0.0;
        }
        self.refresh_time += delta_time;
    }
    pub fn move_snake(&mut self) {
        let mut new_head = self.snake.body[0].clone();
        match self.snake.direction {
            Direction::Up => {
                new_head.y -= 1.0;
            }
            Direction::Down => {
                new_head.y += 1.0;
            }
            Direction::Left => {
                new_head.x -= 1.0;
            }
            Direction::Right => {
                new_head.x += 1.0;
            }
        }
        self.snake.body.insert(0, new_head);
        self.last_block_removed = self.snake.body.pop().into();
    }
    pub fn is_eating(&mut self) -> bool {
        let head = &self.snake.body[0];
        if head.eq(&self.food.unwrap()) {
            self.score += 1;
            return true;
        }
        false
    }
    pub fn restore_tail(&mut self) {
        if let Some(block) = self.last_block_removed {
            self.snake.body.push(block);
        }
    }
    pub fn check_if_snake_alive(&mut self) {
        let head = &self.snake.body[0];
        if
            head.x < 1.0 ||
            head.x > BOARD_WIDTH - 2.0 ||
            head.y < 1.0 ||
            head.y > BOARD_HEIGHT - 2.0 ||
            self.snake.body[1..].contains(head)
        {
            self.is_game_over = true;
        }
    }
    
    pub fn draw_board(&mut self, c: &Context, g: &mut G2d, glyphs: &mut Glyphs) {
        Game::draw_border(c, g);
        if self.is_game_over {
            self.draw_gameover(g, glyphs, c);
        } else {
            self.draw_base_board(g, glyphs, c);
            self.draw_pause_info(glyphs, c, g);
        }
    }
    fn draw_border(c: &Context, g: &mut G2d,) {
        rectangle(BORDER_COLOR, [0.0, 0.0, BOARD_WIDTH * BLOCK_SIZE, BLOCK_SIZE], c.transform, g);
        rectangle(
            BORDER_COLOR,
            [0.0, (BOARD_HEIGHT - 1.0) * BLOCK_SIZE, BOARD_WIDTH * BLOCK_SIZE, BLOCK_SIZE],
            c.transform,
            g
        );
        rectangle(BORDER_COLOR, [0.0, 0.0, BLOCK_SIZE, BOARD_HEIGHT * BLOCK_SIZE], c.transform, g);
        rectangle(
            BORDER_COLOR,
            [(BOARD_WIDTH - 1.0) * BLOCK_SIZE, 0.0, BLOCK_SIZE, BOARD_HEIGHT * BLOCK_SIZE],
            c.transform,
            g
        );
    }
    fn draw_base_board(&mut self, g: &mut G2d, glyphs: &mut Glyphs, c: &Context) {
        clear(FIELD_COLOR, g);
        text::Text
            ::new_color([1.0, 1.0, 1.0, 0.7], 30)
            .draw(
                &self.score.to_string(),
                glyphs,
                &c.draw_state,
                c.transform.trans(230.0, 60.0),
                g
            )
            .unwrap();
        text::Text
            ::new_color([1.0, 1.0, 1.0, 0.8], 30)
            .draw(WINDOW_TITLE, glyphs, &c.draw_state, c.transform.trans(130.0, 250.0), g)
            .unwrap();
    }
    fn draw_pause_info(&mut self, glyphs: &mut Glyphs, c: &Context, g: &mut G2d) {
        if !self.is_stopped {
            text::Text
                ::new_color([1.0, 1.0, 1.0, 0.5], 18)
                .draw(
                    SPACE_TO_PAUSE,
                    glyphs,
                    &c.draw_state,
                    c.transform.trans(115.0, 280.0),
                    g
                )
                .unwrap();
        } else {
            text::Text
                ::new_color([1.0, 1.0, 1.0, 0.9], 20)
                .draw(PAUSED, glyphs, &c.draw_state, c.transform.trans(190.0, 90.0), g)
                .unwrap();
            text::Text
                ::new_color([1.0, 1.0, 1.0, 0.7], 18)
                .draw(
                    SPACE_TO_CONTINUE,
                    glyphs,
                    &c.draw_state,
                    c.transform.trans(105.0, 280.0),
                    g
                )
                .unwrap();
        }
    }
    fn draw_gameover(&mut self, g: &mut G2d, glyphs: &mut Glyphs, c: &Context) {
        clear(GAMEOVER_COLOR, g);
        self.food = None;
        self.snake.body.clear();
        let mut score: String = String::from("SCORE: ");
        score.push_str(&self.score.to_string());
        text::Text
            ::new_color([0.0, 0.0, 0.0, 1.0], 20)
            .draw(
                &score,
                glyphs,
                &c.draw_state,
                c.transform.trans(180.0, 200.0),
                g
            )
            .unwrap();
        text::Text
            ::new_color([0.0, 0.0, 0.0, 1.0], 30)
            .draw(GAMEOVER, glyphs, &c.draw_state, c.transform.trans(130.0, 250.0), g)
            .unwrap();
        text::Text
            ::new_color([0.0, 0.0, 0.0, 0.8], 18)
            .draw(
                ENTER_TO_RESTART,
                glyphs,
                &c.draw_state,
                c.transform.trans(100.0, 280.0),
                g
            )
            .unwrap();
    }
    pub fn draw_food(&self, c: &Context, g: &mut G2d) {
        if let Some(food) = self.food {
            food.draw(FOOD_COLOR, c, g);
        }
    }
    pub fn draw_snake(&self, c: &Context, g: &mut G2d) {
        self.snake.body.iter().for_each(|block| {
            block.draw(SNAKE_COLOR, c, g);
        });
    }
}

pub struct Snake {
    body: Vec<Block>,
    direction: Direction,
}
impl Snake {
    pub fn new() -> Snake {
        Snake {
            body: vec![Block::new(5.0, 5.0), Block::new(4.0, 5.0), Block::new(3.0, 5.0)],
            direction: Direction::Down,
        }
    }
}

#[derive(Clone, PartialEq, Copy)]
pub struct Block {
    x: f64,
    y: f64,
}
impl Block {
    pub fn new(x: f64, y: f64) -> Block {
        Block { x, y }
    }
    pub fn random() -> Block {
        let mut rng = rand::thread_rng();
        let x = rng.gen_range(1..(BOARD_WIDTH as i32) - 1) as f64;
        let y = rng.gen_range(1..(BOARD_HEIGHT as i32) - 1) as f64;
        Block::new(x, y)
    }
    pub fn draw(&self, color: [f32; 4], c: &Context, g: &mut G2d) {
        rectangle(
            color,
            [self.x * BLOCK_SIZE, self.y * BLOCK_SIZE, BLOCK_SIZE, BLOCK_SIZE],
            c.transform,
            g
        );
    }
}

fn main() {
    let mut window = create_window();
    let mut glyphs = window.load_font("retro-gaming.ttf").unwrap();

    let mut game = Game::new();
    while let Some(event) = window.next() {
        game.handle_keypress(&event);
        window.draw_2d(&event, |c, g, d| {
            game.draw_board(&c, g, &mut glyphs);
            game.draw_snake(&c, g);
            game.draw_food(&c, g);
            glyphs.factory.encoder.flush(d);
        });
        event.update(|arg| {
            game.update(arg.dt);
        });
    }
}

fn create_window() -> PistonWindow {
    WindowSettings::new(WINDOW_TITLE, [BOARD_WIDTH * BLOCK_SIZE, BOARD_HEIGHT * BLOCK_SIZE])
        .exit_on_esc(true)
        .build()
        .unwrap()
}
