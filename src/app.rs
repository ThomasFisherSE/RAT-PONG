use crate::input::{InputAction, InputActionState, InputEvent, InputSystem};
use crate::pong::PongGame;
use color_eyre::Result;
use crossterm::event::{self, Event};
use ratatui::{
    style::Stylize, text::Line,
    widgets::{Block, BorderType, Paragraph},
    DefaultTerminal,
    Frame,
};
use std::time::{Duration, Instant};

const TARGET_FPS: u64 = 60;

#[derive(Debug)]
pub struct App {
    running: bool,

    pong: PongGame,

    input_system: InputSystem,

    last_tick: Instant,
    tick_rate: Duration,
}

impl Default for App {
    fn default() -> Self {
        Self::new()
    }
}

impl App {
    pub fn new() -> Self {
        Self {
            running: false,
            pong: PongGame::new(),
            input_system: InputSystem::new(),
            last_tick: Instant::now(),
            tick_rate: Duration::from_millis(1000/TARGET_FPS)
        }
    }

    pub fn run(mut self, mut terminal: DefaultTerminal) -> Result<()> {
        self.running = true;

        while self.running {
            terminal.draw(|frame| self.render(frame))?;

            // handle input with timeout until next tick
            let elapsed = Instant::now().saturating_duration_since(self.last_tick);
            let timeout = if elapsed >= self.tick_rate {
                Duration::from_millis(0)
            } else {
                self.tick_rate - elapsed
            };

            if event::poll(timeout)? {
                self.handle_crossterm_events()?;
            }

            // update if it's time
            let tick_time = Instant::now();
            if tick_time.saturating_duration_since(self.last_tick) >= self.tick_rate {
                let delta_time = tick_time.duration_since(self.last_tick).as_secs_f32();
                self.update(delta_time);
                self.last_tick = tick_time;
            }
        }
        Ok(())
    }

    fn render(&mut self, frame: &mut Frame) {
        let scores = self.pong.scores();
        let title_text = format!(
            "RAT-PONG  |  {} : {}  |  v={}  |  [W/S][↑/↓] move  [T]rail={}  [M]arker={}  [q/Esc] quit",
            scores.left_score(),
            scores.right_score(),
            self.pong.ball_speed(),
            if self.pong.trail_enabled() { "on" } else { "off" },
            self.pong.marker()
        );
        let title = Line::from(title_text).bold().blue().centered();

        let area = frame.area();
        let block = Block::bordered()
            .border_type(BorderType::Rounded)
            .title(title);
        let inner = block.inner(area);

        // draw outer chrome
        frame.render_widget(block, area);

        // delegate game rendering into inner area or show help if too small
        if inner.width as i32 >= 10 && inner.height as i32 >= 7 {
            self.pong.render(frame, inner);
        } else {
            let help = Paragraph::new("Make the window bigger to play.").centered();
            frame.render_widget(help, inner);
        }
    }

    fn handle_crossterm_events(&mut self) -> Result<()> {
        match event::read()? {
            Event::Key(key) =>  self.input_system.on_key_event(key),
            Event::Mouse(_) => {}
            Event::Resize(_, _) => {}
            _ => {}
        }

        Ok(())
    }

    fn process_inputs(&mut self, dt: f32, input_events: Vec<InputEvent>) {
        let step: f32 = 1.0; // move 1 cell per key press
        for event in input_events {
            match event.action {
                InputAction::ToggleMarker => self.pong.toggle_marker(),
                InputAction::ToggleTrail => self.pong.toggle_trail(),
                InputAction::Quit => self.quit(),
                InputAction::LeftPlayerMoveUp => {
                    if event.state == InputActionState::Pressed { self.pong.nudge_left_paddle(-step); }
                }
                InputAction::LeftPlayerMoveDown => {
                    if event.state == InputActionState::Pressed { self.pong.nudge_left_paddle(step); }
                }
                InputAction::RightPlayerMoveUp => {
                    if event.state == InputActionState::Pressed { self.pong.nudge_right_paddle(-step); }
                }
                InputAction::RightPlayerMoveDown => {
                    if event.state == InputActionState::Pressed { self.pong.nudge_right_paddle(step); }
                }
            }
        }
        
        // Game update no longer uses continuous input state for movement
        self.pong.update(dt);
    }

    fn update(&mut self, dt: f32) {
        let events = self.input_system.drain_events();
        self.process_inputs(dt, events);
    }

    pub fn quit(&mut self) {
        self.running = false;
    }
}
