# Rusty-Bar
## A terminal bar graph plotting library for Rust

### Goal
Implement a library for plotting static and dynamic graphs to the terminal. The plotter takes in data from various sources such as: Maps, array slices, vectors, files or data streams.

### Why build a plotting library?
Rust has are very little options for plotting data in the terminal. Given a strong focus on testing, it would be useful to run bennchmarks with visual cues for performance especially if the data source is dynamic.

### Difficulties
1. The biggest challenge will be the implementation of a dynamic plotter. Unlike with the static data source, we need to design an efficient way to read the source data and write it to the terminal.
1. The second challenge will be the dynamic terminal editing, especially the bookkeeping for moving the curson.

### Functional Requirements
1. Static Bar graph PLot
    1. Flags
    1. Labelling rows/columns
    1. Color-coding
    1. Custom icons
    1. Compound column data in one bar
1. Continuous graphing

### Reach features
1. Vertical graphing
1. Anchoring graph in the terminal
1. Displaying heatmaps

### API Sketch
```rust
pub trait DataStream: impl Iterator<utah::DataFrame> {}

pub trait Graph: {
       fn setOptions(impl GraphOptions)
	fn graph(data: impl DataFrame);
	fn dyn_graph(data: impl DataStream);
}
pub trait GraphOptions{}

pub struct BarGraph: impl Graph{}
pub struct HeatMap: impl Graph{}
```


### Use cases
1. Displaying static histograms
1. Displaying sound level bars
1. Displaying memory usage
1. Displaying resource usage