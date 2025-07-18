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
    if a > 0 { index += 1;   }
    if b > 0 { index += 2;   }
    if c > 0 { index += 4;   }
    if d > 0 { index += 8;   }
    if e > 0 { index += 16;  }
    if f > 0 { index += 32;  }
    if g > 0 { index += 64;  }
    if h > 0 { index += 128; }
    return BLOCK_CHARS[index]
}

fn sandbox_to_text(sandbox: &Vec<Vec<u8>>) -> Vec<String> {
    let mut text: Vec<String> = vec![String::new()];
    for x in 0..sandbox.len()/2 {
        for y in 0..sandbox[0].len()/8 {
            let a = sandbox[2*x    ][6*y + 0]; 
            let b = sandbox[2*x + 1][6*y + 0]; 
            let c = sandbox[2*x    ][6*y + 1]; 
            let d = sandbox[2*x + 1][6*y + 1];
            let e = sandbox[2*x    ][6*y + 2];
            let f = sandbox[2*x + 1][6*y + 2]; 
            let g = sandbox[2*x    ][6*y + 3]; 
            let h = sandbox[2*x + 1][6*y + 3];
            text[y].push(block_to_char(a, b, c, d, e, f, g, h))
        }
    }
    return text
}

fn print_sandbox(text: Vec<String>) {
    for line in text {
        println!("{}", line)
    }
}

fn process_gravity(sandbox: &mut Vec<Vec<u8>>) {
    for y in (0..sandbox[0].len()).rev() {
        for x in 0..sandbox.len() {
            if sandbox[x][y] == 1 && y < (sandbox[0].len() - 1) {
                if sandbox[x][y+1] == 0 {
                    sandbox[x][y+1] = sandbox[x][y];
                    sandbox[x][y] = 0;
                } else if x > 0 { // pixel always falls left: this should be random
                    if sandbox[x-1][y+1] == 0 {  
                        sandbox[x-1][y+1] = sandbox[x][y];
                        sandbox[x][y] = 0;
                    }
                } else if x < (sandbox.len() - 1) { 
                    if sandbox[x+1][y+1] == 0 {
                        sandbox[x+1][y+1] = sandbox[x][y];
                        sandbox[x][y] = 0;
                    } 
                }
            }
        }
    }
}

fn main() {
    let mut sandbox: Vec<Vec<u8>> = vec![vec![1; 8]; 4];
    sandbox[0][2] = 0;
    print_sandbox(sandbox_to_text(&sandbox));
    process_gravity(&mut sandbox);
    print_sandbox(sandbox_to_text(&sandbox));
}