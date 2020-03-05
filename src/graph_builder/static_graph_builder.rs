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

    pub fn build() -> StaticPlot<'a> {
        todo!();
    }

    pub fn set_colors(&'a mut self, colors: &'a[&'a str]) -> &'a Self {
        for &c in colors {
            match c {
                "red" => self.color_palette.push(Color::Red),
                "green" => self.color_palette.push(Color::Green),
                "yellow" => self.color_palette.push(Color::Yellow),
                "blue" => self.color_palette.push(Color::Blue),
                "magenta" => self.color_palette.push(Color::Magenta),
                "cyan" => self.color_palette.push(Color::Cyan),
                "white" => self.color_palette.push(Color::White),
                _ => ()
            }
        }
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

    pub fn load_2d_array(&'a mut self, array: &'a[&'a[u32]]) -> &'a Self {
        self.data = array;
        self
    }

    pub fn load_1d_array(&'a mut self, array: &'a[u32]) -> &'a Self {
        todo!();
    }

    pub fn plot_static() -> &'a Self {
        todo!();
    }

    pub fn plot_dynamic() -> &'a Self {
        todo!();
    }
}
