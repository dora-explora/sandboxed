use color_eyre::Result;
use crossterm::event::{self, Event, KeyCode, KeyEvent, MouseEvent};

use std::sync::mpsc::{self, Receiver, Sender};
use std::thread;
use rand::random_bool;
use std::time::{Duration, Instant};

mod tui;

const BLOCK_CHARS: [char; 256] = [ // THANK YOU SO MUCH https://wiki.ourworldoftext.com/wiki/Octant I LOVE YOU
    ' ', '𜺨', '𜺫', '🮂', '𜴀', '▘', '𜴁', '𜴂', '𜴃', '𜴄', '▝', '𜴅', '𜴆', '𜴇', '𜴈', '▀',
    '𜴉', '𜴊', '𜴋', '𜴌', '🯦', '𜴍', '𜴎', '𜴏', '𜴐', '𜴑', '𜴒', '𜴓', '𜴔', '𜴕', '𜴖', '𜴗',
    '𜴘', '𜴙', '𜴚', '𜴛', '𜴜', '𜴝', '𜴞', '𜴟', '🯧', '𜴠', '𜴡', '𜴢', '𜴣', '𜴤', '𜴥', '𜴦',
    '𜴧', '𜴨', '𜴩', '𜴪', '𜴫', '𜴬', '𜴭', '𜴮', '𜴯', '𜴰', '𜴱', '𜴲', '𜴳', '𜴴', '𜴵', '🮅',
    '𜺣', '𜴶', '𜴷', '𜴸', '𜴹', '𜴺', '𜴻', '𜴼', '𜴽', '𜴾', '𜴿', '𜵀', '𜵁', '𜵂', '𜵃', '𜵄',
    '▖', '𜵅', '𜵆', '𜵇', '𜵈', '▌', '𜵉', '𜵊', '𜵋', '𜵌', '▞', '𜵍', '𜵎', '𜵏', '𜵐', '▛',
    '𜵑', '𜵒', '𜵓', '𜵔', '𜵕', '𜵖', '𜵗', '𜵘', '𜵙', '𜵚', '𜵛', '𜵜', '𜵝', '𜵞', '𜵟', '𜵠',
    '𜵡', '𜵢', '𜵣', '𜵤', '𜵥', '𜵦', '𜵧', '𜵨', '𜵩', '𜵪', '𜵫', '𜵬', '𜵭', '𜵮', '𜵯', '𜵰',
    '𜺠', '𜵱', '𜵲', '𜵳', '𜵴', '𜵵', '𜵶', '𜵷', '𜵸', '𜵹', '𜵺', '𜵻', '𜵼', '𜵽', '𜵾', '𜵿',
    '𜶀', '𜶁', '𜶂', '𜶃', '𜶄', '𜶅', '𜶆', '𜶇', '𜶈', '𜶉', '𜶊', '𜶋', '𜶌', '𜶍', '𜶎', '𜶏',
    '▗', '𜶐', '𜶑', '𜶒', '𜶓', '▚', '𜶔', '𜶕', '𜶖', '𜶗', '▐', '𜶘', '𜶙', '𜶚', '𜶛', '▜',
    '𜶜', '𜶝', '𜶞', '𜶟', '𜶠', '𜶡', '𜶢', '𜶣', '𜶤', '𜶥', '𜶦', '𜶧', '𜶨', '𜶩', '𜶪', '𜶫',
    '▂', '𜶬', '𜶭', '𜶮', '𜶯', '𜶰', '𜶱', '𜶲', '𜶳', '𜶴', '𜶵', '𜶶', '𜶷', '𜶸', '𜶹', '𜶺',
    '𜶻', '𜶼', '𜶽', '𜶾', '𜶿', '𜷀', '𜷁', '𜷂', '𜷃', '𜷄', '𜷅', '𜷆', '𜷇', '𜷈', '𜷉', '𜷊',
    '𜷋', '𜷌', '𜷍', '𜷎', '𜷏', '𜷐', '𜷑', '𜷒', '𜷓', '𜷔', '𜷕', '𜷖', '𜷗', '𜷘', '𜷙', '𜷚',
    '▄', '𜷛', '𜷜', '𜷝', '𜷞', '▙', '𜷟', '𜷠', '𜷡', '𜷢', '▟', '𜷣', '▆', '𜷤', '𜷥', '█'
];

fn block_to_char(a: u8, b: u8, c: u8, d: u8, e: u8, f: u8, g: u8, h: u8) -> char {
    let mut index = 0;
    if a > 0 { 
        index += 1;   }
    if b > 0 { 
        index += 2;   }
    if c > 0 { 
        index += 4;   }
    if d > 0 { 
        index += 8;   }
    if e > 0 { 
        index += 16;  }
    if f > 0 { 
        index += 32;  }
    if g > 0 { 
        index += 64;  }
    if h > 0 { 
        index += 128; }
    return BLOCK_CHARS[index]
}

fn sandbox_to_text(sandbox: &Vec<Vec<u8>>) -> String {
    let mut lines: Vec<String> = vec![String::new(); sandbox[0].len()/4];
    for x in 0..sandbox.len()/2 {
        for y in 0..sandbox[0].len()/4 {
            let a = sandbox[2*x    ][4*y + 0]; 
            let b = sandbox[2*x + 1][4*y + 0]; 
            let c = sandbox[2*x    ][4*y + 1]; 
            let d = sandbox[2*x + 1][4*y + 1];
            let e = sandbox[2*x    ][4*y + 2];
            let f = sandbox[2*x + 1][4*y + 2]; 
            let g = sandbox[2*x    ][4*y + 3]; 
            let h = sandbox[2*x + 1][4*y + 3];
            lines[y].push(block_to_char(a, b, c, d, e, f, g, h))
        }
    }
    // lines.insert(0, String::from_utf8(vec![0x23; sandbox.len()/2]).expect("could not add hashtags at start of sandbox text"));
    // lines.push(String::from_utf8(vec![0x23; sandbox.len()/2]).expect("could not add hashtags at end of sandbox text"));
    let mut text = String::new();
    for line in lines {
        // text.push('#');
        text.push_str(&line.as_str());
        // text.push('#');
        text.push('\n');
    }
    return text
}

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

fn process_gravity(mut sandbox: Vec<Vec<u8>>) -> Vec<Vec<u8>> {
    for y in (0..sandbox[0].len()).rev() {
    for x in 0..sandbox.len() {
        if sandbox[x][y] == 1 && y < (sandbox[0].len() - 1) {
            if sandbox[x][y+1] == 0 {
                sandbox[x][y+1] = sandbox[x][y];
                sandbox[x][y] = 0;
            }
            if random_bool(0.5) {
                check_left_gravity(&mut sandbox, x, y);
                check_right_gravity(&mut sandbox, x, y);
            } else {
                check_right_gravity(&mut sandbox, x, y);
                check_left_gravity(&mut sandbox, x, y);
            }
        }
    }
    }
    return sandbox;
}

fn process_physics(old_sandbox: &Vec<Vec<u8>>) -> Vec<Vec<u8>> {
    let mut sandbox = old_sandbox.clone();
    sandbox = process_gravity(sandbox);
    return sandbox;
}


pub enum Update {
    Exit(),
    Sandbox(Vec<Vec<u8>>)
}

pub enum Input {
    Key(KeyEvent),
    Mouse(MouseEvent),
    Resize(u16, u16),
}

struct Simulation {
    update_sender: Sender<Update>,
    input_receiver: Receiver<Input>,
    sandbox: Vec<Vec<u8>>,
    faucet_pos: usize,
    faucet_pouring: bool
}

impl Simulation {

    fn new(update_sender: Sender<Update>, input_receiver: Receiver<Input>, width: usize, height: usize) -> Simulation {
        Simulation {
            update_sender,
            input_receiver,
            sandbox: vec![vec![0; (height - 2) * 4]; (width - 2) * 2],
            faucet_pos: 0,
            faucet_pouring: false,
        }
    }

    fn run_sandbox_thread(&mut self) {
        let frame_time = Duration::from_millis(8);
        loop {
            let start_time = Instant::now();
            if self.faucet_pouring { self.sandbox[self.faucet_pos][0] = 1; }
            self.sandbox = process_physics(&self.sandbox);
            self.update_sender.send(Update::Sandbox(self.sandbox.clone())).expect("could not send sandbox update along mpsc channel");
            self.handle_input();
            let elapsed_time = start_time.elapsed();
            let sleep_time = frame_time.checked_sub(elapsed_time).unwrap_or(Duration::ZERO);
            thread::sleep(sleep_time);
        }
    }

    fn handle_input(&mut self) {
        match self.input_receiver.try_recv() {
            Ok(Input::Key(key_event)) => self.handle_key_event(key_event),
            Ok(Input::Mouse(mouse_event)) => self.handle_mouse_event(mouse_event),
            Ok(Input::Resize(x, y)) => self.handle_resize_event(x, y),
            Err(_) => {}
        }
    }

    fn handle_key_event(&mut self, key_event: KeyEvent) {
        match key_event.code {
            KeyCode::Char('q') => self.update_sender.send(Update::Exit()).expect("could not send exit event along mpsc channel"),
            KeyCode::Left => if self.faucet_pos > 0 { self.faucet_pos -= 1 },
            KeyCode::Right => if self.faucet_pos < (self.sandbox.len() - 1) { self.faucet_pos += 1 },
            KeyCode::Up => self.faucet_pouring ^= true,
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
    
    let (update_sender, update_receiver) = mpsc::channel::<Update>();
    let (input_sender, input_receiver) = mpsc::channel::<Input>();
    let mut tui = tui::TUI::new(vec![vec![0]], update_receiver);
    let mut sim = Simulation::new(update_sender, input_receiver, 80, 20);
    thread::spawn(move || {sim.run_sandbox_thread();});
    thread::spawn(move || handle_input_events(input_sender));

    let mut terminal = ratatui::init();
    let result = tui.run(&mut terminal);
    ratatui::restore();
    return result;
}