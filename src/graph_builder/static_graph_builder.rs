use crossterm::style::Color;
pub use super::static_plot::StaticPlot;

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

    pub fn set_color() -> &'a Self {
        todo!();
    }

    pub fn show_legend() ->  &'a Self {
        todo!();
    }

    pub  fn show_col_labels() ->  &'a Self {
        todo!();
    }

    pub fn load_hashmap() -> &'a Self {
        todo!();
    }

    pub fn load_2d_array() -> &'a Self {
        todo!();
    }

    pub fn load_1d_vector() -> &'a Self {
        todo!();
    }

    pub fn plot_static() -> &'a Self {
        todo!();
    }

    pub fn plot_dynamic() -> &'a Self {
        todo!();
    }
}
