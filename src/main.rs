use crossterm::{
    execute,
    style::{Color, Print, ResetColor, SetBackgroundColor, SetForegroundColor},
    ExecutableCommand, Result,
    cursor,
};
use std::io::{stdout, Write};

fn main() {
    print_label_group(&"test", &[20, 10, 15, 2]);
    print_label_group(&"test", &[20, 10, 15, 2]);
}

fn print_label_group(label: &str, columns: &[u32]) {
    print_label(label, Color::White);
    for i in 0..columns.len() {
        execute!(
            stdout(),
            cursor::SavePosition,
        ).unwrap();
        print_bar(columns[i], Color::White);
        execute!(
            stdout(),
            cursor::RestorePosition,
            cursor::MoveDown(1),
        ).unwrap();
    };
    execute!(stdout(), cursor::MoveToNextLine(0)).unwrap();
}

fn print_label(label: &str, color: Color) {
    execute!(
        stdout(),
        SetForegroundColor(color),
        Print(label),
        ResetColor,
        Print(": "),
    ).unwrap();
}

fn print_bar(size: u32, color: Color) {
    execute!(
        stdout(),
        SetForegroundColor(color),
        Print(generate_bar(size)),
        Print(format!(" {}\n", size)),
        ResetColor
    ).unwrap();
}

fn generate_bar(amount: u32) -> String {
    let mut result = String::from("");
    for _ in 0..amount {
        result.push_str(&"▇");
    };
    result
}
