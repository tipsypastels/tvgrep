mod error;
mod event;
mod message;

use self::{
    error::Errors,
    event::{Event, Events},
    message::{Messages, Messenger},
};
use anyhow::{Context, Error, Result};
use crossterm::event::Event as TermEvent;
use ratatui::{DefaultTerminal, buffer::Buffer, layout::Rect};

pub trait App {
    type Messenger: Messenger;

    fn tick(&mut self, tx: Tx<Self>) -> Result<()>;
    fn render(&mut self, info: RenderInfo, area: Rect, buf: &mut Buffer);
    fn handle(&mut self, event: TermEvent, tx: Tx<Self>, quit: &mut bool) -> Result<()>;

    fn on_message(
        message: <Self::Messenger as Messenger>::Input,
        context: <Self::Messenger as Messenger>::Context,
    ) -> impl Future<Output = <Self::Messenger as Messenger>::Output> + Send + 'static;
    fn apply_message(&mut self, output: <Self::Messenger as Messenger>::Output) -> Result<()>;
    fn new_message_context(&self) -> <Self::Messenger as Messenger>::Context;

    fn on_start(&mut self, tx: Tx<Self>) {
        let _ = tx;
    }

    fn on_quit(&mut self) {}

    fn is_terminal(error: &Error) -> bool {
        let _ = error;
        false
    }

    async fn run(&mut self, term: &mut DefaultTerminal) -> Result<()> {
        let mut messages = Messages::new(self.new_message_context(), Self::on_message);
        let mut events = Events::new();
        let mut errors = Errors::new();
        let mut quit = false;
        let mut frame_no = 0;

        self.on_start(Tx(&mut messages));

        macro_rules! rinfo {
            ($quitting:expr) => {
                RenderInfo {
                    frame_no,
                    error: errors.peek(),
                    loading: messages.is_loading(),
                    quitting: $quitting,
                }
            };
        }

        while !quit {
            term.draw(|frame| self.render(rinfo!(false), frame.area(), frame.buffer_mut()))
                .context("render error")?;

            let mut catch = |result: Result<()>| match result {
                Err(e) if !Self::is_terminal(&e) => {
                    errors.push(e);
                    Ok(())
                }
                res => res,
            };

            tokio::select! {
                message = messages.next() => {
                    let message = message.context("message loop error")?;
                    catch(self.apply_message(message).context("message error"))?;
                }
                event = events.next() => {
                    let event = event.context("event loop error")?;
                    match event {
                        Event::Tick => {
                            catch(self.tick(Tx(&mut messages)).context("tick error"))?;
                            errors.tick();
                        },
                        Event::Term(event) => {
                            catch(self.handle(event, Tx(&mut messages), &mut quit).context("event error"))?;
                        }
                    }

                }
            }

            frame_no = frame_no.wrapping_add(1);
        }

        term.draw(|frame| self.render(rinfo!(true), frame.area(), frame.buffer_mut()))
            .context("render final frame error")?;

        messages.close().await.context("message close error")?;
        self.on_quit();

        Ok(())
    }
}

pub struct RenderInfo<'a> {
    pub frame_no: usize,
    pub error: Option<&'a Error>,
    pub loading: bool,
    pub quitting: bool,
}

pub struct Tx<'a, A: App + ?Sized>(&'a mut Messages<A::Messenger>);

impl<A: App> Tx<'_, A> {
    pub fn send(&mut self, message: <A::Messenger as Messenger>::Input) {
        self.0.send(message);
    }
}
