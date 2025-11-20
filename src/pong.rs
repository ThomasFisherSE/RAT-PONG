use ratatui::{
    Frame,
    style::{Color},
    symbols::Marker,
    widgets::canvas::{Canvas, Circle, Points},
};

const PADDLE_DISTANCE_FROM_EDGE: u16 = 2;

enum BallDirection {
    Left,
    Right
}

#[derive(Debug, Clone, Copy)]
struct Paddle {
    /// The vertical center position (in cells) of the paddle in the playfield.
    vertical_pos: f32,
    /// Half of the paddle height (in cells).
    half_height: u16,
    /// The horizontal center position (in cells) of the paddle in the playfield (fixed).
    horizontal_pos: u16,
    /// The color of the paddle.
    color: Color,
}

impl Paddle {
    pub fn default() -> Self {
        Paddle {
            vertical_pos: 10.0,
            half_height: 2,
            horizontal_pos: PADDLE_DISTANCE_FROM_EDGE,
            color: Color::White
        }
    }

    pub fn new(color: Color) -> Self {
        let mut paddle = Paddle::default();
        paddle.set_color(color);
        paddle
    }

    pub fn set_color(&mut self, color: Color) { self.color = color; }
}

#[derive(Debug, Clone, Copy)]
struct Ball {
    horizontal_pos: f32,
    vertical_pos: f32,
    horizontal_vel: f32,
    vertial_vel: f32,
}

impl Ball {
    pub fn default() -> Self {
        Ball {
            horizontal_pos: 10.0,
            vertical_pos: 10.0,
            horizontal_vel: 18.0,
            vertial_vel: 0.0
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Scores {
    left_score: u32,
    right_score: u32,
}

impl Scores {
    pub fn default() -> Self {
        Scores {
            left_score: 0,
            right_score: 0
        }
    }

    pub fn left_score(&self) -> u32 { self.left_score }

    pub fn right_score(&self) -> u32 { self.right_score }
}

#[derive(Debug)]
pub struct PlayArea {
    pub width: u16,
    pub height: u16,
}

impl PlayArea {
    pub fn default() -> Self {
        PlayArea { width: 40, height: 20 }
    }
}

#[derive(Debug, Clone, Copy)]
struct Spark {
    x: f32,
    y: f32,
    ttl: f32, // seconds remaining
}

impl Spark {
    pub fn new(x: f32, y: f32) -> Self {
        Spark {
            x,
            y,
            ttl: 0.22
        }
    }
}

#[derive(Debug)]
pub struct PongGame {
    left_paddle: Paddle,
    right_paddle: Paddle,
    ball: Ball,
    scores: Scores,
    play_area: PlayArea,

    marker: Marker,
    trail_enabled: bool,
    ball_trail: std::collections::VecDeque<(f32, f32)>,
    max_trail: usize,
    sparks: Vec<Spark>,
}

impl PongGame {
    pub fn new() -> Self {
        Self {
            left_paddle: Paddle::new(Color::Blue),
            right_paddle: Paddle::new(Color::Green),
            ball: Ball::default(),
            scores: Scores::default(),
            play_area: PlayArea::default(),
            marker: Marker::Block,
            trail_enabled: true,
            ball_trail: std::collections::VecDeque::with_capacity(16),
            max_trail: 16,
            sparks: Vec::new(),
        }
    }

    pub fn set_play_area(&mut self, width: u16, height: u16) {
        self.play_area.width = width;
        self.play_area.height = height;

        // align right paddle to the right edge minus gap
        let right_edge = width - 1;
        self.right_paddle.horizontal_pos = right_edge - PADDLE_DISTANCE_FROM_EDGE;
    }

    pub fn toggle_trail(&mut self) {
        self.trail_enabled = !self.trail_enabled;
        if !self.trail_enabled { self.ball_trail.clear(); }
    }

    pub fn toggle_marker(&mut self) {
        self.marker = match self.marker {
            Marker::Block => Marker::Dot,
            _ => Marker::Block
        };
    }

    pub fn trail_enabled(&self) -> bool { self.trail_enabled }
    pub fn marker(&self) -> Marker { self.marker }
    pub fn scores(&self) -> Scores { self.scores }
    pub fn ball_speed(&self) -> i32 { self.ball.horizontal_vel.hypot(self.ball.vertial_vel) as i32 }

    pub fn nudge_left_paddle(&mut self, delta: f32) {
        self.left_paddle.vertical_pos += delta;
        Self::clamp_paddle(&mut self.left_paddle, self.play_area.height);
    }

    pub fn nudge_right_paddle(&mut self, delta: f32) {
        self.right_paddle.vertical_pos += delta;
        Self::clamp_paddle(&mut self.right_paddle, self.play_area.height);
    }

    pub fn update(&mut self, dt: f32) {
        Self::clamp_paddle(&mut self.left_paddle, self.play_area.height);
        Self::clamp_paddle(&mut self.right_paddle, self.play_area.height);

        self.update_ball_pos(dt);
        self.update_trail();
        self.update_existing_sparks(dt);
        self.update_wall_collisions();
        self.update_paddle_collisions();
        self.update_scores();
    }

    fn update_ball_pos(&mut self, dt: f32) {
        self.ball.horizontal_pos += self.ball.horizontal_vel * dt;
        self.ball.vertical_pos += self.ball.vertial_vel * dt;
    }


    fn update_trail(&mut self) {
        if self.trail_enabled {
            if self.ball_trail.len() >= self.max_trail {
                self.ball_trail.pop_front();
            }
            self.ball_trail.push_back((self.ball.horizontal_pos, self.ball.vertical_pos));
        } else {
            self.ball_trail.clear();
        }
    }

    fn update_existing_sparks(&mut self, dt: f32) {
        if !self.sparks.is_empty() {
            for sp in &mut self.sparks { sp.ttl -= dt; }
            self.sparks.retain(|sp| sp.ttl > 0.0);
        }
    }

    fn update_wall_collisions(&mut self) {
        if self.ball.vertical_pos < 0.0 {
            self.ball.vertical_pos = 0.0;
            self.ball.vertial_vel = self.ball.vertial_vel.abs();
        }
        if self.ball.vertical_pos > (self.play_area.height - 1) as f32 {
            self.ball.vertical_pos = (self.play_area.height - 1) as f32;
            self.ball.vertial_vel = -self.ball.vertial_vel.abs();
        }
    }

    fn update_paddle_collisions(&mut self) {
        let mut collided = false;

        let ball_horizontal_pos = self.ball.horizontal_pos.round() as u16;
        let mut handle_collision_with_paddle = |paddle: &Paddle| {
            let top = paddle.vertical_pos - paddle.half_height as f32;
            let bot = paddle.vertical_pos + paddle.half_height as f32;
            if self.ball.vertical_pos >= top - 0.5 && self.ball.vertical_pos <= bot + 0.5 {
                self.ball.horizontal_pos = paddle.horizontal_pos as f32 - self.ball.horizontal_vel.signum();
                self.ball.horizontal_vel = self.ball.horizontal_vel * -1.0;
                let offset = (self.ball.vertical_pos - paddle.vertical_pos) / (paddle.half_height as f32 + 0.5);
                const INITIAL_Y_VEL_LIMIT: f32 = 0.5;
                const OFFSET_MULTIPLIER: f32 = 10.0;
                const BASE_Y_VEL_MULTIPLIER: f32 = 5.0;
                self.ball.vertial_vel = (self.ball.vertial_vel.signum().max(INITIAL_Y_VEL_LIMIT))
                    * BASE_Y_VEL_MULTIPLIER + offset * OFFSET_MULTIPLIER;
                collided = true;
            }
        };

        if ball_horizontal_pos <= self.left_paddle.horizontal_pos {
            handle_collision_with_paddle(&self.left_paddle);
        }
        else if ball_horizontal_pos >= self.right_paddle.horizontal_pos {
            handle_collision_with_paddle(&self.right_paddle);
        }

        if collided {
            self.create_spark();
        }
    }

    fn update_scores(&mut self)
    {
        const SCORE_DISTANCE: f32 = 3.0;
        if self.ball.horizontal_pos < (self.left_paddle.horizontal_pos as f32 - SCORE_DISTANCE) {
            self.scores.right_score += 1;
            self.reset_ball(BallDirection::Left);
            self.ball_trail.clear();
        } else if self.ball.horizontal_pos > (self.right_paddle.horizontal_pos as f32 + SCORE_DISTANCE) {
            self.scores.left_score += 1;
            self.reset_ball(BallDirection::Right);
            self.ball_trail.clear();
        }

    }

    fn create_spark(&mut self) {
        self.sparks.push(Spark::new(self.ball.horizontal_pos, self.ball.vertical_pos));

        const SPARK_LIMIT: usize = 8;
        if self.sparks.len() > SPARK_LIMIT {
            self.sparks.drain(0..(self.sparks.len() - SPARK_LIMIT));
        }
    }

    fn reset_ball(&mut self, ball_direction: BallDirection) {
        self.ball.horizontal_pos = (self.play_area.width / 2) as f32;
        self.ball.vertical_pos = (self.play_area.height / 2) as f32;
        let ball_speed = 18.0;
        self.ball.horizontal_vel = match ball_direction {
            BallDirection::Left => -ball_speed,
            BallDirection::Right => ball_speed,
        };
        self.ball.vertial_vel = 6.0;
    }

    fn clamp_paddle(p: &mut Paddle, field_h: u16) {
        let min_y = p.half_height as f32;
        let max_y = (field_h - 1 - p.half_height) as f32;
        p.vertical_pos = p.vertical_pos.clamp(min_y, max_y);
    }

    pub fn render(&mut self, frame: &mut Frame, area: ratatui::layout::Rect) {
        let width = area.width;
        let height = area.height;
        self.set_play_area(width, height);

        const MIN_WIDTH: u16 = 10;
        const MIN_HEIGHT: u16 = 7;
        if width < MIN_WIDTH || height < MIN_HEIGHT {
            // Too small to render
            return;
        }

        let canvas = Canvas::default()
            .marker(self.marker)
            .x_bounds([0.0, (width - 1) as f64])
            .y_bounds([0.0, (height - 1) as f64])
            .paint(|ctx| {
                // Canvas Y increases up, but game Y increases down. Flip Y for drawing.
                let flip_y = |y: f64| -> f64 { (height - 1) as f64 - y };

                // Draw background grid
                const BACKGROUND_GRID_STEP: usize = 2;
                let mut grid: Vec<(f64, f64)> = Vec::new();
                for grid_y in (0..height).step_by(BACKGROUND_GRID_STEP) {
                    for grid_x in (0..width).step_by(BACKGROUND_GRID_STEP) {
                        grid.push((grid_x as f64, flip_y(grid_y as f64)));
                    }
                }
                const BACKGROUND_GRID_COLOR: Color = Color::Rgb(40, 40, 40);
                ctx.draw(&Points { coords: &grid, color: BACKGROUND_GRID_COLOR });

                // Draw net
                const NET_STEP: u16 = 2;
                let net_horizontal_pos = (width as f64) / 2.0;
                let mut net_points: Vec<(f64, f64)> = Vec::with_capacity(height as usize);
                for row in 0..height {
                    if row % NET_STEP == 0 {
                        net_points.push((net_horizontal_pos, flip_y(row as f64)));
                    }
                }
                const NET_COLOR: Color = Color::DarkGray;
                ctx.draw(&Points { coords: &net_points, color: NET_COLOR });

                // Draw ball trail
                if self.trail_enabled && !self.ball_trail.is_empty() {
                    let trail_length = self.ball_trail.len();
                    let older_part = (trail_length as f32 / 3.0).ceil() as usize;
                    let middle_part = (2.0 * trail_length as f32 / 3.0).ceil() as usize;
                    let mut older_points: Vec<(f64, f64)> = Vec::new();
                    let mut middle_points: Vec<(f64, f64)> = Vec::new();
                    let mut recent_points: Vec<(f64, f64)> = Vec::new();
                    for (i, &(x, y)) in self.ball_trail.iter().enumerate() {
                        let trail_point = (x as f64, flip_y(y as f64));
                        if i < older_part {
                            older_points.push(trail_point);
                        }
                        else if i < middle_part {
                            middle_points.push(trail_point);
                        }
                        else {
                            recent_points.push(trail_point);
                        }
                    }
                    if !older_points.is_empty() {
                        ctx.draw(&Points { coords: &older_points, color: Color::DarkGray });
                    }
                    if !middle_points.is_empty() {
                        ctx.draw(&Points { coords: &middle_points, color: Color::Gray });
                    }
                    if !recent_points.is_empty() {
                        ctx.draw(&Points { coords: &recent_points, color: Color::LightRed });
                    }
                }

                // Draw paddles
                let mut draw_paddle = |paddle: &Paddle| {
                    let top = (paddle.vertical_pos - paddle.half_height as f32) as f64;
                    let bot = (paddle.vertical_pos + paddle.half_height as f32) as f64;
                    let horizontal_pos = paddle.horizontal_pos as f64;
                    ctx.draw(&ratatui::widgets::canvas::Line {
                        x1: horizontal_pos,
                        y1: flip_y(top),
                        x2: horizontal_pos,
                        y2: flip_y(bot),
                        color: paddle.color
                    });
                };
                draw_paddle(&self.left_paddle);
                draw_paddle(&self.right_paddle);

                // Draw sparks near collisions
                if !self.sparks.is_empty() {
                    for sp in &self.sparks {
                        let mut pts: Vec<(f64, f64)> = Vec::with_capacity(8);
                        let x = sp.x as f64; let y = flip_y(sp.y as f64);
                        pts.push((x, y));
                        pts.push((x+1.0, y));
                        pts.push((x-1.0, y));
                        pts.push((x, y+1.0));
                        pts.push((x, y-1.0));
                        pts.push((x+0.7, y+0.7));
                        pts.push((x-0.7, y-0.7));
                        pts.push((x+0.7, y-0.7));
                        ctx.draw(&Points { coords: &pts, color: Color::Yellow });
                    }
                }

                // Draw ball
                const BALL_RADIUS: f64 = 0.2;
                const BALL_COLOR: Color = Color::Red;
                ctx.draw(&Circle {
                    x: self.ball.horizontal_pos as f64,
                    y: flip_y(self.ball.vertical_pos as f64),
                    radius: BALL_RADIUS,
                    color: BALL_COLOR
                });
            });
        frame.render_widget(canvas, area);
    }
}