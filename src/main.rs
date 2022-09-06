mod action;
mod fuzzy;
mod parse_places;

use action::copier;
use fuzzy::search;
use iced::widget::{button, column, pick_list, text, text_input, Column, Text};
use iced::{executor, window, Alignment, Application, Command, Element, Renderer, Settings, Theme};
use ngrammatic::SearchResult;
use parse_places::get_firefox_entries;

// fn main() {
//     // let x = get_firefox_entries().unwrap();
//     // for item in x {
//     //     println!("{:?}", item)
//     // }
//     // typer("Hello World!");
// }

fn main() -> iced::Result {
    Counter::run(Settings {
        window: window::Settings {
            size: (600, 400),
            always_on_top: true,
            ..window::Settings::default()
        },
        ..Settings::default()
    })
}

struct Counter {
    value: i32,
    search_term: String,
    results: Vec<SearchResult>,
}

#[derive(Debug, Clone)]
enum Message {
    IncrementPressed,
    DecrementPressed,
    Submitted,
    EventOccurred(iced::Event),
    InputChanged(String),
    TypeClicked,
}

impl Application for Counter {
    type Message = Message;
    type Theme = Theme;
    type Executor = executor::Default;
    type Flags = ();

    fn new(_flags: ()) -> (Counter, Command<Message>) {
        (
            Self {
                value: 0,
                search_term: "placeholder".to_string(),
                results: Vec::new(),
            },
            Command::none(),
        )
    }

    fn title(&self) -> String {
        String::from("Niced Riced Iced App")
    }

    fn subscription(&self) -> iced::Subscription<Self::Message> {
        iced::subscription::events().map(Message::EventOccurred)
    }

    fn update(&mut self, message: Message) -> Command<Message> {
        let entries = get_firefox_entries().unwrap();
        let mut titles = Vec::new();

        for entry in entries {
            match entry.title {
                None => continue,
                Some(title) => titles.push(title),
            }
        }
        match message {
            Message::IncrementPressed => {
                self.value += 1;
            }
            Message::DecrementPressed => {
                self.value -= 1;
            }

            Message::Submitted => {
                self.value *= 2;
                if self.results.len() > 0 {
                    copier(self.results[0].text.as_str())
                }
            }
            Message::EventOccurred(event) => {
                // println!("{:#?}", event);
            }
            Message::InputChanged(string) => {
                // println!("{:#?}", string);
                self.search_term = string;
                self.results = search(
                    &self.search_term,
                    titles.iter().map(|s| s as &str).collect(),
                    0.1,
                );

                // println!("{:?}", self.results);
            }
            Message::TypeClicked => copier("HELLLOWROOOLD!"),
        };
        Command::none()
    }

    fn view(&self) -> Element<Message> {
        let input = text_input("Search here", &self.search_term, Message::InputChanged)
            .padding(15)
            .size(30)
            .on_submit(Message::Submitted);

        let mut c = Column::new();
        // for title in &titles[1..10] {
        //     c = c.push(Text::new(title));
        // }
        let l = self.results.len();
        if l > 0 {
            for title in &self.results[..l] {
                c = c.push(Text::new(&title.text).size(30));
            }
        }

        column![
            // button("Increment").on_press(Message::IncrementPressed),
            // text(self.value).size(50),
            // text(&self.search_term).size(20),
            input,
            // button("Decrement").on_press(Message::DecrementPressed),
            // button("Double").on_press(Message::DoublePressed),
            // button("Type Stuff").on_press(Message::TypeClicked),
            c
        ]
        .padding(20)
        .align_items(Alignment::Center)
        .into()
    }
}

// fn get_text(s: &str) -> widget::Text<Renderer> {
//     text(s)
// }

// fn grid(items: Vec<&str>) {
//     // let children: widget::Text<Renderer> = items.iter().map(|x| get_text(x)).collect();
//     let mut children: Vec<widget::Text> = vec![];
//     for item in items {
//         children.push(text(item))
//     }

// }
