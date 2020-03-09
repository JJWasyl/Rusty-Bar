use crossterm::{
    cursor, execute,
    style::{Color, Print, ResetColor, SetForegroundColor},
};
use std::collections::HashMap;
use std::io::{stdout, Write};

#[derive(Default, Debug, Clone)]
pub struct GraphBuilder<'a> {
    data: Vec<Vec<u32>>,
    labels: &'a [&'a str],
    col_labels: &'a [&'a str],
    show_col_labels: bool,
    show_legend: bool,
    color_palette: Vec<Color>,
    width: usize,
}

impl<'a> GraphBuilder<'a> {
    pub fn new() -> GraphBuilder<'a> {
        let builder = GraphBuilder::default();
        builder.set_width(GraphBuilder::max_width())
    }

    fn max_width()-> usize {
        return (crossterm::terminal::size().unwrap().0 as usize - 5)
    }

    pub fn build(self) -> StaticPlot<'a> {
        StaticPlot {
            data: self.data,
            labels: self.labels,
            col_labels: self.col_labels,
            width: self.width as u16,
            show_col_labels: self.show_col_labels,
            show_legend: self.show_legend,
            color_palette: self.color_palette.to_owned(),
        }
    }

    pub fn set_colors(mut self, colors: &'a [&'a str]) -> Self {
        let mut color_palette: Vec<Color> = vec![];
        for &c in colors {
            match c {
                "red" => color_palette.push(Color::Red),
                "green" => color_palette.push(Color::Green),
                "yellow" => color_palette.push(Color::Yellow),
                "blue" => color_palette.push(Color::Blue),
                "magenta" => color_palette.push(Color::Magenta),
                "cyan" => color_palette.push(Color::Cyan),
                "white" => color_palette.push(Color::White),
                _ => (),
            }
        }
        self.color_palette = color_palette;
        self
    }

    pub fn view_legend(mut self, visible: bool) -> Self {
        self.show_legend = visible;
        self
    }

    pub fn view_col_labels(mut self, visible: bool) -> Self {
        self.show_col_labels = visible;
        self
    }

    pub fn set_width(mut self, width: usize) -> Self {
        let termsize = crossterm::terminal::size().unwrap().0 as usize;
        if (width > termsize){
            self.width = termsize;
        }
        else {
            self.width = width;
        }
        self
    }

    pub fn load_hashmap(&'a mut self, map: HashMap<&'a str, &'a [u32]>) -> &'a mut Self {
        todo!();
    }

    pub fn load_2d_vec(
        mut self,
        vec: Vec<Vec<u32>>,
        labels: &'a [&'a str],
        col_labels: &'a [&'a str],
    ) -> Self {
        self.data = vec;
        self.labels = labels;
        self.col_labels = col_labels;
        self
    }

    //difficult part
    pub fn load_1d_array(&'a mut self, array: &'a [u32], label: &'a str) -> &'a mut Self {
        let mut new_data: Vec<&'a [u32]> = Vec::with_capacity(array.len());
        new_data.push(array);
        //self.data = new_data;
        self
    }
}

pub struct StaticPlot<'a> {
    data: Vec<Vec<u32>>,
    labels: &'a [&'a str],
    col_labels: &'a [&'a str],
    width: u16,
    show_col_labels: bool,
    show_legend: bool,
    color_palette: Vec<Color>,
}

impl<'a> StaticPlot<'a> {
    pub fn refresh_data(&mut self, new_data: Vec<Vec<u32>>) {
        self.data = new_data;
        self.reset_cursor(self.get_graph_height());
        self.clear_space();
        self.print_static();
    }

    pub fn print_static(&self) {
        let mut max_lab = 0;
        let mut max_col = 0;

        let mut cp_iter = self.color_palette.iter().cycle();
        self.labels
            .into_iter()
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

    fn get_graph_height(&self) -> u16 {
        let mut height = self.data[0].len() * self.data.len() + 1;
        if self.show_legend == true {
            height += 1;
        }
        height as u16
    }

    fn clear_space(&self) {
        let height = self.get_graph_height();
        for _ in 0..height {
            execute!(
                stdout(),
                Print(" ".repeat(self.width as usize)),
                Print("\n")
            ).unwrap();
        }
        self.reset_cursor(height as u16);
    }

    fn reset_cursor(&self, height: u16) {
        execute!(
            stdout(),
            cursor::MoveToPreviousLine(height),
        ).unwrap();
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

        let max_val = self.find_max_val();
        let draw_width = self.normalize_draw_space(max_label_len, max_col_len, max_val);

        for i in 0..self.data[row_idx].len() {
            execute!(stdout(), cursor::SavePosition,).unwrap();
            if self.show_col_labels {
                self.print_label(
                    &self.norm_label(self.col_labels[i], max_col_len),
                    Color::White,
                );
            }
            self.print_bar(self.data[row_idx][i], *cp_iter.next().unwrap(), draw_width, max_val);
            execute!(stdout(),
                    Print("\n"),
                    cursor::RestorePosition,
                    cursor::MoveDown(1)
            ).unwrap();
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

    fn print_bar(&self, size: u32, color: Color, draw_width: u32, max_val: u32) {
        let mut draw_size: u32 = size;
        if max_val > draw_width {
            draw_size = draw_size*draw_width / max_val;
        }
        execute!(
            stdout(),
            SetForegroundColor(color),
            Print(self.generate_bar(draw_size)),
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

    fn find_max_val(&self) -> u32 {
        let mut max = 0;
        for row in &self.data {
            for &val in row {
                if val > max {
                    max = val;
                }
            }
        }
        max
    }

    fn normalize_draw_space(&self, max_label_len: usize, max_col_len: usize, max_val: u32) -> u32 {
        let mut reserve_space: u32 = (max_label_len + 2) as u32;
        if self.show_col_labels {
            reserve_space += (max_col_len + 2) as u32;
        }
        self.width as u32 - reserve_space - (max_val as f64).log(10.) as u32 - 3
    }
}
