use std::cell::RefCell;
use std::rc::Rc;

use crate::page::Symbol;

#[derive(Clone)]
pub struct LayoutContext {
    buffer: Rc<RefCell<Vec<Vec<Symbol>>>>,
    view_size: (usize, usize),
    glimpse_origin: (usize, usize),
    glimpse_size: (usize, usize),
    fg_color: (f32, f32, f32),
    h1_color: (f32, f32, f32),
    h2_color: (f32, f32, f32),
    h3_color: (f32, f32, f32),
    h4_color: (f32, f32, f32),
}

impl LayoutContext {
    pub fn new(buffer: Rc<RefCell<Vec<Vec<Symbol>>>>, view_size: (usize, usize), glimpse_origin: (usize, usize), glimpse_size: (usize, usize), fg_color: (f32, f32, f32), h1_color: (f32, f32, f32), h2_color: (f32, f32, f32), h3_color: (f32, f32, f32), h4_color: (f32, f32, f32)) -> Self {
        Self { buffer, view_size, glimpse_origin, glimpse_size, fg_color, h1_color, h2_color, h3_color, h4_color }
    }

    pub fn view_size(&self) -> (usize, usize){
        return self.glimpse_size;
    }
}

impl LayoutContext {
    pub fn frame(&mut self, inner: fn(LayoutContext)) {
        if self.glimpse_size.0 < 2 || self.glimpse_size.1 < 2 { return; }

        {
            let mut buf = self.buffer.borrow_mut();

            buf[self.glimpse_origin.1][self.glimpse_origin.0] = self.c('+');
            buf[self.glimpse_origin.1 + self.glimpse_size.1 - 1][self.glimpse_origin.0] = self.c('+');
            buf[self.glimpse_origin.1 + self.glimpse_size.1 - 1][self.glimpse_origin.0 + self.glimpse_size.0 - 1] = self.c('+');
            buf[self.glimpse_origin.1][self.glimpse_origin.0 + self.glimpse_size.0 - 1] = self.c('+');

            for y in (self.glimpse_origin.1 + 1)..(self.glimpse_origin.1 + self.glimpse_size.1 - 1) {
                buf[y][self.glimpse_origin.0] = self.c('|');
                buf[y][self.glimpse_origin.0 + self.glimpse_size.0 - 1] = self.c('|');
            }

            for x in (self.glimpse_origin.0 + 1)..(self.glimpse_origin.0 + self.glimpse_size.0 - 1) {
                buf[self.glimpse_origin.1][x] = self.c('-');
                buf[self.glimpse_origin.1 + self.glimpse_size.1 - 1][x] = self.c('-');
            }
        }

        self.padding(1, 1, 1, 1, inner);
    }

    pub fn vsplit(&mut self, split: isize, up: fn(LayoutContext), down: fn(LayoutContext)) {
        let split_loc = if split >= 0 {
            split
        } else {
            self.glimpse_size.1 as isize - split.abs()
        };

        if split_loc < 0 {
            down(self.clone());
        } else if split_loc as usize > self.glimpse_size.1 {
            up(self.clone());
        } else {
            let split_loc = split_loc as usize;

            for x in self.glimpse_origin.0..(self.glimpse_origin.0 + self.glimpse_size.0) {
                self.buffer.borrow_mut()[self.glimpse_origin.1 + split_loc][x] = self.c('-');
            }

            if split_loc > 0 {
                let mut ctx = self.clone();
                ctx.glimpse_size = (self.glimpse_size.0, split_loc);
                up(ctx);
            }

            if split_loc < self.glimpse_size.1 - 1 {
                let mut ctx = self.clone();
                ctx.glimpse_origin = (self.glimpse_origin.0, self.glimpse_origin.1 + split_loc + 1);
                ctx.glimpse_size = (self.glimpse_size.0, self.glimpse_size.1 - split_loc - 1);
                down(ctx);
            }
        }
    }

    pub fn hsplit(&mut self, split: isize, left: fn(LayoutContext), right: fn(LayoutContext)) {
        let split_loc = if split >= 0 {
            split
        } else {
            self.glimpse_size.0 as isize - split.abs()
        };

        if split_loc < 0 {
            right(self.clone());
        } else if split_loc as usize > self.glimpse_size.0 {
            left(self.clone());
        } else {
            let split_loc = split_loc as usize;

            for y in self.glimpse_origin.1..(self.glimpse_origin.1 + self.glimpse_size.1) {
                self.buffer.borrow_mut()[y][self.glimpse_origin.0 + split_loc] = self.c('|');
            }

            if split_loc > 0 {
                let mut ctx = self.clone();
                ctx.glimpse_size = (split_loc, self.glimpse_size.1);
                left(ctx);
            }

            if split_loc < self.glimpse_size.0 - 1 {
                let mut ctx = self.clone();
                ctx.glimpse_origin = (self.glimpse_origin.0 + split_loc + 1, self.glimpse_origin.1);
                ctx.glimpse_size = (self.glimpse_size.0 - split_loc - 1, self.glimpse_size.1);
                right(ctx);
            }
        }
    }

    pub fn padding(&mut self, left: usize, right: usize, up: usize, down: usize, inner: fn(LayoutContext)) {
        if self.glimpse_size.0 < left + right + 1 { return; }
        if self.glimpse_size.1 < up + down + 1 { return; }

        let mut ctx = self.clone();
        ctx.glimpse_origin = (self.glimpse_origin.0 + left, self.glimpse_origin.1 + up);
        ctx.glimpse_size = (self.glimpse_size.0 - left - right, self.glimpse_size.1 - up - down);
        inner(ctx);
    }

    pub fn text(&mut self, text: &str) {
        let mut x = 0;
        let mut y = 0;

        for c in text.chars() {
            match c {
                '\n' => {
                    x = 0;
                    y += 1;
                }
                c => {
                    self.buffer.borrow_mut()[self.glimpse_origin.1 + y][self.glimpse_origin.0 + x] = self.c(c);
                    x += 1;
                }
            }

            if x >= self.glimpse_size.0 {
                x = 0;
                y += 1;
            }
        }

        if x == 0 && y > 0 { y -= 1; }
        self.glimpse_origin.1 += y + 1;
        self.glimpse_size.1 = (self.glimpse_size.1 as isize - y as isize - 1).max(0) as usize;
    }

    pub fn ftext(&mut self, text: &str) {
        let chars: Vec<char> = text.chars().collect();

        let mut x = 0;
        let mut y = 0;
        let mut i = 0;

        let mut color = self.fg_color;
        let mut bold = false;
        let mut italic = false;

        while i < chars.len() {
            match chars[i] {
                '\n' => {
                    x = 0;
                    y += 1;
                }
                '<' => {
                    if i + 4 < chars.len() {
                        match chars[i..i+4].iter().collect::<String>().as_str() {
                            "<fg>" => {
                                color = self.fg_color;
                                bold = false;
                                italic = false;
                                i += 3;
                            },
                            "<h1>" => {
                                color = self.h1_color;
                                i += 3;
                            },
                            "<h2>" => {
                                color = self.h2_color;
                                i += 3;
                            },
                            "<h3>" => {
                                color = self.h3_color;
                                i += 3;
                            },
                            "<h4>" => {
                                color = self.h4_color;
                                i += 3;
                            }
                            "<bo>" => {
                                bold = true;
                                i += 3;
                            }
                            "<it>" => {
                                italic = true;
                                i += 3;
                            }
                            _ => {
                                self.buffer.borrow_mut()[self.glimpse_origin.1 + y][self.glimpse_origin.0 + x] = Symbol {
                                    character: '<',
                                    color,
                                    bold,
                                    italic,
                                };
                                x += 1;
                            }
                        }
                    } else {
                        self.buffer.borrow_mut()[self.glimpse_origin.1 + y][self.glimpse_origin.0 + x] = Symbol {
                            character: '<',
                            color,
                            bold,
                            italic,
                        };
                    }
                }
                c => {
                    self.buffer.borrow_mut()[self.glimpse_origin.1 + y][self.glimpse_origin.0 + x] = Symbol {
                        character: c,
                        color,
                        bold,
                        italic,
                    };
                    x += 1;
                }
            }

            if x >= self.glimpse_size.0 {
                x = 0;
                y += 1;
            }

            i += 1;
        }

        if x == 0 && y > 0 { y -= 1; }
        self.glimpse_origin.1 += y + 1;
        self.glimpse_size.1 = (self.glimpse_size.1 as isize - y as isize - 1).max(0) as usize;
    }

    pub fn img(&mut self, img: &Vec<f32>, w: usize, h: usize) {
        for y in 0..h {
            for x in 0..w {
                if x < self.glimpse_size.0 && y < self.glimpse_size.1 {
                    let pix = (y * w + x) * 3;

                    let mut ch = ' ';
                    let brightness = 0.299 * img[pix + 0] + 0.587 * img[pix + 1] + 0.114 * img[pix + 2];
                    if brightness < 0.2 {
                        ch = '.';
                    } else if 0.2 <= brightness && brightness < 0.4 {
                        ch = ':';
                    } else if 0.4 <= brightness && brightness < 0.6 {
                        ch = 'o';
                    } else if 0.6 <= brightness && brightness < 0.8 {
                        ch = '0';
                    } else if 0.8 <= brightness  {
                        ch = '@';
                    }

                    self.buffer.borrow_mut()[self.glimpse_origin.1 + y][self.glimpse_origin.0 + x] = Symbol {
                        character: ch,
                        color: (img[pix + 0], img[pix + 1], img[pix + 2]),
                        bold: false,
                        italic: false,
                    };
                }
            }
        }
    }
}

impl LayoutContext {
    fn c(&self, c: char) -> Symbol {
        Symbol {
            character: c,
            color: self.fg_color,
            bold: false,
            italic: false,
        }
    }
}