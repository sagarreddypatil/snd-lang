use std::fmt::{self, Display, Formatter};

#[derive(Debug)]
pub struct Context {
    pub start: usize,
    pub len: usize,

    pub path: &'static str,
    pub src: &'static str,
}

impl Context {
    fn get_line_info(&self) -> (usize, usize) {
        let s = self.src;
        let index = self.start;

        let mut line_number = 1;
        let mut line_start = 0;

        for (i, c) in s.char_indices() {
            if i >= index {
                break;
            }
            if c == '\n' {
                line_number += 1;
                line_start = i + 1;
            }
        }

        let position_in_line = index - line_start + 1;
        (line_number, position_in_line)
    }
}

impl Display for Context {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        let (line, col) = self.get_line_info();
        write!(f, "{}:{}:{}", self.path, line, col)
    }
}

impl Context {
    pub fn in_context(&self) -> String {
        let (line, col) = self.get_line_info();
        let line_src = self.src.lines().nth(line - 1).unwrap();

        format!(
            "{}:{}:{}\n{}\n{}{}",
            self.path,
            line,
            col,
            line_src,
            " ".repeat(col - 1),
            "^".repeat(self.len)
        )
    }
}
