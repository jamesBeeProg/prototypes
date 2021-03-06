use cgmath::Vector2;
use glfw::{Action, Key, MouseButton, WindowEvent};
use std::collections::HashSet;

#[derive(Debug, Clone, Default)]
pub struct Input {
    key_held: HashSet<Key>,
    key_pressed: HashSet<Key>,

    mouse_position: Option<Vector2<f64>>,
    mouse_held: HashSet<MouseButton>,
    mouse_pressed: HashSet<MouseButton>,
}

impl Input {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn handle(&mut self, event: &WindowEvent, window_size: Vector2<u32>) {
        match event {
            WindowEvent::Key(key, _, Action::Press, _) => {
                self.key_held.insert(*key);
                self.key_pressed.insert(*key);
            }

            WindowEvent::Key(key, _, Action::Release, _) => {
                self.key_held.remove(key);
            }

            WindowEvent::CursorPos(x, y) => {
                self.mouse_position = Some(Vector2::new(*x, *y));
            }

            WindowEvent::MouseButton(button, Action::Press, _) => {
                self.mouse_held.insert(*button);
                self.mouse_pressed.insert(*button);
            }

            WindowEvent::MouseButton(button, Action::Release, _) => {
                self.mouse_held.remove(button);
            }

            _ => {}
        }

        let mouse_outside = self
            .mouse_position
            .map(|it| {
                it.zip(window_size, |it, window_size| {
                    (0.0..window_size as f64).contains(&it)
                })
            })
            .map(|it| it.x || it.y)
            .unwrap_or(false);

        if mouse_outside {
            self.mouse_position = None;
        }
    }

    pub fn is_key_held(&self, key: Key) -> bool {
        self.key_held.contains(&key)
    }

    pub fn was_key_pressed(&mut self, key: Key) -> bool {
        self.key_pressed.take(&key).is_some()
    }

    pub fn mouse_position(&self) -> &Option<Vector2<f64>> {
        &self.mouse_position
    }

    pub fn is_mouse_held(&self, button: MouseButton) -> bool {
        self.mouse_held.contains(&button)
    }

    pub fn was_mouse_pressed(&mut self, button: MouseButton) -> bool {
        self.mouse_pressed.take(&button).is_some()
    }
}
