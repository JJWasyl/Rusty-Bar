use crossterm::style::Color;

#[derive(Default)]
pub struct DynamicGraphBuilder<'a> {
    data: &'a[&'a[u32]],
    labels: &'a[&'a str],
    col_labels: &'a[&'a str],
    show_col_labels: bool,
    show_legend: bool,
    color_palette: Vec<Color>,
}

impl<'a> DynamicGraphBuilder<'a> {
    pub fn new() -> Self {
        DynamicGraphBuilder::default()
    }

    pub fn set_color() -> Self {
        todo!();
    }

    pub fn show_legend() {
        todo!();
    }

    pub  fn show_col_labels() {
        todo!();
    }

    pub fn load_hashmap() {
        todo!();
    }

    pub fn load_2d_array() {
        todo!();
    }

    pub fn load_1d_vector() {
        todo!();
    }

    pub fn plot_static() {
        todo!();
    }

    pub fn plot_dynamic() {
        todo!();
    }
}
