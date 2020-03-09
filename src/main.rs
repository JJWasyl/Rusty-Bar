use rusty_bar::graph_builder;
use std::io::{stdout, Write};
use std::{thread, time};
use futures::stream::Stream;
use futures::task::{Context, Poll};
use core::pin::Pin;
use rand::Rng;
use rand;

fn main() {
    let data = vec![vec![100, 250, 170], vec![200, 100, 150]];
    let labels: &[&str] = &[&"Station1", &"Station2"];
    let columns: &[&str] = &[&"CPU", &"GPU", &"MEMORY"];
    let rng = rand::thread_rng();
 
    let mut graph = graph_builder::GraphBuilder::new()
        .set_colors(&["red", "green", "cyan"])
        .load_2d_vec(data, labels, columns)
        .view_legend(true)
        .build();
    graph.print_static();
    let wait = time::Duration::from_millis(5000);
    
    loop {
        thread::sleep(wait);
        let new_data = generate_new_data(3, 2, 250, rng);
        graph.refresh_data(new_data);
    }
    /*
            self.reset_cursor(self.get_graph_height());
        self.clear_space();
    */
    

}


fn generate_new_data(x: usize, y: usize, max: u32, mut rng: rand::rngs::ThreadRng) -> Vec<Vec<u32>> {
    let mut new_data: Vec<Vec<u32>> = vec![vec![0;x];y];
    let max_val = max;
    for row in 0..new_data.len() {
        for col in 0..new_data[0].len() {
            new_data[row][col] = rng.gen_range(0u32, max_val);
        }
    }
    new_data
}


struct DataGenerator {
    rng: rand::rngs::ThreadRng,
    labels: usize,
    columns: usize,
    max: u32,
}

impl DataGenerator {
    pub fn new(y: usize, x: usize,  max_val: u32) -> Self {
        DataGenerator {
            rng: rand::thread_rng(),
            labels: y,
            columns: x,
            max: max_val,
        }
    }
}

impl Stream for DataGenerator {
    type Item = Vec<Vec<u32>>;

    fn poll_next(mut self: Pin<&mut Self>, cx: &mut Context<'_>) 
        -> Poll<Option<Self::Item>> {
        let mut new_data: Vec<Vec<u32>> = vec![vec![0;self.columns];self.labels];
        let max_val = self.max;
        for row in 0..new_data.len() {
            for col in 0..new_data[0].len() {
                new_data[row][col] = self.rng.gen_range(0u32, max_val);
            }
        }
        Poll::Ready(Some(new_data))
    }
}