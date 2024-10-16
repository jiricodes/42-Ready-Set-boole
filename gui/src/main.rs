use rsb_lib::{adder, multiplier};
use std::default;

use iced::{
    widget::{button, column, container, horizontal_space, row, text, text_input, Column, vertical_space},
    Element,
    Length::Fill, Color,
};

#[derive(Default)]
struct App {
    screen: Screen,
    uint_a: u32,
    uint_b: u32,
    error_string: Option<String>,
}

impl App {
    fn update(&mut self, msg: Message) {
        match msg {
            Message::Screen(value) => self.screen = value,
            Message::IntInputA(value) => {
                if let Ok(x) = value.parse() {
                    self.uint_a = x;
                    self.error_string = None;
                } else {
                    self.error_string = Some(format!("Failed to parse \"{}\" into u32", value));
                }
            }
            Message::IntInputB(value) => {
                if let Ok(x) = value.parse() {
                    self.uint_b = x;
                    self.error_string = None;
                } else {
                    self.error_string = Some(format!("Failed to parse \"{}\" into u32", value));
                }
            }
        }
    }

    fn view(&self) -> Element<Message> {
        let mut controls = row![horizontal_space()];
        for screen in Screen::ALL {
            controls = controls.push(button(screen.as_str()).on_press(Message::from(*screen)));
            // controls = controls.push(horizontal_space());
        }
        controls = controls.push(horizontal_space());

        let screen = match self.screen {
            Screen::Home => self.home(),
            Screen::Ex00 => self.ex00(),
            Screen::Ex01 => self.ex01(),
            Screen::Ex02 => self.ex02(),
            Screen::Ex03 => self.ex03(),
            Screen::Ex04 => self.ex04(),
            Screen::Ex05 => self.ex05(),
            Screen::Ex06 => self.ex06(),
            Screen::Ex07 => self.ex07(),
            Screen::Ex08 => self.ex08(),
            Screen::Ex09 => self.ex09(),
            Screen::Ex10 => self.ex10(),
            Screen::Ex11 => self.ex11(),
        };

        let mut status_bar = row![];
        if let Some(error_text) = &self.error_string {
            status_bar = status_bar.push(text(error_text).color(Color::from_rgb8(255, 20, 20)));
        }

        let content: Element<_> = column![controls, screen, vertical_space(), status_bar].into();

        // container(content).center_y(Fill).into()
        container(content).into()
    }
    fn home(&self) -> Element<Message> {
        column![text("Ready Set Bool").size(50)]
            .spacing(20)
            .push("An introduction to Boolean Algebra")
            .into()
    }

    fn ex00(&self) -> Element<Message> {
        let a: String = format!("{}", self.uint_a);
        let b: String = format!("{}", self.uint_b);

        let a_input = text_input("", a.as_str()).on_input(Message::IntInputA);
        let b_input = text_input("", b.as_str()).on_input(Message::IntInputB);

        let row_a = row!["a: ", a_input];
        let row_b = row!["b: ", b_input];

        let result = format!(
            "Result: {} + {} = {}",
            self.uint_a,
            self.uint_b,
            adder(self.uint_a, self.uint_b)
        );

        column![text("Exercise 00 - Adder").size(50)]
            .spacing(20)
            .push(
                "Goal is to write a function that takes as parameters \
                two natural numbers a and b and returns one natural number \
                that equals a + b. The caveat is that we can use only bitwise \
                operations, assignment and comparison operators.",
            )
            .push(
                "The incrementation operator (++ or += 1) is allowed only \
                to increment the index of a loop and must not be used \
                to compute the result itself.",
            )
            .push(
                "Solution: Simple recursive solution with calling add on \
                carry and result until carry is 0. Since we're working with \
                32bit ints there should be no stack overflow. Rewrite into \
                looped solution is fairly easy. But neither the recursive or \
                the looped solutions could prevent int overflow - that one \
                exists but is more complex.",
            )
            .push(row_a)
            .push(row_b)
            .push(text(result).size(30))
            .into()
    }

    fn ex01(&self) -> Element<Message> {
        let a: String = format!("{}", self.uint_a);
        let b: String = format!("{}", self.uint_b);

        let a_input = text_input("", a.as_str()).on_input(Message::IntInputA);
        let b_input = text_input("", b.as_str()).on_input(Message::IntInputB);

        let row_a = row!["a: ", a_input];
        let row_b = row!["b: ", b_input];

        let result = format!(
            "Result: {} * {} = {}",
            self.uint_a,
            self.uint_b,
            multiplier(self.uint_a, self.uint_b)
        );

        column![text("Exercise 01 - Multiplier").size(50)]
            .spacing(20)
            .push(
                "Goal is to write a function that takes as parameters \
                two natural numbers a and b and returns one natural number \
                that equals a * b. The caveat is that we can use only bitwise \
                operations, assignment and comparison operators.",
            )
            .push(
                "The incrementation operator (++ or += 1) is allowed only \
                to increment the index of a loop and must not be used \
                to compute the result itself.",
            )
            .push(
                "Solution: Has been achieved by implementing the Russian \
                peasant method. Where while we have B > 0 we add A to the \
                result, but only if B is odd. Then we half B and double A \
                and repeat.See incode comments for sources.",
            )
            .push(row_a)
            .push(row_b)
            .push(text(result).size(30))
            .into()
    }

    fn ex02(&self) -> Element<Message> {
        column![text("ex02").size(50)].spacing(20).into()
    }

    fn ex03(&self) -> Element<Message> {
        column![text("ex03").size(50)].spacing(20).into()
    }

    fn ex04(&self) -> Element<Message> {
        column![text("ex04").size(50)].spacing(20).into()
    }

    fn ex05(&self) -> Element<Message> {
        column![text("ex05").size(50)].spacing(20).into()
    }

    fn ex06(&self) -> Element<Message> {
        column![text("ex06").size(50)].spacing(20).into()
    }

    fn ex07(&self) -> Element<Message> {
        column![text("ex07").size(50)].spacing(20).into()
    }

    fn ex08(&self) -> Element<Message> {
        column![text("ex08").size(50)].spacing(20).into()
    }

    fn ex09(&self) -> Element<Message> {
        column![text("ex09").size(50)].spacing(20).into()
    }

    fn ex10(&self) -> Element<Message> {
        column![text("ex10").size(50)].spacing(20).into()
    }

    fn ex11(&self) -> Element<Message> {
        column![text("ex11").size(50)].spacing(20).into()
    }
}

#[derive(Default, Debug, Clone, Copy)]
enum Screen {
    #[default]
    Home,
    Ex00,
    Ex01,
    Ex02,
    Ex03,
    Ex04,
    Ex05,
    Ex06,
    Ex07,
    Ex08,
    Ex09,
    Ex10,
    Ex11,
}

impl Screen {
    const ALL: &'static [Self] = &[
        Self::Home,
        Self::Ex00,
        Self::Ex01,
        Self::Ex02,
        Self::Ex03,
        Self::Ex04,
        Self::Ex05,
        Self::Ex06,
        Self::Ex07,
        Self::Ex08,
        Self::Ex09,
        Self::Ex10,
        Self::Ex11,
    ];

    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Home => "Home",
            Self::Ex00 => "Ex00",
            Self::Ex01 => "Ex01",
            Self::Ex02 => "Ex02",
            Self::Ex03 => "Ex03",
            Self::Ex04 => "Ex04",
            Self::Ex05 => "Ex05",
            Self::Ex06 => "Ex06",
            Self::Ex07 => "Ex07",
            Self::Ex08 => "Ex08",
            Self::Ex09 => "Ex09",
            Self::Ex10 => "Ex10",
            Self::Ex11 => "Ex11",
        }
    }
}

#[derive(Clone, Debug)]
enum Message {
    Screen(Screen),
    IntInputA(String),
    IntInputB(String),
}

impl From<Screen> for Message {
    fn from(value: Screen) -> Self {
        Self::Screen(value)
    }
}

pub fn main() -> iced::Result {
    iced::application("42 Ready Set Bool", App::update, App::view)
        .centered()
        .run()
}

