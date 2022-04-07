use iced::button;
use iced::text_input;
use iced::{Button, Column, Row, Text, Color, Background, Vector, Space, TextInput};
use iced::{executor, Application, Clipboard, Command, Element, Settings};

#[derive(Debug, Clone)]
pub enum PressMessage {
    IncrementPressed,
    DecrementPressed,
    InputChanged(String)
}

pub struct MyButtonStyle;

impl button::StyleSheet for MyButtonStyle {
    fn active(&self) -> button::Style {
        button::Style{
            shadow_offset: Vector::new(0., 0.),
            background: Some(Background::Color(Color::from_rgb(191., 221., 255.))),
            border_radius: 25.,
            border_width: 50.,
            text_color: Color::WHITE,
            border_color: Color::BLACK
        }
    }
}

#[derive(Default)]
struct Counter {
    // The counter value
    value: i32,
    // txt value
    txt: String,
    input_state: text_input::State,

    // The local state of the two buttons
    increment_button: button::State,
    decrement_button: button::State,
}

impl Counter {
    pub fn view(&mut self) -> Row<PressMessage> {
        // We use a column: a simple vertical layout
        Row::new()
            .align_items(iced::Align::Center)
            .push(
                Space::with_width(iced::Length::FillPortion(1))
            )
            .push(
                Column::new()
                    .push(
                        Text::new("1").size(50)
                            .horizontal_alignment(iced::HorizontalAlignment::Center)
                    )
            )
            .push(
                Column::new()
                    .align_items(iced::Align::Center)
                    .push(
                        Space::with_width(iced::Length::FillPortion(1))
                    )
                    .push(
                        // The increment button. We tell it to produce an
                        // `IncrementPressed` message when pressed
                        Button::new(&mut self.increment_button, Text::new("+"))
                            .on_press(PressMessage::IncrementPressed)
                            .style(MyButtonStyle)
                            .padding(5),
                    )
                    .push(
                        // We show the value of the counter here
                        Text::new(self.value.to_string()).size(50)
                            .horizontal_alignment(iced::HorizontalAlignment::Center),
                    )
                    .push(
                        // The decrement button. We tell it to produce a
                        // `DecrementPressed` message when pressed
                        Button::new(&mut self.decrement_button, Text::new("-"))
                            .on_press(PressMessage::DecrementPressed)
                            .style(MyButtonStyle),
                    )
                    .push(
                        TextInput::new(
                            &mut self.input_state,
                            "What needs to be done?",
                            &self.txt,
                            PressMessage::InputChanged,
                        )
                    )
            )
    }

    pub fn update(&mut self, message: PressMessage) {
       match message {
           PressMessage::IncrementPressed => {
               self.value += 1;
           }
           PressMessage::DecrementPressed => {
               self.value -= 1;
           },
           PressMessage::InputChanged(val) => {
               self.txt = val.clone();
           }
       }
   }
}

impl Application for Counter {
    type Executor = executor::Default;
    type Message = PressMessage;
    type Flags = ();

    fn new(_flags: ()) -> (Self, Command<Self::Message>) {
        (Self{
            value: 0,
            txt: "".to_string(),
            input_state: text_input::State::new(),
            increment_button: button::State::new(),
            decrement_button: button::State::new()
        }, Command::none())
    }

    fn title(&self) -> String {
        String::from("A cool application")
    }

    fn update(&mut self, message: Self::Message, _clipboard: &mut Clipboard) -> Command<Self::Message> {
        self.update(message);
        Command::none()
    }

    #[inline]
    fn view(&mut self) -> Element<Self::Message> {
        self.view().into()
    }
}


fn main() -> iced::Result {
    Counter::run(Settings::default())
}
