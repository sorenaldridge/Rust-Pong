use ggez::event::{self, EventHandler, KeyCode, KeyMods};
use ggez::nalgebra::{Point2, Vector2};
use ggez::*;
use ggez::audio::SoundSource;

use std::ffi::OsStr;


const WINDOW_WIDTH: f32 = 1000.0;
const WINDOW_HEIGHT: f32 = 700.0;

const PADDLE_HEIGHT: f32 = 180.0;
const PADDLE_WIDTH: f32 = 20.0;

const BALL_RADIUS: f32 = 12.0;
const BALL_VELOCITY: f32 = 9.0;

const DIFFICULTY: f32 = 3.0;
const PLAYER_SPEED: f32 = 10.0;

const SPLASH_WIDTH: f32 = 347.0;
const SPLASH_HEIGHT: f32 = 111.0;

const BUTTON_HEIGHT: f32 = 40.0;
const BUTTON_WIDTH: f32 = 250.0;

enum Mode {
    TitleScreen,
    Play,
    Settings,
}

struct GameState {
    dt: std::time::Duration,
    mode: Mode,
    pos_y: f32,
    opp_y: f32,
    ball_x_pos: f32,
    ball_y_pos: f32,
    ball_x_vel: f32,
    ball_y_vel: f32,
    player_score: i8,
    ai_score: i8,
    music: ggez::audio::Source,
}

impl GameState {
    fn new(ctx: &mut Context) -> Self {
        let mut soundtrack = audio::Source::new(ctx, "/soundtrack.mp3").unwrap();
    
        soundtrack.play_detached();

        Self {
            dt: std::time::Duration::new(0, 0),
            mode: Mode::TitleScreen,
            pos_y: WINDOW_HEIGHT / 2.0 - PADDLE_HEIGHT / 2.0,
            opp_y: WINDOW_HEIGHT / 2.0 - PADDLE_HEIGHT / 2.0,
            ball_x_pos: WINDOW_WIDTH / 2.0,
            ball_y_pos: WINDOW_HEIGHT / 2.0,
            ball_x_vel: BALL_VELOCITY,
            ball_y_vel: BALL_VELOCITY,
            player_score: 0,
            ai_score: 0,
            music: soundtrack,
        }
    }

    fn play_ai_score_sound(ctx: &mut Context){
        let mut lost_point = audio::Source::new(ctx, "/lost_point.wav").unwrap();
        lost_point.play_detached();
    }

    fn play_player_score_sound(ctx: &mut Context){
        let mut won_point = audio::Source::new(ctx, "/won_point.wav").unwrap();
        won_point.play_detached();
    }

    fn play_player_bump_sound(ctx: &mut Context){
        let mut player_bump = audio::Source::new(ctx, "/player_bump.wav").unwrap();
        player_bump.play_detached();
    }

    fn play_opponent_bump_sound(ctx: &mut Context){
        let mut opponent_bump = audio::Source::new(ctx, "/opponent_bump.wav").unwrap();
        opponent_bump.play_detached();
    }

    fn play_bump_sound(ctx: &mut Context){
        let mut collision = audio::Source::new(ctx, "/collision.wav").unwrap();
        collision.play_detached();
    }
}

//TODO:
//Implement DT Factor
//Start Screen
//Opening Animation
//Scoreboard
//Sound Effects
//Settings Screen
//Results Screen??
//Fix ball trap bug

//Waves Of Tranquility  by spinningmerkaba (c) copyright 2019 Licensed under a Creative Commons Attribution Noncommercial  (3.0) license. http://dig.ccmixter.org/files/jlbrock44/59736 Ft: gurdonark,speck

pub fn main() {
    use ggez::conf::*;

    let resource_dir = std::path::PathBuf::from("./resources");

    let (ref mut ctx, ref mut event_loop) = ggez::ContextBuilder::new("Pong", "Soren")
        .window_setup(WindowSetup::default().title("Pong").vsync(true))
        .window_mode(
            WindowMode::default()
                .dimensions(WINDOW_WIDTH.into(), WINDOW_HEIGHT.into())
                .borderless(true),
        )
        .add_resource_path(resource_dir)
        .build()
        .unwrap();

    let mut game_state = GameState::new(ctx);
    event::run(ctx, event_loop, &mut game_state).unwrap();
}

impl ggez::event::EventHandler for GameState {
    fn update(&mut self, ctx: &mut Context) -> GameResult {
        
        match self.mode {
            Mode::TitleScreen => {}
            Mode::Play => {
                //Ball Position
                if input::mouse::position(ctx).y - PADDLE_HEIGHT / 2.0 > self.pos_y + 10.0 {
                    self.pos_y += PLAYER_SPEED;
                } else if input::mouse::position(ctx).y - PADDLE_HEIGHT / 2.0 < self.pos_y - 10.0 {
                    self.pos_y -= PLAYER_SPEED;
                }

                //Ball "AI"
                if self.ball_y_pos >= self.opp_y + PADDLE_HEIGHT {
                    self.opp_y += DIFFICULTY;
                } else if self.ball_y_pos <= self.opp_y {
                    self.opp_y -= DIFFICULTY;
                }

                //Ball Movement
                self.ball_x_pos -= self.ball_x_vel;
                self.ball_y_pos -= self.ball_y_vel;

                //Bound Player Paddle to Screen
                if self.pos_y < 0.0 {
                    self.pos_y = 0.0;
                } else if self.pos_y + PADDLE_HEIGHT > WINDOW_HEIGHT {
                    self.pos_y = WINDOW_HEIGHT - PADDLE_HEIGHT;
                }

                //Bound Opponent Paddle to Screen
                if self.opp_y < 0.0 {
                    self.opp_y = 0.0;
                } else if self.opp_y + PADDLE_HEIGHT > WINDOW_HEIGHT {
                    self.opp_y = WINDOW_HEIGHT - PADDLE_HEIGHT;
                }

                //Ball Bounce and Score Zoning
                if self.ball_x_pos - BALL_RADIUS < 0.0 {
                    self.ball_x_vel *= -1.0;
                    self.ai_score += 1;
                    GameState::play_ai_score_sound(ctx);
                } else if self.ball_x_pos + BALL_RADIUS >= WINDOW_WIDTH {
                    self.ball_x_vel *= -1.0;
                    self.player_score += 1;
                    GameState::play_player_score_sound(ctx);
                } else if (self.ball_y_pos - BALL_RADIUS < 0.0)
                    | (self.ball_y_pos + BALL_RADIUS >= WINDOW_HEIGHT)
                {
                    self.ball_y_vel *= -1.0;
                    GameState::play_bump_sound(ctx);
                } else if (self.ball_y_pos >= self.pos_y)
                    & (self.ball_y_pos <= self.pos_y + PADDLE_HEIGHT)
                    & (self.ball_x_pos - BALL_RADIUS <= PADDLE_WIDTH)
                {
                    self.ball_x_vel *= -1.0;
                    GameState::play_player_bump_sound(ctx);
                } else if (self.ball_y_pos >= self.opp_y)
                    & (self.ball_y_pos <= self.opp_y + PADDLE_HEIGHT)
                    & (self.ball_x_pos + BALL_RADIUS >= WINDOW_WIDTH - PADDLE_WIDTH)
                {
                    self.ball_x_vel *= -1.0;
                    GameState::play_opponent_bump_sound(ctx);
                }
            }
            Mode::Settings => {}
        }
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        //match statement for checking game mode
        //Intro Animation, Main Menu, Settings, and Gameplay
        match self.mode {
            Mode::TitleScreen => {
                //SET BACKGROUND COLOR
                graphics::clear(ctx, graphics::Color::from_rgb(37, 37, 38));

                let play_button = graphics::Mesh::new_rectangle(
                    ctx,
                    graphics::DrawMode::fill(),
                    graphics::Rect::new(0.0, 0.0, BUTTON_WIDTH, BUTTON_HEIGHT),
                    graphics::Color::from_rgb(150, 150, 150),
                )?;

                graphics::draw(
                    ctx,
                    &play_button,
                    (nalgebra::Point2::new(
                        WINDOW_WIDTH / 2.0 - BUTTON_WIDTH / 2.0,
                        WINDOW_HEIGHT / 2.0 - BUTTON_HEIGHT / 2.0,
                    ),),
                );

                //LOAD IN PONG SPLASH TEXT
                let pong_splash = graphics::Image::new(ctx, "/pong.png").unwrap();

                graphics::draw(
                    ctx,
                    &pong_splash,
                    (nalgebra::Point2::new(
                        WINDOW_WIDTH / 2.0 - SPLASH_WIDTH / 2.0,
                        WINDOW_HEIGHT / 2.0 - SPLASH_HEIGHT / 2.0 - 125.0,
                    ),),
                );
            }
            Mode::Play => {
                //SET BACKGROUND COLOR
                graphics::clear(ctx, graphics::Color::from_rgb(37, 37, 38));

                //DRAW PLAYER PADDLE
                let player_paddle = graphics::Mesh::new_rectangle(
                    ctx,
                    graphics::DrawMode::fill(),
                    graphics::Rect::new(0.0, self.pos_y, PADDLE_WIDTH, PADDLE_HEIGHT),
                    graphics::Color::from_rgb(150, 150, 150),
                )?;
                graphics::draw(ctx, &player_paddle, (nalgebra::Point2::new(0.0, 0.0),))?;

                //DRAW PLAYER PADDLE
                let opponent_paddle = graphics::Mesh::new_rectangle(
                    ctx,
                    graphics::DrawMode::fill(),
                    graphics::Rect::new(
                        WINDOW_WIDTH - PADDLE_WIDTH,
                        self.opp_y,
                        PADDLE_WIDTH,
                        PADDLE_HEIGHT,
                    ),
                    graphics::Color::from_rgb(150, 150, 150),
                )?;
                graphics::draw(ctx, &opponent_paddle, (nalgebra::Point2::new(0.0, 0.0),))?;

                //DRAW BALL
                let circle = graphics::Mesh::new_circle(
                    ctx,
                    graphics::DrawMode::fill(),
                    nalgebra::Point2::new(self.ball_x_pos, self.ball_y_pos),
                    BALL_RADIUS,
                    0.1,
                    graphics::Color::from_rgb(150, 150, 150),
                )?;
                graphics::draw(ctx, &circle, (nalgebra::Point2::new(0.0, 0.0),))?;

                let mut score_board =
                    graphics::Text::new(format!("{} : {}", self.player_score, self.ai_score,));

                score_board.set_font(
                    graphics::Font::new(ctx, "/unispace_bd.ttf").unwrap(),
                    graphics::Scale::uniform(20.0),
                );

                graphics::draw(
                    ctx,
                    &score_board,
                    (nalgebra::Point2::new((WINDOW_WIDTH / 2.0) - 10.0, 10.0),),
                )?;
            }
            Mode::Settings => {}
        }

        graphics::present(ctx)?;
        std::thread::yield_now();
        Ok(())
    }

    fn key_down_event(&mut self, ctx: &mut Context, key: KeyCode, mods: KeyMods, _: bool) {
        match key {
            KeyCode::Down | KeyCode::S => {
                self.pos_y += PADDLE_HEIGHT;
            }
            KeyCode::Up | KeyCode::W => {
                self.pos_y -= PADDLE_HEIGHT;
            }
            KeyCode::Escape => {
                ggez::quit(ctx);
            }
            _ => (),
        }
    }
}
