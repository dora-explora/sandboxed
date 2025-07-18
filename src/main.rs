use color_eyre::Result;

use std::sync::mpsc;
use std::thread;
use rand::random_bool;

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

fn main() -> Result<()> {
    let mut app = tui::App::new(vec![vec![0]]);
    
    let (update_sender, update_reciever) = mpsc::channel::<tui::Update>();
    let input_sender = update_sender.clone();
    thread::spawn(move || {tui::send_input_events(input_sender);});
    let sandbox_sender = update_sender.clone();
    thread::spawn(move || {tui::run_sandbox_thread(sandbox_sender);});

    let mut terminal = ratatui::init();
    let result = app.run(&mut terminal, update_reciever);
    ratatui::restore();
    return result;
}