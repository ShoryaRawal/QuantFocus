use iced::sandbox::{Sandbox};
use iced::widget::{button, Button, column, Column, scrollable, Scrollable, text, Text};
use iced::{Alignment, Element, Length, Settings};

pub struct SemSimulatorApp {
    scroll: scrollable::State,
    run_btn: button::State,
    result_msg: Option<String>, // placeholder for image/results
}

#[derive(Debug, Clone)]
pub enum Message {
    RunSimulation,
}

impl Sandbox for SemSimulatorApp {
    type Message = Message;

    fn new() -> Self {
        Self {
            scroll: scrollable::State::new(),
            run_btn: button::State::new(),
            result_msg: None,
        }
    }

    fn title(&self) -> String {
        String::from("SEM Simulator")
    }

    fn update(&mut self, message: Message) {
        match message {
            Message::RunSimulation => {
                // TODO: call SimulationManager and get image buffer
                self.result_msg = Some(String::from("Simulation ran: result available!"));
            }
        }
    }

    fn view(&self) -> Element<Message> {
        let content = column![
            Button::new(&self.run_btn, Text::new("Run Simulation")).on_press(Message::RunSimulation),
            if let Some(msg) = &self.result_msg {
                Text::new(msg).size(16)
            } else {
                Text::new("No result yet.").size(16)
            }
        ]
        .spacing(20)
        .align_items(Alignment::Center)
        .padding(20);

        Scrollable::new(&self.scroll)
            .padding(10)
            .width(Length::Fill)
            .height(Length::Fill)
            .align_items(Alignment::Center)
            .push(content)
            .into()
    }
}
