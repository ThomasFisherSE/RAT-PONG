use crossterm::event::{KeyCode, KeyEvent, KeyEventKind, KeyModifiers};

#[derive(Debug)]
#[derive(Eq, Hash, PartialEq)]
pub enum InputAction {
    LeftPlayerMoveUp,
    LeftPlayerMoveDown,

    RightPlayerMoveUp,
    RightPlayerMoveDown,

    ToggleMarker,
    ToggleTrail,
    Quit,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum InputActionState {
    Pressed,
    Released,
}

#[derive(Debug)]
pub struct InputEvent {
    pub action: InputAction,
    pub state: InputActionState,
}

#[derive(Debug)]
pub struct InputSystem {
    pub events: Vec<InputEvent>,
}

impl InputSystem {
    pub fn new() -> Self {
        Self {
            events: vec![],
        }
    }

    pub fn drain_events(&mut self) -> Vec<InputEvent> { std::mem::take(&mut self.events) }

    pub fn on_key_event(&mut self, key: KeyEvent) {
        match (key.modifiers, key.code) {
            (_, KeyCode::Esc | KeyCode::Char('q'))
            | (KeyModifiers::CONTROL, KeyCode::Char('c') | KeyCode::Char('C')) => {
                let event = InputEvent {
                    action: InputAction::Quit,
                    state: InputActionState::Pressed
                };
                self.events.push(event);
            },
            _ => {
                match key.code {
                    KeyCode::Char('w') | KeyCode::Char('W') => {
                        match key.kind {
                            KeyEventKind::Press | KeyEventKind::Repeat => {
                                let event = InputEvent {
                                    action: InputAction::LeftPlayerMoveUp,
                                    state: InputActionState::Pressed
                                };
                                self.events.push(event);
                            }
                            KeyEventKind::Release => {
                                let event = InputEvent {
                                    action: InputAction::LeftPlayerMoveUp,
                                    state: InputActionState::Released
                                };
                                self.events.push(event);
                            }
                        }
                    }
                    KeyCode::Char('s') | KeyCode::Char('S') => {
                        match key.kind {
                            KeyEventKind::Press | KeyEventKind::Repeat => {
                                let event = InputEvent {
                                    action: InputAction::LeftPlayerMoveDown,
                                    state: InputActionState::Pressed
                                };
                                self.events.push(event);
                            }
                            KeyEventKind::Release => {
                                let event = InputEvent {
                                    action: InputAction::LeftPlayerMoveDown,
                                    state: InputActionState::Released
                                };
                                self.events.push(event);
                            }
                        }
                    }
                    KeyCode::Up => {
                        match key.kind {
                            KeyEventKind::Press | KeyEventKind::Repeat => {
                                let event = InputEvent {
                                    action: InputAction::RightPlayerMoveUp,
                                    state: InputActionState::Pressed
                                };
                                self.events.push(event);
                            }
                            KeyEventKind::Release => {
                                let event = InputEvent {
                                    action: InputAction::RightPlayerMoveUp,
                                    state: InputActionState::Released
                                };
                                self.events.push(event);
                            }
                        }
                    }
                    KeyCode::Down => {
                        match key.kind {
                            KeyEventKind::Press | KeyEventKind::Repeat => {
                                let event = InputEvent {
                                    action: InputAction::RightPlayerMoveDown,
                                    state: InputActionState::Pressed
                                };
                                self.events.push(event);
                            }
                            KeyEventKind::Release => {
                                let event = InputEvent {
                                    action: InputAction::RightPlayerMoveDown,
                                    state: InputActionState::Released
                                };
                                self.events.push(event);
                            }
                        }
                    }
                    KeyCode::Char('t') | KeyCode::Char('T') => {
                        if matches!(key.kind, KeyEventKind::Press | KeyEventKind::Repeat) {
                            let event = InputEvent {
                                action: InputAction::ToggleTrail,
                                state: InputActionState::Pressed
                            };
                            self.events.push(event);
                        }
                    }
                    KeyCode::Char('m') | KeyCode::Char('M') => {
                        if matches!(key.kind, KeyEventKind::Press | KeyEventKind::Repeat) {
                            let event = InputEvent {
                                action: InputAction::ToggleMarker,
                                state: InputActionState::Pressed
                            };
                            self.events.push(event);
                        }
                    }
                    _ => {}
                }
            }
        }
    }
}
