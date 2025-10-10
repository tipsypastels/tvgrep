use super::event::FPS;
use anyhow::Error;
use std::collections::VecDeque;

const TIMER_MAX: u8 = 5 * FPS as u8;

pub struct Errors {
    queue: VecDeque<Error>,
    timer: Option<u8>,
}

impl Errors {
    pub fn new() -> Self {
        Self {
            queue: VecDeque::new(),
            timer: None,
        }
    }

    pub fn peek(&self) -> Option<&Error> {
        self.queue.front()
    }

    pub fn push(&mut self, error: Error) {
        self.queue.push_back(error);
        self.timer.get_or_insert(TIMER_MAX);
    }

    pub fn tick(&mut self) {
        if let Some(timer) = self.timer {
            if let Some(timer) = timer.checked_sub(1) {
                self.timer = Some(timer);
            } else {
                self.queue.pop_front();
                self.timer = None;
            }
        }
    }
}
