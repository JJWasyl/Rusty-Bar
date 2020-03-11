//! An implementation of a Graphing library with a plot object and
//! a client facing builder.

use crossterm::{
    cursor, execute,
    style::{Color, Print, ResetColor, SetForegroundColor},
};
//use std::collections::HashMap;
use std::fmt;
use std::io::{stdout, Write};

/// A graphbuilding struct holding data and plotting
/// hyperparameters.
#[derive(Default, Debug, Clone)]
pub struct GraphBuilder<'a> {
    data: Vec<Vec<u32>>,
    labels: &'a [&'a str],
    col_labels: &'a [&'a str],
    show_col_labels: bool,
    show_legend: bool,
    color_palette: Vec<Color>,
    width: usize,
    val_cap: Option<u32>,
}

impl<'a> GraphBuilder<'a> {
    /// Creates a new graphbuilder object with default width set
    /// to the edge of the current terminal.
    ///
    /// ## Example
    /// ```
    /// use rusty_bar::graph_builder;
    /// let mut graph_builder = GraphBuilder::new();
    /// ```
    pub fn new() -> GraphBuilder<'a> {
        let builder = GraphBuilder::default();
        builder.set_width(GraphBuilder::max_width())
    }

    fn max_width() -> usize {
        return crossterm::terminal::size().unwrap().0 as usize - 5;
    }

    /// Builds the parametrized graph, any unspecified hyperparameters
    /// are set to their default values. Returns a StaticPlot struct.
    ///
    /// ## Example
    /// ```
    /// use  rusty_bar::graph_builder;
    /// let mut graph_builder = GraphBuilder::new();
    /// let plot = graph_builder.build();
    /// ```
    pub fn build(self) -> StaticPlot<'a> {
        StaticPlot {
            data: self.data,
            labels: self.labels,
            col_labels: self.col_labels,
            width: self.width as u16,
            show_col_labels: self.show_col_labels,
            show_legend: self.show_legend,
            color_palette: self.color_palette.to_owned(),
            val_cap: self.val_cap,
        }
    }

    /// Sets the color palette of the graph.
    ///
    /// ## Arguments
    /// 'colors' -> array of string slices with color names in lowercase
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

    /// Sets the maximum value cap for the graph
    pub fn set_max_val(mut self, val: u32) -> Self {
        self.val_cap = Some(val);
        self
    }

    /// Sets the visibility of the color palette legend
    pub fn view_legend(mut self, visible: bool) -> Self {
        self.show_legend = visible;
        self
    }

    /// Sets the visibility of column labels before each individual column
    pub fn view_col_labels(mut self, visible: bool) -> Self {
        self.show_col_labels = visible;
        self
    }

    /// Overrides the default width of the graph. The cap is set to the maximum
    /// width of the terminal window.
    pub fn set_width(mut self, width: usize) -> Self {
        let termsize = crossterm::terminal::size().unwrap().0 as usize;
        if width > termsize {
            self.width = termsize;
        } else {
            self.width = width;
        }
        self
    }

    /*
    pub fn load_hashmap(&'a mut self, map: HashMap<&'a str, &'a [u32]>) -> &'a mut Self {
        todo!();
    }
    */

    /// Loads data from a 2D vector of u32's.
    pub fn load_2d_vec(mut self, vec: Vec<Vec<u32>>) -> Self {
        self.data = vec;
        self
    }

    /// Loads the row labels of the graph.
    pub fn load_labels(mut self, labels: &'a [&'a str]) -> Self {
        self.labels = labels;
        self
    }

    /// Loads the column labels of the graph.
    pub fn load_col_labels(mut self, col_labels: &'a [&'a str]) -> Self {
        self.col_labels = col_labels;
        self
    }

    /// Loads data from an array of u32's
    pub fn load_1d_array(mut self, array: &'a [u32]) -> Self {
        self.data = vec![array.to_vec()];
        self
    }
}

/// Main Plot struct holding values and hyperparameters. It should be
/// created using a parent builder.
pub struct StaticPlot<'a> {
    data: Vec<Vec<u32>>,
    labels: &'a [&'a str],
    col_labels: &'a [&'a str],
    width: u16,
    show_col_labels: bool,
    show_legend: bool,
    color_palette: Vec<Color>,
    val_cap: Option<u32>,
}

impl<'a> StaticPlot<'a> {
    /// Loads a new 2D vector for the graph, resets the graphing area and plots the new
    /// graph to the designated output stream.
    ///
    /// ## Example
    /// ```
    /// use rusty_bar::graph_builder;
    /// let data = vec![vec![100, 250, 170], vec![200, 100, 150]];
    /// let labels: &[&str] = &[&"Station1", &"Station2"];
    /// let columns: &[&str] = &[&"CPU", &"GPU", &"MEMORY"];
    ///
    /// let mut graph = graph_builder::GraphBuilder::new()
    ///     .set_colors(&["red", "green", "cyan"])
    ///     .load_2d_vec(data)
    ///     .load_labels(labels)
    ///     .load_col_labels(columns)
    ///     .view_legend(true)
    ///     .build();
    /// graph.refresh_2d_data(vec![vec![100, 200, 300], vec![300, 200, 100]])
    /// ```
    pub fn refresh_2d_data(&mut self, new_data: Vec<Vec<u32>>, output: impl Write) {
        self.data = new_data;
        self.reset_cursor(self.get_graph_height());
        self.print_static(output);
    }

    /// Loads a new 1D vector for the graph, resets the graphing area and plots the new
    /// graph to thhe designated output stream.
    /// See `refresh_2d_data()` for examples.
    pub fn refresh_1d_data(&mut self, new_data: Vec<u32>, output: impl Write) {
        self.data = vec![new_data];
        self.reset_cursor(self.get_graph_height());
        self.print_static(output);
    }

    /// Prints a static representation of the current StaticPlot struct
    /// to the designated output stream.
    ///
    /// ## Example
    /// ```
    /// use rusty_bar::graph_builder;
    /// let data = vec![vec![100, 250, 170], vec![200, 100, 150]];
    /// let labels: &[&str] = &[&"Station1", &"Station2"];
    /// let columns: &[&str] = &[&"CPU", &"GPU", &"MEMORY"];
    ///
    /// let mut graph = graph_builder::GraphBuilder::new()
    ///     .set_colors(&["red", "green", "cyan"])
    ///     .load_2d_vec(data)
    ///     .load_labels(labels)
    ///     .load_col_labels(columns)
    ///     .view_legend(true)
    ///     .build();
    /// graph.print_static(std::io::stdout());
    /// ```
    pub fn print_static(&self, mut output: impl Write) {
        self.clear_space();
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
            execute!(output, Print(" ".repeat(offset)),).unwrap();
            for col_label in self.col_labels {
                execute!(
                    output,
                    Print(String::from(*col_label) + &" "),
                    SetForegroundColor(*cp_iter.next().unwrap()),
                    Print(self.generate_bar(1) + &" ".repeat(4)),
                    ResetColor
                )
                .unwrap()
            }
            execute!(output, Print("\n"),).unwrap();
        }
        for (row_idx, _) in self.data.iter().enumerate() {
            self.print_label_group(
                self.labels.get(row_idx).unwrap_or(&""),
                row_idx,
                max_lab,
                max_col,
                output.by_ref(),
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

    /// Overwrites designated graphing area with whitespaces
    fn clear_space(&self) {
        let height = self.get_graph_height();
        for _ in 0..height {
            execute!(
                stdout(),
                Print(" ".repeat(self.width as usize)),
                Print("\n")
            )
            .unwrap();
        }
        self.reset_cursor(height as u16);
    }

    fn reset_cursor(&self, height: u16) {
        execute!(stdout(), cursor::MoveToPreviousLine(height),).unwrap();
    }

    /// Prints an individual row of data and column labels
    fn print_label_group(
        &self,
        label: &str,
        row_idx: usize,
        max_label_len: usize,
        max_col_len: usize,
        mut output: impl Write,
    ) {
        if !label.is_empty() {
            self.print_label(
                &self.norm_label(label, max_label_len),
                Color::White,
                output.by_ref(),
            );
        }
        let mut cp_iter = self.color_palette.iter().cycle();

        let max_val = self.find_max_val();
        let draw_width = self.normalize_draw_space(max_label_len, max_col_len, max_val);

        for i in 0..self.data[row_idx].len() {
            execute!(output, cursor::SavePosition,).unwrap();
            if self.show_col_labels {
                self.print_label(
                    &self.norm_label(self.col_labels[i], max_col_len),
                    Color::White,
                    output.by_ref(),
                );
            }
            self.print_bar(
                self.data[row_idx][i],
                *cp_iter.next().unwrap(),
                draw_width,
                max_val,
                output.by_ref(),
            );
            execute!(
                output,
                Print("\n"),
                cursor::RestorePosition,
                cursor::MoveDown(1)
            )
            .unwrap();
        }
        execute!(stdout(), cursor::MoveToNextLine(0)).unwrap();
    }

    /// Padding for label offset
    fn norm_label(&self, label: &str, max_len: usize) -> String {
        label.to_owned() + &(" ".repeat(max_len - label.len()))
    }

    fn print_label(&self, label: &str, color: Color, mut output: impl Write) {
        execute!(
            output,
            SetForegroundColor(color),
            Print(label),
            Print(": "),
            ResetColor,
        )
        .unwrap();
    }

    /// Prints a graph bar to the designated draw width
    fn print_bar(
        &self,
        size: u32,
        color: Color,
        draw_width: u32,
        max_val: u32,
        mut output: impl Write,
    ) {
        let mut draw_size: u32 = size;
        if size > max_val {
            draw_size = max_val;
        }
        if max_val > draw_width {
            draw_size = draw_size * draw_width / max_val;
        }
        execute!(
            output,
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
        match self.val_cap {
            Some(val) => val,
            None => {
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
        }
    }

    /// Offsets and pads the drawing space  based on presence of labels and magnitude of data
    /// values.
    fn normalize_draw_space(&self, max_label_len: usize, max_col_len: usize, max_val: u32) -> u32 {
        let mut reserve_space: u32 = (max_label_len + 2) as u32;
        if self.show_col_labels {
            reserve_space += (max_col_len + 2) as u32;
        }
        let number_length = (max_val as f64).log(10.) as u32;
        self.width as u32 - reserve_space - number_length - 3
    }
}
