use clap::ValueEnum;

#[derive(Clone, Debug, Default, ValueEnum)]

pub enum GraphvizEngine {
    /// filter for drawing directed graphs
    #[default]
    DOT,
    /// filter for drawing undirected graphs
    NEATO,
    /// filter for radial layouts of graphs
    TWOPI,
    /// filter for circular layout of graphs
    CIRCO,
    /// filter for drawing undirected graphs
    FDP,
    /// filter for drawing large undirected graphs
    SFDP,
    /// filter for squarified tree maps
    PATCHWORK,
    /// filter for array-based layouts
    OSAGE,
}
