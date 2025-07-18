use color_eyre::{Result};
use ratatui::{
    buffer::Buffer,
    layout::Rect,
    style::{Color, Stylize},
    text::{Line, Text},
    widgets::{Block, BorderType, Paragraph, Widget},
    DefaultTerminal, Frame,
};

use std::sync::mpsc::Receiver;

use crate::{sandbox_to_text, Update};

pub struct TUI {
    pub(crate) sandbox: Vec<Vec<u8>>,
    pub(crate) receiver: Receiver<Update>,
    exit: bool,
}

impl TUI {

    pub fn new(sandbox: Vec<Vec<u8>>, receiver: Receiver<Update>) -> TUI {
        return TUI { 
            sandbox,
            receiver,
            exit: false
        };
    }

    pub fn exit(&mut self) {
        self.exit = true;
    }
    
    pub fn run(&mut self, terminal: &mut DefaultTerminal) -> Result<()> {

        while !self.exit {
            terminal.draw(|frame| self.draw(frame))?;

            // handle updates
            match self.receiver.recv().expect("could not recieve updates along updates mpsc channel") {
                Update::Exit() => self.exit(),
                Update::Sandbox(sandbox) => self.sandbox = sandbox
            }
        }
        return Ok(());
    }

    fn draw(&self, frame: &mut Frame) {
        frame.render_widget(self, frame.area());
    }
}

impl Widget for &TUI {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let title = Line::from(" sandboxed ").centered();
        let instructions = Line::from(" press 'q' to exit ").centered();
        let block = Block::bordered()
            .border_type(BorderType::Rounded)
            .title(title)
            .title_bottom(instructions);

        let sandbox_text = Text::from(sandbox_to_text(&self.sandbox)).fg(Color::Yellow);

        Paragraph::new(sandbox_text)
            .centered()
            .block(block)
            .render(area, buf);
    }
}