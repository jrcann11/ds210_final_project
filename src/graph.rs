use petgraph::Graph;
use petgraph::graph::NodeIndex;
use crate::tv_show::TvShow;

pub fn build_graph(tv_shows: &Vec<TvShow>) -> Graph<String, bool, petgraph::Undirected> {
    let mut graph = Graph::<String, bool, petgraph::Undirected>::with_capacity(tv_shows.len(), tv_shows.len() * 2);

    // Create TV show nodes
    let mut node_indices = Vec::with_capacity(tv_shows.len());
    for tv_show in tv_shows.iter() {
        let node_index = graph.add_node(tv_show.title.clone());
        node_indices.push(node_index);
    }

    // Add platform edges to graph
    for (i, tv_show) in tv_shows.iter().enumerate() {
        if tv_show.netflix {
            graph.add_edge(node_indices[i], NodeIndex::new(0), true);
        }
        if tv_show.hulu {
            graph.add_edge(node_indices[i], NodeIndex::new(1), true);
        }
        if tv_show.prime_video {
            graph.add_edge(node_indices[i], NodeIndex::new(2), true);
        }
        if tv_show.disney {
            graph.add_edge(node_indices[i], NodeIndex::new(3), true);
        }
    }

    graph
}