use rsb_lib::{adder, gray_code, multiplier, BoolNode, BoolToken, Lexer, CharToken, VarNode};

use iced::{
    widget::{
        button, column, container, horizontal_space, pick_list, row, text, text_input, toggler,
        vertical_space, Column,
    },
    Color, Element, Theme, Alignment::Center, padding,
};

mod message;
mod screen;
mod tree_widget;
use message::Message;
use screen::Screen;
use tree_widget::{VizTree, adapter};

#[derive(Default)]
struct App {
    screen: Screen,
    uint_a: u32,
    uint_b: u32,
    error_string: Option<String>,
    debug: bool,
    input_string: String,
    boolrpn: String,
    bool_result: Option<bool>,
    viztree: VizTree,
    theme: Theme,
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
            Message::DebugToggle(value) => self.debug = value,
            Message::InputStringChange(value) => {
                self.input_string = value;
            }
            Message::EvalBoolRpn => {
                // validate the RPN
                let lexer = Lexer::new(self.input_string.as_str());
                if let Some(c) = lexer.scan_for_illegal::<BoolToken>() {
                    self.error_string =
                        Some(format!("Illegal character in the RPN input - '{}'", c));
                    self.bool_result = None;
                } else {
                    let rpn: BoolNode = lexer.into();
                    self.boolrpn = self.input_string.clone();
                    self.bool_result = Some(rpn.value());
                }
            },
            Message::EvalVarRpn => {
                // validate the RPN
                let lexer = Lexer::new(self.input_string.as_str());
                if let Some(c) = lexer.scan_for_illegal::<CharToken>() {
                    self.error_string =
                        Some(format!("Illegal character in the RPN input - '{}'", c));
                    self.bool_result = None;
                } else {
                    let rpn: VarNode = lexer.into();
                }
            }
            Message::ThemeSelected(theme) => self.theme = theme,
        }
    }

    fn view(&self) -> Element<Message> {
        let mut controls = row![];
        for screen in Screen::ALL {
            controls = controls.push(button(screen.as_str()).on_press(Message::from(*screen)));
            // controls = controls.push(horizontal_space());
        }
        controls = controls
            .push(horizontal_space())
            .push(
                toggler(self.debug)
                    .label("Debug View")
                    .on_toggle(Message::DebugToggle),
            )
            .padding(5).spacing(5).align_y(Center);

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
        container(if self.debug {
            content.explain(Color::WHITE)
        } else {
            content
        })
        .into()
    }

    fn scene_container(title: &str) -> Column<'_, Message> {
        column![text(title).size(50)].spacing(20).padding(10)
    }

    fn home(&self) -> Element<Message> {
        Self::scene_container("Ready Set Bool")
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

        Self::scene_container("Exercise 00 - Adder")
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

        Self::scene_container("Exercise 01 - Multiplier")
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
        let a: String = format!("{}", self.uint_a);

        let a_input = text_input("", a.as_str()).on_input(Message::IntInputA);

        let row_a = row!["N: ", a_input];

        let width = 100;

        let steps = column![
            row![
                text("GC(N - 1):").width(width),
                text(format!(
                    "{:032b}",
                    if self.uint_a > 0 {
                        gray_code(self.uint_a - 1)
                    } else {
                        0
                    }
                )),
            ],
            row![
                text("GC(N):").width(width),
                text(format!("{:032b}", gray_code(self.uint_a))),
            ],
            row![
                text("GC(N + 1):").width(width),
                text(format!("{:032b}", gray_code(self.uint_a + 1))),
            ],
        ];

        let result = format!("grey_code({}) = {}", self.uint_a, gray_code(self.uint_a));

        Self::scene_container("Exercise 02 - Gray code")
            .push(
                "Goal is to write a function that takes an integer n and \
                returns its equivalent in Gray code",
            )
            .push(row_a)
            .push(steps)
            .push(text(result).size(30))
            .into()
    }

    fn ex03(&self) -> Element<Message> {
        let rpn = &self.input_string;

        let rpn_input = text_input("", rpn).on_input(Message::InputStringChange);

        let row_a = row!["RPN: ", rpn_input];

        let eval_label = "Evaluate";

        let result = if let Some(value) = self.bool_result {
            format!("eval_formula(\"{}\") = {}", self.boolrpn, value)
        } else {
            format!(
                "Type RPN formula to the input field and press '{}'",
                eval_label
            )
        };

        Self::scene_container("Exercise 03 - Boolean evaluation")
            .push(
                "Goal is to write a function that takes as input a string \
                that contains a propositional formula in reverse polish \
                notation, evaluates this formula, then returns the result.",
            )
            .push(row_a)
            .push(button(eval_label).on_press(Message::EvalBoolRpn))
            .push(text(result).size(30))
            .into()
    }

    fn ex04(&self) -> Element<Message> {
        Self::scene_container("ex04").into()
    }

    fn ex05(&self) -> Element<Message> {
        let rpn = &self.input_string;

        let rpn_input = text_input("", rpn).on_input(Message::InputStringChange);

        let row_a = row!["RPN: ", rpn_input];

        let eval_label = "Evaluate";

        let result = if let Some(value) = self.bool_result {
            format!("eval_formula(\"{}\") = {}", self.boolrpn, value)
        } else {
            format!(
                "Type RPN formula to the input field and press '{}'",
                eval_label
            )
        };

        Self::scene_container("Exercise 05 - Negation Normal Form")
            .push(
                "Goal is to write a function that takes as input a string \
                that contains a propositional formula in reverse polish \
                notation, and returns an equivalent formula in Negation \
                Normal Form (NNF), meaning that every negation operators \
                must be located right after a variable. ",
            )
            .push(row_a)
            .push(button(eval_label).on_press(Message::EvalVarRpn))
            .push(text(result).size(30))
            .into()
    }

    fn ex06(&self) -> Element<Message> {
        Self::scene_container("ex06").into()
    }

    fn ex07(&self) -> Element<Message> {
        Self::scene_container("ex07").into()
    }

    fn ex08(&self) -> Element<Message> {
        Self::scene_container("ex08").into()
    }

    fn ex09(&self) -> Element<Message> {
        Self::scene_container("ex09").into()
    }

    fn ex10(&self) -> Element<Message> {
        Self::scene_container("ex10").into()
    }

    fn ex11(&self) -> Element<Message> {
        Self::scene_container("ex11").into()
    }
}

pub fn main() -> iced::Result {
    iced::application("42 Ready Set Bool", App::update, App::view)
        .centered()
        .run()
}

