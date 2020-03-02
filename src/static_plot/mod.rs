use crossterm::{
    cursor, execute,
    style::{Color, Print, ResetColor, SetBackgroundColor, SetForegroundColor},
    ExecutableCommand, Result,
};
use std::io::{stdout, Write};

pub fn print_static(
    data: &[&[u32]],
    rows: &[&str],
    columns: &[&str],
    show_column_labels: bool,
    show_legend: bool,
) {
    let mut max_lab = 0;
    let mut max_col = 0;
    // cp = color palette, mocked for now
    let cp: Vec<Color> = vec![Color::Red, Color::Cyan, Color::White, Color::Green];
    let mut cp_iter = cp.iter();
    rows.into_iter()
        .for_each(|s| max_lab = if s.len() > max_lab { s.len() } else { max_lab });
    columns
        .into_iter()
        .for_each(|s| max_col = if s.len() > max_col { s.len() } else { max_col });
    if show_legend {
        let mut offset = max_lab + 2;
        if show_column_labels {
            offset += max_col + 2;
        }
        execute!(stdout(), Print(" ".repeat(offset)),).unwrap();
        for col_label in columns {
            let color = match cp_iter.next() {
                Some(col) => col,
                None => {
                    cp_iter = cp.iter();
                    cp_iter.next().unwrap()
                }
            };
            execute!(
                stdout(),
                Print(String::from(*col_label) + &" "),
                SetForegroundColor(*color),
                Print(generate_bar(1) + &" ".repeat(4)),
                ResetColor
            )
            .unwrap()
        }
        execute!(stdout(), Print("\n"),).unwrap();
    }
    for (row_idx, row) in data.iter().enumerate() {
        print_label_group(
            rows[row_idx],
            max_lab,
            row,
            columns,
            max_col,
            show_column_labels,
        );
    }
}

fn print_label_group(
    label: &str,
    max_label_len: usize,
    columns: &[u32],
    column_labels: &[&str],
    max_col_len: usize,
    show_column_labels: bool,
) {
    // cp = color palette, mocked for now
    let cp: Vec<Color> = vec![Color::Red, Color::Cyan, Color::White, Color::Green];
    print_label(&norm_label(label, max_label_len), Color::White);
    let mut cp_iter = cp.iter();
    for i in 0..columns.len() {
        execute!(stdout(), cursor::SavePosition,).unwrap();
        if show_column_labels {
            print_label(&norm_label(column_labels[i], max_col_len), Color::White);
        }
        let color = match cp_iter.next() {
            Some(col) => col,
            None => {
                cp_iter = cp.iter();
                cp_iter.next().unwrap()
            }
        };
        print_bar(columns[i], *color);
        execute!(
            stdout(),
            Print("\n"),
            cursor::RestorePosition,
            cursor::MoveDown(1),
        )
        .unwrap();
    }
    execute!(stdout(), cursor::MoveToNextLine(0)).unwrap();
}

fn norm_label(label: &str, max_len: usize) -> String {
    label.to_owned() + &(" ".repeat(max_len - label.len()))
}

fn print_label(label: &str, color: Color) {
    execute!(
        stdout(),
        SetForegroundColor(color),
        Print(label),
        Print(": "),
        ResetColor,
    )
    .unwrap();
}

fn print_bar(size: u32, color: Color) {
    execute!(
        stdout(),
        SetForegroundColor(color),
        Print(generate_bar(size)),
        Print(format!(" {}", size)),
        ResetColor
    )
    .unwrap();
}

fn generate_bar(amount: u32) -> String {
    let mut result = String::from("");
    for _ in 0..amount {
        result.push_str(&"▄");
    }
    result
}
