use ggez::event::{self, EventHandler, KeyCode, MouseButton};
use ggez::graphics::{self, Color, DrawMode, Rect};
use ggez::{Context, ContextBuilder, GameError};

const WIDTH: f32 = 900.0;
const ROWS: usize = 50;
const COLORS: [(u8, u8, u8); 9] = [
    (255, 0, 0),       // Red
    (0, 255, 0),       // Green
    (0, 0, 255),       // Blue
    (255, 255, 255),   // White
    (0, 0, 0),         // Black
    (128, 0, 128),     // Purple
    (255, 165, 0),     // Orange
    (64, 224, 208),    // Turquoise
    (128, 128, 128),   // Gray
];

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum SpotType {
    Empty,
    Start,
    End,
    Barrier,
    Open,
    Closed,
    Path,
}

#[derive(Clone, Debug)]
struct Spot {
    row: usize,
    col: usize,
    width: f32,
    spot_type: SpotType,
    neighbors: Vec<(usize, usize)>,
}

impl Spot {
    fn new(row: usize, col: usize, width: f32) -> Self {
        Self {
            row,
            col,
            width,
            spot_type: SpotType::Empty,
            neighbors: Vec::new(),
        }
    }

    fn reset(&mut self) {
        self.spot_type = SpotType::Empty;
    }

    fn update_neighbors(&mut self, grid: &[Vec<Spot>]) {
        self.neighbors.clear();
        let rows = grid.len();
        let cols = grid[0].len();

        // Check valid neighbors (up, down, left, right)
        if self.row > 0 && grid[self.row - 1][self.col].spot_type != SpotType::Barrier {
            self.neighbors.push((self.row - 1, self.col));
        }
        if self.row < rows - 1 && grid[self.row + 1][self.col].spot_type != SpotType::Barrier {
            self.neighbors.push((self.row + 1, self.col));
        }
        if self.col > 0 && grid[self.row][self.col - 1].spot_type != SpotType::Barrier {
            self.neighbors.push((self.row, self.col - 1));
        }
        if self.col < cols - 1 && grid[self.row][self.col + 1].spot_type != SpotType::Barrier {
            self.neighbors.push((self.row, self.col + 1));
        }
    }
}

struct MainState {
    grid: Vec<Vec<Spot>>,
    start: Option<(usize, usize)>,
    end: Option<(usize, usize)>,
    width: f32,
}

impl MainState {
    fn new() -> Self {
        let width = WIDTH / ROWS as f32;
        let mut grid = Vec::new();
        for row in 0..ROWS {
            let mut row_vec = Vec::new();
            for col in 0..ROWS {
                row_vec.push(Spot::new(row, col, width));
            }
            grid.push(row_vec);
        }

        Self {
            grid,
            start: None,
            end: None,
            width,
        }
    }

    fn draw_grid(&self, ctx: &mut Context) -> Result<(), GameError> {
        for row in &self.grid {
            for spot in row {
                let color = match spot.spot_type {
                    SpotType::Empty => Color::from_rgb(255, 255, 255),
                    SpotType::Start => Color::from_rgb(255, 165, 0),
                    SpotType::End => Color::from_rgb(64, 224, 208),
                    SpotType::Barrier => Color::from_rgb(0, 0, 0),
                    SpotType::Open => Color::from_rgb(0, 255, 0),
                    SpotType::Closed => Color::from_rgb(255, 0, 0),
                    SpotType::Path => Color::from_rgb(128, 0, 128),
                };

                let rect = Rect::new(
                    spot.col as f32 * self.width,
                    spot.row as f32 * self.width,
                    self.width,
                    self.width,
                );
                let mesh = graphics::Mesh::new_rectangle(ctx, DrawMode::fill(), rect, color)?;
                graphics::draw(ctx, &mesh, graphics::DrawParam::default())?;
            }
        }
        Ok(())
    }
}

impl EventHandler<GameError> for MainState {
    fn update(&mut self, _ctx: &mut Context) -> Result<(), GameError> {
        Ok(())
    }
    
    fn draw(&mut self, ctx: &mut Context) -> Result<(), GameError> {
        graphics::clear(ctx, Color::from_rgb(200, 200, 200));
        self.draw_grid(ctx)?;
        graphics::present(ctx)?;
        Ok(())
    }

    fn mouse_button_down_event(&mut self, _ctx: &mut Context, button: MouseButton, x: f32, y: f32) {
        let row = (y / self.width) as usize;
        let col = (x / self.width) as usize;

        if row < ROWS && col < ROWS {
            let spot = &mut self.grid[row][col];

            match button {
                MouseButton::Left => {
                    if self.start.is_none() {
                        self.start = Some((row, col));
                        spot.spot_type = SpotType::Start;
                    } else if self.end.is_none() {
                        self.end = Some((row, col));
                        spot.spot_type = SpotType::End;
                    } else {
                        spot.spot_type = SpotType::Barrier;
                    }
                }
                MouseButton::Right => {
                    spot.reset();
                    if self.start == Some((row, col)) {
                        self.start = None;
                    } else if self.end == Some((row, col)) {
                        self.end = None;
                    }
                }
                _ => {}
            }
        }
    }

fn key_down_event(&mut self, ctx: &mut Context, keycode: KeyCode, _keymods: event::KeyMods, _repeat: bool) {
    match keycode {
        KeyCode::Space => {
            self.a_star();
        }
        KeyCode::C => {
            for row in &mut self.grid {
                for spot in row {
                    spot.reset();
                }
            }
            self.start = None;
            self.end = None;
        }
        KeyCode::Escape => {
            event::quit(ctx);
        }
        _ => {}
    }
}

}

fn main() -> Result<(), GameError> {
    let (mut ctx, mut event_loop) = ContextBuilder::new("astar", "Author")
        .window_setup(ggez::conf::WindowSetup::default().title("A* Algorithm"))
        .window_mode(ggez::conf::WindowMode::default().dimensions(WIDTH, WIDTH))
        .build()?;

    let mut state = MainState::new();
    event::run(ctx, event_loop, state)
}
