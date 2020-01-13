use chrono::Local;
use iced::{
    Align, Application, Column, Command, Container, Element, Length, Settings, Subscription, Text,
};
use std::time::{Duration, Instant};

pub fn main() {
    Clock::run(Settings::default())
}

struct Clock {}

#[derive(Debug, Clone)]
enum Message {
    Tick(Instant),
}

impl Application for Clock {
    type Message = Message;

    fn new() -> (Self, Command<Message>) {
        (Self {}, Command::none())
    }

    fn title(&self) -> String {
        String::from("Clock")
    }

    fn update(&mut self, _message: Message) -> Command<Message> {
        Command::none()
    }

    fn subscription(&self) -> Subscription<Message> {
        every(Duration::from_secs(1)).map(Message::Tick)
    }

    fn view(&mut self) -> Element<Message> {
        let now = Text::new(Local::now().format("%F (%a) %T").to_string()).size(40);
        let content = Column::new().align_items(Align::Center).push(now);
        Container::new(content)
            .width(Length::Fill)
            .height(Length::Fill)
            .center_x()
            .center_y()
            .into()
    }
}

fn every(duration: std::time::Duration) -> iced::Subscription<std::time::Instant> {
    iced::Subscription::from_recipe(Every(duration))
}

struct Every(std::time::Duration);

impl<H, I> iced_native::subscription::Recipe<H, I> for Every
where
    H: std::hash::Hasher,
{
    type Output = std::time::Instant;

    fn hash(&self, state: &mut H) {
        use std::hash::Hash;
        std::any::TypeId::of::<Self>().hash(state);
        self.0.hash(state);
    }

    fn stream(self: Box<Self>, _input: I) -> futures::stream::BoxStream<'static, Self::Output> {
        use futures::stream::StreamExt;
        async_std::stream::interval(self.0)
            .map(|_| std::time::Instant::now())
            .boxed()
    }
}
