mod tv_show;
mod graph;
mod tv_graph_analyzing;
mod tests;

use csv::Reader;
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    #[cfg(test)] {
        tests::test_parse_tv_shows();
        tests::test_build_graph();
        tests::test_calculate_degree_centrality();
        tests::test_calculate_degree_distribution();
        tests::test_calculate_exclusive_content();
        tests::test_calculate_platform_overlap();
        tests::test_age_rating_sort();
    }

    let mut reader = Reader::from_path("tv_shows.csv")?;
    let tv_shows = tv_show::parse_tv_shows(&mut reader)?;
    let mut graph = graph::build_graph(&tv_shows);
    tv_graph_analyzing::calculate_degree_centrality(&mut graph);
    tv_graph_analyzing::calculate_degree_distribution(&mut graph, &tv_shows);
    tv_graph_analyzing::calculate_exclusive_content(&mut graph, &tv_shows);
    tv_graph_analyzing::calculate_platform_overlap(&mut graph);
    tv_graph_analyzing::age_rating_sort(&mut graph, tv_shows);

    Ok(())
}