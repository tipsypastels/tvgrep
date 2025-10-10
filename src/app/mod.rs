mod error;
mod event;

use self::{
    error::Errors,
    event::{Event, Events},
};
use anyhow::{Context, Error, Result};
use crossterm::event::Event as TermEvent;
use ratatui::{DefaultTerminal, buffer::Buffer, layout::Rect};

type MessengerImpl<A> = (
    <A as App>::Message,
    <A as App>::MessageOutput,
    <A as App>::MessageContext,
);

pub trait App {
    type Message: Send + 'static;
    type MessageOutput: Send + 'static;
    type MessageContext: Clone + Send + 'static;

    fn tick(&mut self, messenger: Messenger<Self>) -> Result<()>;
    fn render(&mut self, error: Option<&Error>, area: Rect, buf: &mut Buffer);
    fn handle(
        &mut self,
        event: TermEvent,
        messenger: Messenger<Self>,
        quit: &mut bool,
    ) -> Result<()>;

    fn on_message(
        message: Self::Message,
        context: Self::MessageContext,
    ) -> impl Future<Output = Self::MessageOutput> + Send + 'static;
    fn apply_message(&mut self, output: Self::MessageOutput) -> Result<()>;
    fn new_message_context(&self) -> Self::MessageContext;

    fn on_start(&mut self, messenger: Messenger<Self>) {
        let _ = messenger;
    }

    fn on_quit(&mut self) {}

    fn is_terminal(error: &Error) -> bool {
        let _ = error;
        false
    }

    async fn run(&mut self, term: &mut DefaultTerminal) -> Result<()> {
        let message_context = self.new_message_context();
        let mut events = Events::<MessengerImpl<Self>>::new(message_context, Self::on_message);
        let mut errors = Errors::new();
        let mut quit = false;

        self.on_start(Messenger(&events));

        while !quit {
            term.draw(|frame| self.render(errors.peek(), frame.area(), frame.buffer_mut()))
                .context("render error")?;

            let mut catch = |result: Result<()>| match result {
                Err(e) if !Self::is_terminal(&e) => {
                    errors.push(e);
                    Ok(())
                }
                res => res,
            };

            match events.next().await.context("event loop error")? {
                Event::Tick => {
                    catch(self.tick(Messenger(&events)).context("tick error"))?;
                    errors.tick();
                }
                Event::Term(event) => {
                    catch(
                        self.handle(event, Messenger(&events), &mut quit)
                            .context("event error"),
                    )?;
                }
                Event::MessageOutput(output) => {
                    catch(self.apply_message(output).context("message error"))?;
                }
            }
        }

        // TODO: Ensure events are shut down?
        self.on_quit();
        Ok(())
    }
}

pub struct Messenger<'a, A: App + ?Sized>(&'a Events<MessengerImpl<A>>);

impl<A: App> Messenger<'_, A> {
    pub fn send(&self, message: A::Message) {
        self.0.send_message(message);
    }
}
