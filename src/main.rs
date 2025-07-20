use color_eyre::Result;
use crossterm::event::{self, Event, KeyCode, KeyEvent, MouseEvent};
use ratatui::DefaultTerminal;

use std::sync::mpsc::{channel, Receiver, Sender};
use std::thread;
use rand::random_bool;
use std::time::{Duration, Instant};

mod tui;

// fn print_sandbox(text: Vec<String>) {
//     for line in text {
//         println!("{}", line)
//     }
// }

enum Input {
    Key(KeyEvent),
    Mouse(MouseEvent),
    Resize(u16, u16),
}

pub struct App {
    pub(crate) sandbox: Vec<Vec<usize>>,
    receiver: Receiver<Input>,
    faucet_pos: usize,
    faucet_pouring: bool,
    faucet_color: usize,
    quit: bool,
}

impl App {

    fn new(width: usize, height: usize, receiver: Receiver<Input>) -> App {
        return App { 
            sandbox: vec![vec![0; (height - 4) * 4]; (width - 2) * 2],
            receiver,
            faucet_pos: 0, 
            faucet_pouring: true,
            faucet_color: 1, 
            quit: false
        };
    }

    fn quit(&mut self) {
        self.quit = true;
    }
    
    fn run(&mut self, terminal: &mut DefaultTerminal) -> Result<()> {
        const FRAME_TIME: Duration = Duration::from_millis(18);
        while !self.quit {
            let start_time = Instant::now();
            if self.faucet_pouring { self.sandbox[self.faucet_pos][0] = self.faucet_color; }
            else { self.sandbox[self.faucet_pos][0] = 0; }
            self.process_physics();
            self.handle_input();
            self.sandbox[self.faucet_pos][0] = self.faucet_color;
            terminal.draw(|frame| self.draw(frame))?;
            let elapsed_time = start_time.elapsed();
            let sleep_time = FRAME_TIME.checked_sub(elapsed_time).unwrap_or(Duration::ZERO);
            thread::sleep(sleep_time);
        }
        return Ok(());
    }

    fn reset_sandbox(&mut self) {
        self.sandbox = vec![vec![0; self.sandbox[0].len()]; self.sandbox.len()];
    }

    fn process_physics(&mut self) {
        self.process_gravity();
    }

    fn process_gravity(&mut self) {
        for y in (0..self.sandbox[0].len()).rev() {
        for x in 0..self.sandbox.len() {
            if self.sandbox[x][y] > 0 && y < (self.sandbox[0].len() - 1) {
                if self.sandbox[x][y+1] == 0 {
                    self.sandbox[x][y+1] = self.sandbox[x][y];
                    self.sandbox[x][y] = 0;
                }
                if random_bool(0.5) {
                // if true {
                    self.check_left_gravity(x, y);
                    self.check_right_gravity(x, y);
                } else {
                    self.check_right_gravity(x, y);
                    self.check_left_gravity(x, y);
                }
            }
        }
        }
    }

    fn check_left_gravity(&mut self, x: usize, y: usize) {
        if x > 0 {
            if self.sandbox[x-1][y+1] == 0 {  
                self.sandbox[x-1][y+1] = self.sandbox[x][y];
                self.sandbox[x][y] = 0;
            }
        }
    }

    fn check_right_gravity(&mut self, x: usize, y: usize) {
        if x < (self.sandbox.len() - 1) { 
            if self.sandbox[x+1][y+1] == 0 {
                self.sandbox[x+1][y+1] = self.sandbox[x][y];
                self.sandbox[x][y] = 0;
            }
        }
    }

    fn handle_input(&mut self) {
        match self.receiver.try_recv() {
            Ok(Input::Key(key_event)) => self.handle_key_event(key_event),
            Ok(Input::Mouse(mouse_event)) => self.handle_mouse_event(mouse_event),
            Ok(Input::Resize(x, y)) => self.handle_resize_event(x, y),
            Err(_) => {}
        }
    }

    fn handle_key_event(&mut self, key_event: KeyEvent) {
        match key_event.code {
            KeyCode::Char('q') => self.quit(),
            KeyCode::Left => if self.faucet_pos > 0 { self.faucet_pos -= 1 },
            KeyCode::Right => if self.faucet_pos < (self.sandbox.len() - 1) { self.faucet_pos += 1 },
            KeyCode::Up => self.faucet_pouring ^= true,
            KeyCode::Down => self.reset_sandbox(),
            KeyCode::Enter => self.switch_color(),
            _ => {}
        }
    }

    fn handle_mouse_event(&mut self, _mouse_event: MouseEvent) {
        {}
    }

    fn handle_resize_event(&mut self, _x: u16, _y: u16) {
        {}
    }
}

fn handle_input_events(sender: Sender<Input>) {
    loop {
        match event::read().expect("could not read crossterm events") {
            Event::Key(key_event) => sender.send(Input::Key(key_event)).expect("could not send key event along inputs mpsc channel"),
            Event::Mouse(mouse_event) => sender.send(Input::Mouse(mouse_event)).expect("could not send mouse event along inputs mpsc channel"),
            Event::Resize(x, y) => sender.send(Input::Resize(x, y)).expect("could not send resize event along inputs mpsc channel"),
            _ => {}
        }
    }
}

fn main() -> Result<()> {
    
    let (sender, receiver) = channel::<Input>();
    let mut app = App::new(130, 25, receiver);
    thread::spawn(move || handle_input_events(sender));

    let mut terminal = ratatui::init();
    let result = app.run(&mut terminal);
    ratatui::restore();
    return result;
}
