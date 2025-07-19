use ratatui::{
    buffer::Buffer,
    layout::{Alignment, Rect},
    style::{Style, Color, Stylize},
    text::{Line, Span, Text},
    widgets::{Block, BorderType, Paragraph, Widget},
    Frame,
};

use crate::App;

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

fn sandbox_to_text(sandbox: &Vec<Vec<u8>>) -> Text {
    let width = sandbox.len()/2;
    let height = sandbox[0].len()/4;
    let mut lines: Vec<Line> = vec![Line::raw(""); height];
    let mut spans: Vec<Vec<Span>> = Vec::new();
    for y in 0..height {
        for x in 0..width {
            let a = sandbox[2*x    ][4*y + 0]; 
            let b = sandbox[2*x + 1][4*y + 0]; 
            let c = sandbox[2*x    ][4*y + 1]; 
            let d = sandbox[2*x + 1][4*y + 1];
            let e = sandbox[2*x    ][4*y + 2];
            let f = sandbox[2*x + 1][4*y + 2]; 
            let g = sandbox[2*x    ][4*y + 3]; 
            let h = sandbox[2*x + 1][4*y + 3];
            let span = block_to_char(a, b, c, d, e, f, g, h).to_string();
            lines[y].push_span(span);
        }
    }
    
    return Text {
        alignment: Some(Alignment::Center),
        style: Style::new(),
        lines
    };
}



impl App {
    pub fn draw(&self, frame: &mut Frame) {
        frame.render_widget(self, frame.area());
    }
}

impl Widget for &App {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let title = Line::from(" sandboxed ").centered();
        // let instructions: Line<'_> = Line::from("| press 'q' to exit | use ← and → to move the faucet | press ↑ to stop pouring | press ↓ to reset sandbox |").centered();        let instructions: Line<'_> = Line::from("| press 'q' to exit | use ← and → to move the faucet | press ↑ to stop pouring | press ↓ to reset sandbox |").centered();
        let instructions: Line<'_> = Line::from(format!("| {}ms |", self.last_frame_time * 1000.)).centered();
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