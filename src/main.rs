
//use oorandom::Rand32;

use ggez::input::mouse::position;
use glam::*;
use ggez::{event, Context, GameResult, graphics};
//use ggez::input::mouse::MouseButton;
use ggez::event::{KeyCode, KeyMods, MouseButton};
use ggez::graphics::{GlBackendSpec, Image, draw, Rect, DrawMode,
                     ImageGeneric, clear, MeshBuilder, present};

use std::{path, env};
use std::time::{Duration, Instant};

const BASE_RESOLUTION: (f32, f32) = (800.0, 600.0);
//const STRETCHED_RESOLUTION: (f32, f32) = ((BASE_RESOLUTION.0 / RESOLUTION.0),
//                                       (BASE_RESOLUTION.1 / RESOLUTION.1));
const PLAYER_MOVEMENT: (f32, f32) = (5.00, 5.00);

const GRID_SIZE: f32 = 50.0;

fn get_scaled_resolution(coords: (f32, f32)) -> (f32, f32) {
    ((coords.0 / BASE_RESOLUTION.0).round(),
    (coords.1 / BASE_RESOLUTION.1).round())
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
struct Direction {
    up: bool,
    down: bool,
    left: bool,
    right: bool,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
struct Grid {
    x: u8,
    y: u8,
}

impl Grid {
    fn from_position(position: Position) -> Grid {
        Grid {
            x: (position.x / GRID_SIZE) as u8,
            y: (position.y / GRID_SIZE) as u8
        }
    }
}

#[derive(Copy, Clone, Debug, Default, PartialEq)]
struct Position {
    x: f32,
    y: f32,
}

impl From<Grid> for Position {
    fn from(grid: Grid) -> Self {
        Position {
            x: GRID_SIZE * grid.x as f32,
            y: GRID_SIZE * grid.y as f32,
        }
    }

}

impl Direction {

    pub fn update_from_keycode(&mut self, key: KeyCode, down: bool) {
        match key {
            KeyCode::Up => self.up = down,
            KeyCode::Down => self.down = down,
            KeyCode::Left => self.left = down,
            KeyCode::Right => self.right = down,
            _ => (),
        };
    }
}

#[derive(Default, Copy, Clone)]
struct AnimationFrames {
    animation_frame: f32,
    animation_total_frames: f32,
    last_animation: Option<std::time::Instant>,
}

impl AnimationFrames {
    fn new(total_frames: f32) -> AnimationFrames {
        AnimationFrames {
            animation_frame: 0.0,
            animation_total_frames: total_frames,
            last_animation: Some(std::time::Instant::now()),
        }
    }
}

struct Sprite {
    texture: ImageGeneric<GlBackendSpec>,
    frames: Option<AnimationFrames>,
}

trait Animate {
    fn animate_frames(&mut self);
    fn get_animation_direction(&self) -> f32;
}

impl Animate for Player {

    fn animate_frames(&mut self) {
        // Animation movement
        if let Some(mut frames) = self.sprite.frames {
            if let Some(last_animation) = frames.last_animation {
                if last_animation.elapsed() >  Duration::new(0, 150_000_000) {
                    frames.last_animation = Some(Instant::now());
                    frames.animation_frame += 1.0 / frames.animation_total_frames;
                    if frames.animation_frame >= 1.0 {
                        frames.animation_frame = 0.0;
                    }
                }
                self.sprite.frames = Some(frames);
            }
        }
    }

    fn get_animation_direction(&self) -> f32 {
        if self.direction.up {
            0.25
        } else if self.direction.left {
            0.5
        } else if self.direction.right {
            0.75
        } else {
            0.0
        }
    }
}

struct Player {
    resolution: (f32, f32),
    draw_position: Position,
    grid_position: Grid,
    grid_destination: Grid,
    direction: Direction,
    is_moving: bool,
    sprite: Sprite
}

impl Sprite {
    
    fn new(ctx: &mut Context, texture: &str, total_frames: f32) -> Sprite {
        let new_texutre = Image::new(ctx,
                texture).unwrap();
        let frames = AnimationFrames::new(total_frames);
        Sprite {
            texture: new_texutre,
            frames: Some(frames),
        }
    }
}

impl Player {

    fn new(ctx: &mut Context, resolution: (f32, f32)) -> Player {
        Player {
            draw_position: Position::default(),
            grid_position: Grid::default(),
            grid_destination: Grid::default(),
            direction: Direction::default(),
            resolution,
            is_moving: false,
            sprite: Sprite::new(ctx, "/hero.png", 4.0),
        }
    }

    fn update(&mut self) {
        //println!("{:?} {:?}", self.grid_position, self.grid_destination);
        if self.grid_position.x != self.grid_destination.x {
            self.is_moving = true;
            self.draw_position.x += 1.0;
            self.grid_position = Grid::from_position(self.draw_position);
        } else {
            self.is_moving = false;
        }
    }

    fn move_to_posiiton(&mut self, x: f32, y: f32) {
        //self.destination.x = x;
        //self.destination.y = y;
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult<()> {
        if self.is_moving {
            self.animate_frames();
        }
        let frame_x = if let Some(frames) = self.sprite.frames {
            frames.animation_frame
        } else {
            0.0
        };
        let frame_y = self.get_animation_direction();
        let param = graphics::DrawParam::new()
        .src(graphics::Rect {x: frame_x, y: frame_y, w: 0.25, h: 0.25})
        .dest(Vec2::new(self.draw_position.x * get_scaled_resolution(self.resolution).0, 
                              self.draw_position.y * get_scaled_resolution(self.resolution).1))
        .offset(Vec2::new(0.00, 0.00))
        // Scale image based on resolution
        .scale(Vec2::new(get_scaled_resolution(self.resolution).0 / 2.0,
                                get_scaled_resolution(self.resolution).1 / 2.0));
        draw(ctx, &self.sprite.texture, param)?;
        Ok(())
    }
}

struct GameState {
    player: Player,
    resolution: (f32, f32),
    mouse_x: f32,
    mouse_y: f32,
    mouse_dx: f32,
    mouse_dy: f32,
}

impl GameState {
    pub fn new(ctx: &mut Context, resolution: (f32, f32)) -> Self {

        GameState {
            player: Player::new(ctx, resolution),
            resolution,
            mouse_x: 0.0,
            mouse_y: 0.0,
            mouse_dx: 0.0,
            mouse_dy: 0.0,
            
        }
    }
}

impl event::EventHandler<ggez::GameError> for GameState {
    fn update(&mut self, _ctx: &mut Context) -> GameResult {
        self.player.update();
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult {

        // Get scale
        let scaled_resolution = get_scaled_resolution(self.resolution);

        // Clear background
        clear(ctx, [0.0, 0.0, 0.0, 1.0].into());

        let background_rectangle = graphics::Mesh::new_rectangle(
            ctx,
            graphics::DrawMode::fill(),
            Rect::new(0.0,
                0.0, 
                BASE_RESOLUTION.0 * scaled_resolution.0, 
                BASE_RESOLUTION.1 * scaled_resolution.1),
            [0.1, 0.1, 0.1, 1.0].into(),
        )?;
        graphics::draw(ctx, &background_rectangle, (ggez::mint::Point2 { x: 0.0, y: 0.0 },))?;

        // Draw where mouse outline is
        let outline_color = [0.2, 0.2, 1.0, 1.0];
        let grid_outline = graphics::Mesh::new_rectangle(
            ctx,
            graphics::DrawMode::fill(),
            Rect::new(0.0,
                0.0, 
                GRID_SIZE * scaled_resolution.0,
                GRID_SIZE * scaled_resolution.1),
            outline_color.into(),
        )?;
        let mouse_pos = Position { x: self.mouse_x / scaled_resolution.0,
            y: self.mouse_y / scaled_resolution.1};
        let mut mouse_grid = Grid::from_position(mouse_pos);
        mouse_grid.x = mouse_grid.x * scaled_resolution.0 as u8;
        mouse_grid.y = mouse_grid.y * scaled_resolution.1 as u8;
        let into_pos: Position = mouse_grid.into();
        graphics::draw(ctx, &grid_outline, (ggez::mint::Point2 { x: into_pos.x, y: into_pos.y },))?;

        // Draw Grid Vertical
        let grid_color = (200, 200, 0);
        for i in 1..16 {
            let line = MeshBuilder::new()
            .line(&[glam::vec2(i as f32 * (GRID_SIZE * scaled_resolution.0), 0.0),
                           glam::vec2(i as f32 * (GRID_SIZE * scaled_resolution.0), self.resolution.0)],
                  1.0, grid_color.into())?
            //.circle(DrawMode::fill(), glam::vec2(60.0, 38.0), 40.0, 1.0, (0, 255, 0).into())?
            .build(ctx)?;
            graphics::draw(ctx, &line, (ggez::mint::Point2 { x: 0.0, y: 0.0 },))?;
        }
        // Draw Grid Horizontal
        for i in 1..12 {
            let line = MeshBuilder::new()
            .line(&[glam::vec2(0.0, i as f32 * GRID_SIZE * scaled_resolution.0),
                           glam::vec2(BASE_RESOLUTION.0 * scaled_resolution.0, i as f32 * GRID_SIZE * scaled_resolution.0)],
                  1.0, grid_color.into())?
            .build(ctx)?;
            graphics::draw(ctx, &line, (ggez::mint::Point2 { x: 0.0, y: 0.0 },))?;
        }
        // Draw Player
        self.player.draw(ctx)?;
        present(ctx)?;
        ggez::timer::yield_now();
        Ok(())
    }

    fn mouse_motion_event(&mut self, _ctx: &mut Context, x: f32, y: f32, dx: f32, dy: f32) {
        self.mouse_x = x;
        self.mouse_y = y;
        self.mouse_dx = dy;
        self.mouse_dy = dy;
    }
    
    fn mouse_button_down_event(
        &mut self,
        _ctx: &mut Context,
        button: MouseButton,
        x: f32,
        y: f32,
    ) {
        let scale_x = BASE_RESOLUTION.0 / self.resolution.0;
        let scale_y = BASE_RESOLUTION.1 / self.resolution.1;
        let mouse_pos = Position {
            x: x * scale_x,
            y: y * scale_y
        };
        let grid_pos: Grid = Grid::from_position(mouse_pos);
        self.player.grid_destination = grid_pos;
        println!("Grid Clicked: {:?}", grid_pos);
    }

    fn key_down_event(
        &mut self,
        _ctx: &mut Context,
        keycode: KeyCode,
        _keymod: KeyMods,
        _repeat: bool,
    ) {
        //self.player.direction.update_from_keycode(keycode, true);
    }

    fn key_up_event(
        &mut self,
        _ctx: &mut Context,
        keycode: KeyCode,
        _keymod: KeyMods,
    ) {
        //self.player.direction.update_from_keycode(keycode, false);
        if keycode == KeyCode::G {
            //println!("G");
            self.player.move_to_posiiton(100.00, 100.00);
        }
    }
}

fn main() -> GameResult {

    let resolution: (f32, f32) = (1920.0, 1080.0);

    let window_setup = ggez::conf::WindowSetup::default().title("Riablo");

    let resource_dir = if let Ok(manifest_dir) = env::var("CARGO_MANIFEST_DIR") {
        let mut path = path::PathBuf::from(manifest_dir);
        path.push("textures");
        path
    } else {
        path::PathBuf::from("./textures")
    };

    let (mut ctx, events_loop) = ggez::ContextBuilder::new("player", "Mitt Miles")
        .window_setup(window_setup)
        .window_mode(ggez::conf::WindowMode::default().dimensions(resolution.0, resolution.1))
        .add_resource_path(resource_dir)
        .build()?;

    let state = GameState::new(&mut ctx, resolution);
    event::run(ctx, events_loop, state)
}