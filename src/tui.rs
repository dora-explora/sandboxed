use ratatui::{
    buffer::Buffer,
    layout::{Alignment, Rect},
    style::{Color, Style},
    text::{Line, Span, Text},
    widgets::{Block, BorderType, Widget},
    Frame,
};
use counter::Counter;

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

const PIXEL_COLORS: [Color; 5] = [Color::Black, Color::Yellow, Color::Blue, Color::Green, Color::Magenta];

fn block_to_char(block: [usize; 8], reference: usize) -> char {
    let mut index = 0;
    if block[0] == reference { 
        index += 1;   }
    if block[1] == reference { 
        index += 2;   }
    if block[2] == reference { 
        index += 4;   }
    if block[3] == reference { 
        index += 8;   }
    if block[4] == reference { 
        index += 16;  }
    if block[5] == reference { 
        index += 32;  }
    if block[6] == reference { 
        index += 64;  }
    if block[7] == reference { 
        index += 128; }
    return BLOCK_CHARS[index]
}


impl App {
    fn sandbox_to_text(&self) -> Text {
        let width = self.sandbox.len()/2;
        let height = self.sandbox[0].len()/4;
        let mut spans: Vec<Vec<Span>> = vec![vec![]; height];
        let mut block: [usize; 8] = [0; 8];
        for y in 0..height {
            for x in 0..width {
                block[0] = self.sandbox[2*x    ][4*y + 0]; 
                block[1] = self.sandbox[2*x + 1][4*y + 0]; 
                block[2] = self.sandbox[2*x    ][4*y + 1];
                block[3] = self.sandbox[2*x + 1][4*y + 1];
                block[4] = self.sandbox[2*x    ][4*y + 2];
                block[5] = self.sandbox[2*x + 1][4*y + 2]; 
                block[6] = self.sandbox[2*x    ][4*y + 3]; 
                block[7] = self.sandbox[2*x + 1][4*y + 3];
                let mut style: Style = Style::new();
                let mut foreground: usize = PIXEL_COLORS.len() + 1;
                let counts = block.iter().collect::<Counter<_>>().most_common_ordered();
                if counts.len() == 1 && *counts[0].0 > 0 { 
                    style = style.fg(PIXEL_COLORS[*counts[0].0]); 
                    foreground = *counts[0].0;
                } else if counts.len() == 2 { 
                    if *counts[0].0 == 0 { 
                        style = style.fg(PIXEL_COLORS[*counts[1].0]); 
                        foreground = *counts[1].0;
                    } else if *counts[1].0 == 0 { 
                        style = style.fg(PIXEL_COLORS[*counts[0].0]); 
                        foreground = *counts[0].0;
                    } else {
                        style = style.bg(PIXEL_COLORS[*counts[0].0]);
                        style = style.fg(PIXEL_COLORS[*counts[1].0]);
                        foreground = *counts[1].0;
                    }
                } else if counts.len() > 2 {
                    let background: usize;
                    if *counts[0].0 == 0 { 
                        style = style.fg(PIXEL_COLORS[*counts[1].0]);
                        foreground = *counts[1].0;
                        background = *counts[2].0;
                    } else { 
                        style = style.fg(PIXEL_COLORS[*counts[0].0]);
                        foreground = *counts[0].0;
                        if *counts[1].0 != 0 && *counts[2].0 != 0 { 
                            background = *counts[1].0; 
                            style = style.bg(PIXEL_COLORS[background]);
                        }
                        else { background = 0; }
                    }
                    for i in 0..8 {
                        if block[i] != 0 && block[i] != foreground && block[i] != background {
                            block[i] = foreground
                        }     
                    }
                }
                let span = Span::styled(block_to_char(block, foreground).to_string(), style);
                spans[y].push(span);
            }
        }

        let mut lines = vec![
            Line::from("| use ← and → to move | press ↑ to toggle pouring |").centered(),
            Line::from("| press enter to change color | press 'r' to reset | press 'q' to quit |").centered(),
            self.display_faucet()
            ];
        for spanline in spans {
            lines.push(Line::from(spanline));
        }

        return Text {
            alignment: Some(Alignment::Center),
            style: Style::new(),
            lines
        };
    }

    fn display_faucet(&self) -> Line {
        let mut string = String::new();
        for _ in 0..(self.faucet_pos/2) { string.push(' '); }
        if self.faucet_pos % 2 == 0 { string.push('▌'); }
        else { string.push('▐'); }
        return Line::from(string).style(PIXEL_COLORS[self.faucet_color]).left_aligned();
    }

    pub fn switch_color(&mut self) {
        self.faucet_color += 1;
        if self.faucet_color == 5 { self.faucet_color = 1; }
    }

    pub fn draw(&self, frame: &mut Frame) {
        frame.render_widget(self, frame.area());
    }
}

impl Widget for &App {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let title = Line::from(" sand:boxed ").centered();
        let block = Block::bordered()
            .border_type(BorderType::Rounded)
            .title(title);

        let sandbox_text = Text::from(self.sandbox_to_text());

        ratatui::widgets::

        Paragraph::new(sandbox_text)
            .centered()
            .block(block)
            .render(area, buf);
    }
}