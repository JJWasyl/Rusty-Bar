use crossterm::{
    execute,
    style::{Color, Print, ResetColor, SetBackgroundColor, SetForegroundColor},
    ExecutableCommand, Result,
    cursor,
};
use std::io::{stdout, Write};

pub fn print_static(data: &[&[u32]], rows: &[&str], columns: &[&str]) {
    let mut max_lab = 0;
    let mut max_col = 0;
    rows.into_iter().for_each(|s| max_lab = if s.len() > max_lab {s.len()} else {max_lab});
    columns.into_iter().for_each(|s| max_col = if s.len() > max_col {s.len()} else {max_col});
    for (row_idx, row) in data.iter().enumerate() {
        print_label_group(rows[row_idx], max_lab, row, columns, max_col);
    }
}

fn print_label_group(label: &str, max_label_len: usize, columns: &[u32], column_labels: &[&str], max_col_len: usize) {
    print_label(&(label.to_owned() + &(" ".repeat(max_label_len - label.len()))), Color::White);
    for i in 0..columns.len() {
        execute!(
            stdout(),
            cursor::SavePosition,
        ).unwrap();
        print_label(&(column_labels[i].to_owned() + &(" ".repeat(max_col_len - column_labels[i].len()))), Color::White);
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
        Print(": "),
        ResetColor,
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
        result.push_str(&"▄");
    };
    result
}
