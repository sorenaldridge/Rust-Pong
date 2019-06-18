use ggez::event::{self, EventHandler, KeyCode, KeyMods};
use ggez::nalgebra::{Point2, Vector2};
use ggez::*;

const WINDOW_WIDTH: f32 = 1000.0;
const WINDOW_HEIGHT: f32 = 700.0;

const PADDLE_HEIGHT: f32 = 180.0;
const PADDLE_WIDTH: f32 = 20.0;

const BALL_RADIUS: f32 = 12.0;
const BALL_VELOCITY: f32 = 5.0;

const DIFFICULTY: f32 = 3.0;

struct GameState {
    dt: std::time::Duration,
    pos_y: f32,
    opp_y: f32,
    ball_x_pos: f32,
    ball_y_pos: f32,
    ball_x_vel: f32,
    ball_y_vel: f32,
    player_score: i8,
    ai_score: i8,
}

impl GameState {}

pub fn main() {
    use ggez::audio::*;
    use ggez::conf::*;

    enum Mode {
        Intro,
        TitleScreen,
        Play,
        Results,
    }

    let (ref mut ctx, ref mut event_loop) = ggez::ContextBuilder::new("Pong", "Soren")
        .window_setup(WindowSetup::default().title("Pong").vsync(true))
        .window_mode(
            WindowMode::default()
                .dimensions(WINDOW_WIDTH.into(), WINDOW_HEIGHT.into())
                .borderless(true),
        )
        .build()
        .unwrap();

    let game_state = &mut GameState {
        dt: std::time::Duration::new(0, 0),
        pos_y: WINDOW_HEIGHT / 2.0 - PADDLE_HEIGHT / 2.0,
        opp_y: WINDOW_HEIGHT / 2.0 - PADDLE_HEIGHT / 2.0,
        ball_x_pos: WINDOW_WIDTH / 2.0,
        ball_y_pos: WINDOW_HEIGHT / 2.0,
        ball_x_vel: BALL_VELOCITY,
        ball_y_vel: BALL_VELOCITY,
        player_score: 0,
        ai_score: 0,
    };

    event::run(ctx, event_loop, game_state).unwrap();
}

impl ggez::event::EventHandler for GameState {
    fn update(&mut self, ctx: &mut Context) -> GameResult {
        //Ball Position
        self.pos_y = input::mouse::position(ctx).y - PADDLE_HEIGHT / 2.0;

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
            println!("AI POINT!");
            // self.ball_x_pos = WINDOW_WIDTH / 2.0;
            // self.ball_y_pos = WINDOW_HEIGHT / 2.0;
            self.ball_x_vel *= -1.0;
            self.ai_score += 1;
        } else if self.ball_x_pos + BALL_RADIUS >= WINDOW_WIDTH {
            println!("PLAYER POINT!");
            // self.ball_x_pos = WINDOW_WIDTH / 2.0;
            // self.ball_y_pos = WINDOW_HEIGHT / 2.0;
            self.ball_x_vel *= -1.0;
            self.player_score += 1;
        } else if (self.ball_y_pos - BALL_RADIUS < 0.0)
            | (self.ball_y_pos + BALL_RADIUS >= WINDOW_HEIGHT)
        {
            self.ball_y_vel *= -1.0;
        } else if (self.ball_y_pos >= self.pos_y)
            & (self.ball_y_pos <= self.pos_y + PADDLE_HEIGHT)
            & (self.ball_x_pos - BALL_RADIUS <= PADDLE_WIDTH)
        {
            self.ball_x_vel *= -1.0;
        } else if (self.ball_y_pos >= self.opp_y)
            & (self.ball_y_pos <= self.opp_y + PADDLE_HEIGHT)
            & (self.ball_x_pos + BALL_RADIUS >= WINDOW_WIDTH - PADDLE_WIDTH)
        {
            self.ball_x_vel *= -1.0;
        }
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult {
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

        //DRAW SCORE
        // let score_str =
        //     graphics::TextFragment::new(format!("{} : {}", self.player_score, self.ai_score,));
        // let score_board = graphics::Text::new(score_str);
        // graphics::TextFragment::font(
        //     score_board,
        //     graphics::Font::new(
        //         ctx,
        //         (core::convert::AsRef::as_ref("resources/unispace_rg.ttf")),
        //     ),
        // );
        // graphics::draw(
        //     ctx,
        //     &score_board,
        //     (nalgebra::Point2::new((WINDOW_WIDTH / 2.0) - 10.0, 10.0),),
        // )?;

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
