use crossterm::style::Color;
pub use super::static_plot::StaticPlot;
use std::collections::HashMap;

#[derive(Default)]
pub struct StaticGraphBuilder<'a> {
    data: &'a[&'a[u32]],
    labels: &'a[&'a str],
    col_labels: &'a[&'a str],
    show_col_labels: bool,
    show_legend: bool,
    color_palette: Vec<Color>,
}

impl<'a> StaticGraphBuilder<'a> {
    pub fn new() -> Self {
        StaticGraphBuilder::default()
    }

    pub fn build(&'a self) -> StaticPlot<'a> {
        StaticPlot::new(
            self.data,
            self.labels,
            self.col_labels,
            false,
            false,
            vec![Color::White],
        )
    }

    pub fn set_colors(&'a mut self, colors: &'a[&'a str]) -> &'a Self {
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
                _ => ()
            }
        }
        self.color_palette = color_palette;
        self
    }

    pub fn show_legend(&'a mut self, visible: bool) ->  &'a Self {
        self.show_legend = visible;
        self
    }

    pub  fn show_col_labels(&'a mut self, visible: bool) ->  &'a Self {
        self.show_col_labels = visible;
        self
    }

    pub fn load_hashmap(&'a mut self, map: HashMap<&'a str, &'a [u32]>) -> &'a Self {
        todo!();
    }

    pub fn load_2d_array(&'a mut self, array: &'a[&'a[u32]], labels: &'a[&'a str], col_labels: &'a[&'a str]) -> &'a Self {
        self.data = array;
        self.labels = labels;
        self.col_labels = col_labels;
        self
    }

    //difficult part
    pub fn load_1d_array(&'a mut self, array: &'a [u32], label: &'a str) -> &'a Self {
        let mut new_data: Vec<&'a[u32]> = Vec::with_capacity(array.len());
        new_data.push(array);
        //self.data = new_data;
        self
    }

    pub fn plot_static(&'a self) -> &'a Self {
        
        self
    }
}
