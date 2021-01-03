use ansi_term::Colour::*;
use ansi_term::Style;

pub fn blue(s: &dyn ToString) -> String {
    //Style::new().fg(Blue).paint(s.to_string()).to_string()
    s.to_string()
}

pub fn green(s: &dyn ToString) -> String {
    //Style::new().fg(Green).paint(s.to_string()).to_string()
    s.to_string()
}

pub fn cyan(s: &dyn ToString) -> String {
    //Style::new().fg(Cyan).paint(s.to_string()).to_string()
    s.to_string()
}
