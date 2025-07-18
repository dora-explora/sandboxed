use color_eyre::{eyre::bail, Result};
use crossterm::event::{self, Event, KeyCode, KeyEvent, KeyEventKind};
use ratatui::{
    buffer::Buffer,
    layout::Rect,
    text::{Line},
    widgets::{Block, BorderType, Paragraph, Widget},
    DefaultTerminal, Frame,
};

    use crate::{process_gravity, sandbox_to_text};

#[derive(Debug, Default)]
pub struct App {
    sandbox: Vec<Vec<u8>>,
    exit: bool,
}

impl App {

    pub fn new(x: usize, y: usize) -> App {
        return App { 
            sandbox: vec![vec![0; y]; x],
            exit: false
        };
    }

    pub fn exit(&mut self) -> Result<()> {
        self.exit = true;
        return Ok(());
    }
    
    pub fn run(&mut self, terminal: &mut DefaultTerminal) -> Result<()> {
        while !self.exit {
            self.sandbox[10][0] = 1;
            process_gravity(&mut self.sandbox);
            terminal.draw(|frame| self.draw(frame))?;
            self.handle_events()?;
        }
        return Ok(());
    }

    fn draw(&self, frame: &mut Frame) {
        frame.render_widget(self, frame.area());
    }

    fn handle_events(&mut self) -> Result<()> {
        return match event::read()? {
            Event::Key(key_event) if key_event.kind == KeyEventKind::Press => {
                self.handle_key_event(key_event)
            }
            _ => Ok(())
        };
    }

    fn handle_key_event(&mut self, key_event: KeyEvent) -> Result<()> {
        match key_event.code {
            KeyCode::Char('q') => self.exit(),
            _ => Ok(())
        }
    }
}

impl Widget for &App {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let title = Line::from(" sandboxed ").centered();
        let block = Block::bordered()
            .border_type(BorderType::Rounded)
            .title(title);

        Paragraph::new(sandbox_to_text(&self.sandbox))
            .centered()
            .block(block)
            .render(area, buf);
    }
}