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

fn check_left_gravity(sandbox: &mut Vec<Vec<u8>>, x: usize, y: usize) {
    if x > 0 {
        if sandbox[x-1][y+1] == 0 {  
            sandbox[x-1][y+1] = sandbox[x][y];
            sandbox[x][y] = 0;
        }
    }
}

fn check_right_gravity(sandbox: &mut Vec<Vec<u8>>, x: usize, y: usize) {
    if x < (sandbox.len() - 1) { 
        if sandbox[x+1][y+1] == 0 {
            sandbox[x+1][y+1] = sandbox[x][y];
            sandbox[x][y] = 0;
        }
    }
}

fn process_gravity(sandbox: &mut Vec<Vec<u8>>) {
    for y in (0..sandbox[0].len()).rev() {
    for x in 0..sandbox.len() {
        if sandbox[x][y] == 1 && y < (sandbox[0].len() - 1) {
            if sandbox[x][y+1] == 0 {
                sandbox[x][y+1] = sandbox[x][y];
                sandbox[x][y] = 0;
            }
            if random_bool(0.5) {
            // if true {
                check_left_gravity(sandbox, x, y);
                check_right_gravity(sandbox, x, y);
            } else {
                check_right_gravity(sandbox, x, y);
                check_left_gravity(sandbox, x, y);
            }
        }
    }
    }
}

fn process_physics(sandbox: &mut Vec<Vec<u8>>) {
    process_gravity(sandbox);
}

enum Input {
    Key(KeyEvent),
    Mouse(MouseEvent),
    Resize(u16, u16),
}

pub struct App {
    pub(crate) sandbox: Vec<Vec<u8>>,
    receiver: Receiver<Input>,
    faucet_pos: usize,
    faucet_pouring: bool,
    last_frame_time: f64,
    exit: bool,
}

impl App {

    fn new(width: usize, height: usize, receiver: Receiver<Input>) -> App {
        return App { 
            sandbox: vec![vec![0; (height - 2) * 4]; (width - 2) * 2],
            receiver,
            faucet_pos: 0, 
            faucet_pouring: true,
            last_frame_time: 0., 
            exit: false
        };
    }

    fn exit(&mut self) {
        self.exit = true;
    }
    
    fn run(&mut self, terminal: &mut DefaultTerminal) -> Result<()> {
        const FRAME_TIME: Duration = Duration::from_millis(15);
        while !self.exit {
            let start_time = Instant::now();
            if self.faucet_pouring { self.sandbox[self.faucet_pos][0] = 1; }
            process_physics(&mut self.sandbox);
            self.handle_input();
            terminal.draw(|frame| self.draw(frame))?;
            let elapsed_time = start_time.elapsed();
            self.last_frame_time = elapsed_time.as_secs_f64();
            let sleep_time = FRAME_TIME.checked_sub(elapsed_time).unwrap_or(Duration::ZERO);
            thread::sleep(sleep_time);
        }
        return Ok(());
    }

    fn reset_sandbox(&mut self) {
        self.sandbox = vec![vec![0; self.sandbox[0].len()]; self.sandbox.len()];
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
            KeyCode::Char('q') => self.exit(),
            KeyCode::Left => if self.faucet_pos > 0 { self.faucet_pos -= 1 },
            KeyCode::Right => if self.faucet_pos < (self.sandbox.len() - 1) { self.faucet_pos += 1 },
            KeyCode::Up => self.faucet_pouring ^= true,
            KeyCode::Down => self.reset_sandbox(),
            _ => {}
        }
    }

    fn handle_mouse_event(&mut self, mouse_event: MouseEvent) {
        todo!();
    }

    fn handle_resize_event(&mut self, x: u16, y: u16) {
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
    let mut app = App::new(70, 20, receiver);
    thread::spawn(move || handle_input_events(sender));

    let mut terminal = ratatui::init();
    let result = app.run(&mut terminal);
    ratatui::restore();
    return result;
}
