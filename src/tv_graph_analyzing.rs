use petgraph::Graph;
use petgraph::graph::NodeIndex;
use std::collections::HashMap;
use crate::tv_show::TvShow;

// This function calculates and prints the degree centralities for the platform nodes
pub fn calculate_degree_centrality(graph: &Graph<String, bool, petgraph::Undirected>) -> Vec<usize> {
    let mut degrees = vec![0; 4];
    for i in 0..4 {
        let platform_node = NodeIndex::new(i);
        for neighbor in graph.neighbors(platform_node) {
            degrees[i] += graph.edges_connecting(platform_node, neighbor).count();
        }
    }

    println!("Centrality values for the different streaming platforms:");
    println!("Netflix degree centrality: {}", degrees[0]);
    println!("Hulu degree centrality: {}", degrees[1]);
    println!("Prime Video degree centrality: {}", degrees[2]);
    println!("Disney+ degree centrality: {}", degrees[3]);
    degrees
}

// This function calculates and prints the degree distributions for the TV show nodes
pub fn calculate_degree_distribution(graph: &Graph<String, bool, petgraph::Undirected>, tv_shows: &Vec<TvShow>) -> Vec<usize> {
    let mut degree_counts = vec![0; tv_shows.len()];
    let node_indices = graph.node_indices().collect::<Vec<_>>();

    for (_i, node_index) in node_indices.iter().enumerate() {
        if *node_index != NodeIndex::new(0) && *node_index != NodeIndex::new(1) && *node_index != NodeIndex::new(2) && *node_index != NodeIndex::new(3) {
            let degree = graph.neighbors(*node_index).count();
            degree_counts[degree] += 1;
        }
    }

    println!("\nDegree distribution for TV show nodes:");

    for (degree, count) in degree_counts.iter().enumerate() {
        if *count > 0 {
            println!("Degree {}: {}", degree, count);
        }
    }
    degree_counts
}

// This function finds and prints the amount of exclusive content each platform has
pub fn calculate_exclusive_content(graph: &Graph<String, bool, petgraph::Undirected>, tv_shows: &Vec<TvShow>) -> Vec<u32> {
    let total_shows = tv_shows.len() as f64;
    let mut exclusive_counts = vec![0; 4];

    for i in 0..4 {
        let platform_node = NodeIndex::new(i);
        let mut exclusive_shows = 0;
        for neighbor in graph.neighbors(platform_node) {
            if neighbor.index() >= 4 { // only count TV show nodes, not platform nodes
                if graph.edges_connecting(platform_node, neighbor).count() == 1 { // the TV show is only connected to this platform node
                    exclusive_shows += 1;
                }
            }
        }
        let exclusive_percent = (exclusive_shows as f64 / total_shows) * 100.0;
        exclusive_counts[i] = exclusive_percent as u32;
    }

    println!("\nPercentage of TV shows exclusive to each platform:");
    println!("Netflix: {}%", exclusive_counts[0]);
    println!("Hulu: {}%", exclusive_counts[1]);
    println!("Prime Video: {}%", exclusive_counts[2]);
    println!("Disney+: {}%", exclusive_counts[3]);
    exclusive_counts
}

// This function calculate the amount of overlap in content (tv shows) between platform nodes
pub fn calculate_platform_overlap(graph: &Graph<String, bool, petgraph::Undirected>) -> Vec<Vec<usize>> {
    let mut overlap_counts = vec![vec![0; 4]; 4];
    for i in 0..4 {
        let node_i = NodeIndex::new(i);
        for j in (i+1)..4 {
            let node_j = NodeIndex::new(j);
            let common_neighbors = graph.neighbors(node_i)
                .filter(|n| graph.neighbors(node_j).any(|m| n == &m))
                .count();
            overlap_counts[i][j] = common_neighbors;
            overlap_counts[j][i] = common_neighbors;
        }
    }

    println!("\nOverlap counts between streaming platforms:");
    println!("Netflix-Hulu overlap count: {}", overlap_counts[0][1]);
    println!("Netflix-Prime Video overlap count: {}", overlap_counts[0][2]);
    println!("Netflix-Disney+ overlap count: {}", overlap_counts[0][3]);
    println!("Hulu-Prime Video overlap count: {}", overlap_counts[1][2]);
    println!("Hulu-Disney+ overlap count: {}", overlap_counts[1][3]);
    println!("Prime Video-Disney+ overlap count: {}", overlap_counts[2][3]);
    overlap_counts
}

// This function sorts the tv shows by different age categories and prints how many per age category are available on each platform
pub fn age_rating_sort(_graph: &Graph<String, bool, petgraph::Undirected>, tv_shows: Vec<TvShow>) -> HashMap<String, Vec<TvShow>> {
    // Here we group the TV shows into different age categories
    let mut age_categories: HashMap<String, Vec<TvShow>> = HashMap::new();
    for tv_show in tv_shows.into_iter() {
        let age_category = match tv_show.age.as_str() {
            "all" => "All",
            "7+" | "7" | "TV-G" | "TV-Y" | "TV-Y7" | "TV-Y7-FV" => "Children",
            "13+" | "13" | "TV-PG" => "Teen",
            "16+" | "16" | "TV-14" => "Mature",
            "18+" | "18" | "TV-MA" => "Adult",
            _ => "Unknown",
        };
        age_categories.entry(age_category.to_owned())
            .or_default()
            .push(tv_show);
    }

    // Here we count the number of TV shows that are available on each streaming platform for each age category
    for (age_category, tv_shows) in &age_categories {
        let mut platform_counts: HashMap<&str, usize> = HashMap::new();
        for tv_show in tv_shows {
            if tv_show.netflix == true {
                *platform_counts.entry("Netflix").or_default() += 1;
            }
            if tv_show.hulu == true {
                *platform_counts.entry("Hulu").or_default() += 1;
            }
            if tv_show.prime_video == true {
                *platform_counts.entry("Prime Video").or_default() += 1;
            }
            if tv_show.disney == true {
                *platform_counts.entry("Disney+").or_default() += 1;
            }
        }

        // And here we print the results to the console
        println!("\nAge Category: {}", age_category);
        for (platform, count) in platform_counts {
            println!("{}: {}", platform, count);
        }
    }
    age_categories
}