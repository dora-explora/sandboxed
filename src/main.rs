use color_eyre::Result;
use crossterm::event::{self, Event, KeyCode, KeyEvent};
use ratatui::layout::Size;
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

pub struct App {
    pub(crate) sandbox: Vec<Vec<usize>>,
    receiver: Receiver<KeyEvent>,
    faucet_pos: usize,
    faucet_pouring: bool,
    faucet_color: usize,
    quit: bool,
}

impl App {

    fn new(width: u16, height: u16, receiver: Receiver<KeyEvent>) -> App {
        return App { 
            sandbox: vec![vec![0; (height as usize - 4) * 4]; (width as usize - 2) * 2],
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
            Ok(key_event) => self.handle_key_event(key_event),
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
}

fn send_input_events(sender: Sender<KeyEvent>) {
    loop {
        match event::read().expect("could not read crossterm events") {
            Event::Key(key_event) => sender.send(key_event).expect("could not send key event along inputs mpsc channel"),
            _ => {}
        }
    }
}

fn main() -> Result<()> {
    


    let mut terminal = ratatui::init();
    let size = match terminal.size() {
        Ok(size) => size,
        Err(_) => Size::new(80, 25)
    };
    let (sender, receiver) = channel::<KeyEvent>();
    let mut app = App::new(size.width, size.height, receiver);
    thread::spawn(move || send_input_events(sender));
    let result = app.run(&mut terminal);
    ratatui::restore();
    return result;
}
