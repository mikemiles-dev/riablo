
//use oorandom::Rand32;

use ggez::input::mouse::position;
use glam::*;
use ggez::{event, Context, GameResult, graphics};
//use ggez::input::mouse::MouseButton;
use ggez::event::{KeyCode, KeyMods, MouseButton};
use ggez::graphics::{GlBackendSpec, Image, draw, Rect,
                     ImageGeneric, clear, present};

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

#[derive(Debug, Default, PartialEq)]
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
        //println!("{} {}", self.draw_position.x, self.destination.x);
        //if self.draw_position.x != self.destination.x {
        //    self.is_moving = true;
        //    self.draw_position.x += 1.00;
        //} else {
        //    self.is_moving = false;
        //}
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
        .scale(Vec2::new(get_scaled_resolution(self.resolution).0,
                                get_scaled_resolution(self.resolution).1));
        draw(ctx, &self.sprite.texture, param)?;
        Ok(())
    }
}

struct GameState {
    player: Player,
    resolution: (f32, f32),
}

impl GameState {
    pub fn new(ctx: &mut Context, resolution: (f32, f32)) -> Self {

        GameState {
            player: Player::new(ctx, resolution),
            resolution
        }
    }
}

impl event::EventHandler<ggez::GameError> for GameState {
    fn update(&mut self, _ctx: &mut Context) -> GameResult {
        self.player.update();
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        // Clear background
        clear(ctx, [0.0, 0.0, 0.0, 1.0].into());

        let green_rectangle = graphics::Mesh::new_rectangle(
            ctx,
            graphics::DrawMode::fill(),
            Rect::new(0.0,
                0.0, 
                BASE_RESOLUTION.0 * get_scaled_resolution(self.resolution).0, 
                BASE_RESOLUTION.1 * get_scaled_resolution(self.resolution).1),
            [0.0, 1.0, 0.0, 1.0].into(),
        )?;
        graphics::draw(ctx, &green_rectangle, (ggez::mint::Point2 { x: 0.0, y: 0.0 },))?;

        // Draw Player
        self.player.draw(ctx)?;
        present(ctx)?;
        ggez::timer::yield_now();
        Ok(())
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