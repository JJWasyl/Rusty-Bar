use crossterm::{
    cursor, execute,
    style::{Color, Print, ResetColor, SetForegroundColor}
};
use std::io::{stdout, Write};

#[derive(Default)]
pub struct StaticPlot<'a> {
    data: &'a[&'a[u32]],
    labels: &'a[&'a str],
    col_labels: &'a[&'a str],
    show_col_labels: bool,
    show_legend: bool,
    color_palette: Vec<Color>,
}

impl<'a> StaticPlot<'a> {   
    pub fn new( data: &'a[&'a[u32]],
                labels: &'a[&'a str],
                col_labels: &'a[&'a str],
                show_col_labels: bool,
                show_legend: bool,
                color_palette: Vec<Color>,) -> Self {
        StaticPlot{
            data: data,
            labels: labels,
            col_labels: col_labels,
            color_palette: color_palette,

            show_col_labels: show_col_labels,
            show_legend: show_legend
        }
    }

    pub fn refresh_data(&mut self, new_data: &'a[&'a[u32]]) {
        self.data = new_data;
        self.print_static();
    }

    pub fn print_static(&self) {
        let mut max_lab = 0;
        let mut max_col = 0;

        let mut cp_iter = self.color_palette.iter().cycle();
        self.labels.into_iter()
            .for_each(|s| max_lab = if s.len() > max_lab { s.len() } else { max_lab });
        self.col_labels
            .into_iter()
            .for_each(|s| max_col = if s.len() > max_col { s.len() } else { max_col });
        if self.show_legend {
            let mut offset = max_lab + 2;
            if self.show_col_labels {
                offset += max_col + 2;
            }
            execute!(stdout(), Print(" ".repeat(offset)),).unwrap();
            for col_label in self.col_labels {
                execute!(
                    stdout(),
                    Print(String::from(*col_label) + &" "),
                    SetForegroundColor(*cp_iter.next().unwrap()),
                    Print(self.generate_bar(1) + &" ".repeat(4)),
                    ResetColor
                )
                .unwrap()
            }
            execute!(stdout(), Print("\n"),).unwrap();
        }
        for (row_idx, _) in self.data.iter().enumerate() {
            self.print_label_group(
                self.labels.get(row_idx).unwrap_or(&""),
                row_idx,
                max_lab,
                max_col,
            );
        }
    }
    
    fn print_label_group(
        &self,
        label: &str,
        row_idx: usize,
        max_label_len: usize,
        max_col_len: usize,
    ) {
        if !label.is_empty() {
            self.print_label(&self.norm_label(label, max_label_len), Color::White);
        }
        let mut cp_iter = self.color_palette.iter().cycle();
        for i in 0..self.data[row_idx].len() {
            execute!(stdout(), cursor::SavePosition,).unwrap();
            if self.show_col_labels {
                self.print_label(&self.norm_label(self.col_labels[i], max_col_len), Color::White);
            }
            self.print_bar(self.data[row_idx][i], *cp_iter.next().unwrap());
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
    
    fn norm_label(&self, label: &str, max_len: usize) -> String {
        label.to_owned() + &(" ".repeat(max_len - label.len()))
    }
    
    fn print_label(&self, label: &str, color: Color) {
        execute!(
            stdout(),
            SetForegroundColor(color),
            Print(label),
            Print(": "),
            ResetColor,
        )
        .unwrap();
    }
    
    fn print_bar(&self, size: u32, color: Color) {
        execute!(
            stdout(),
            SetForegroundColor(color),
            Print(self.generate_bar(size)),
            Print(format!(" {}", size)),
            ResetColor
        )
        .unwrap();
    }
    
    fn generate_bar(&self, amount: u32) -> String {
        let mut result = String::from("");
        for _ in 0..amount {
            result.push_str(&"▄");
        }
        result
    }
    
}