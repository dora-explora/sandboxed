use color_eyre::Result;
use crossterm::event::{self, Event, KeyCode, KeyEvent, MouseEvent};
use ratatui::{
    buffer::Buffer,
    layout::Rect,
    text::{Line},
    widgets::{Block, BorderType, Paragraph, Widget},
    DefaultTerminal, Frame,
};

use std::thread;
use std::time::{Duration, Instant};
use std::sync::mpsc::{Sender, Receiver};

use crate::{sandbox_to_text, process_physics};

pub struct App {
    pub(crate) sandbox: Vec<Vec<u8>>,
    exit: bool,
}

pub enum Update {
    Key(crossterm::event::KeyEvent),
    Mouse(crossterm::event::MouseEvent),
    Resize(u16, u16),
    Sandbox(Vec<Vec<u8>>)
}

pub fn send_input_events(sender: Sender<Update>) {
    loop {
        match event::read().expect("Could not read crossterm events") {
            Event::Key(key_event) => sender.send(Update::Key(key_event)).expect("could not send key event along mpsc channel"),
            Event::Mouse(mouse_event) => sender.send(Update::Mouse(mouse_event)).expect("could not send mouse event along mpsc channel"),
            Event::Resize(x, y) => sender.send(Update::Resize(x, y)).expect("could not send resize event along mpsc channel"),
            _ => {}
        }
    }
}

pub fn run_sandbox_thread(sender: Sender<Update>) {
    let width: usize = (80 - 2) * 2;
    let height: usize = (20 - 2) * 4;
    let mut sandbox: Vec<Vec<u8>> = vec![vec![0; height]; width];
    let frame_time = Duration::from_millis(8);
    loop {
        let start_time = Instant::now();
        sandbox[width/2][0]= 1;
        sandbox = process_physics(&sandbox);
        sender.send(Update::Sandbox(sandbox.clone())).expect("could not send sandbox update along mpsc channel");
        let elapsed_time = start_time.elapsed();
        let sleep_time = frame_time.checked_sub(elapsed_time).unwrap_or(Duration::ZERO);
        thread::sleep(sleep_time);
    }
}

impl App {

    pub fn new(sandbox: Vec<Vec<u8>>) -> App {
        return App { 
            sandbox,
            exit: false
        };
    }

    pub fn exit(&mut self) {
        self.exit = true;
    }
    
    pub fn run(&mut self, terminal: &mut DefaultTerminal, reciever: Receiver<Update>) -> Result<()> {

        while !self.exit {
            terminal.draw(|frame| self.draw(frame))?;

            // handle updates
            match reciever.recv().expect("could not recieve updates along mpsc channel") {
                Update::Key(key_event) => self.handle_key_event(key_event),
                Update::Mouse(mouse_event) => self.handle_mouse_event(mouse_event),
                Update::Resize(x, y) => self.handle_resize_event(x, y),
                Update::Sandbox(sandbox) => self.sandbox = sandbox
            }
        }
        return Ok(());
    }

    fn draw(&self, frame: &mut Frame) {
        frame.render_widget(self, frame.area());
    }

    fn handle_key_event(&mut self, key_event: KeyEvent) {
        match key_event.code {
            KeyCode::Char('q') => self.exit(),
            _ => {}
        }
    }

    fn handle_mouse_event(&mut self, mouse_event: MouseEvent) {
        {}
    }

    fn handle_resize_event(&mut self, x: u16, y: u16) {
        {}
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