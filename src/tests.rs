#[test] 
fn test_parse_tv_shows() {
    let input = "\
        show_id,title,release_year,rating,Netflix,Hulu,Prime Video,Disney+\n\
        1,Stranger Things,2016,TV-14,1,0,0,0\n\
        2,Enola Holmes,2020,PG-13,1,0,1,0\n\
        ";

    let mut reader = Reader::from_reader(Cursor::new(input));
    let tv_shows = parse_tv_shows(&mut reader).unwrap();

    assert_eq!(tv_shows.len(), 2);

    assert_eq!(tv_shows[0].title, "Stranger Things");
    assert_eq!(tv_shows[0].age, "TV-14");
    assert_eq!(tv_shows[0].netflix, true);
    assert_eq!(tv_shows[0].hulu, false);
    assert_eq!(tv_shows[0].prime_video, false);
    assert_eq!(tv_shows[0].disney, false);

    assert_eq!(tv_shows[1].title, "Enola Holmes");
    assert_eq!(tv_shows[1].age, "PG-13");
    assert_eq!(tv_shows[1].netflix, true);
    assert_eq!(tv_shows[1].hulu, false);
    assert_eq!(tv_shows[1].prime_video, true);
    assert_eq!(tv_shows[1].disney, false);
}

#[test]
fn test_build_graph() {
    let tv_shows = vec![
        TvShow {
            title: "Stranger Things".to_string(),
            age: "TV-14".to_string(),
            netflix: true,
            hulu: false,
            prime_video: false,
            disney: false,
        },
        TvShow {
            title: "Breaking Bad".to_string(),
            age: "TV-MA".to_string(),
            netflix: false,
            hulu: true,
            prime_video: true,
            disney: false,
        },
        TvShow {
            title: "The Mandalorian".to_string(),
            age: "TV-14".to_string(),
            netflix: false,
            hulu: false,
            prime_video: false,
            disney: true,
        },
    ];

    let graph = build_graph(&tv_shows);

    // Check that the graph has the expected number of nodes and edges
    assert_eq!(graph.node_count(), 3);
    assert_eq!(graph.edge_count(), 4);

    // Check that the node labels are correct
    let node_labels: Vec<_> = graph.node_indices().map(|i| &graph[i]).collect();
    assert_eq!(node_labels, ["Stranger Things", "Breaking Bad", "The Mandalorian"]);

    // Check that the edge labels are correct
    assert!(graph.find_edge(node_indices[0], NodeIndex::new(0)).is_some());
    assert!(graph.find_edge(node_indices[1], NodeIndex::new(1)).is_some());
    assert!(graph.find_edge(node_indices[1], NodeIndex::new(2)).is_some());
    assert!(graph.find_edge(node_indices[2], NodeIndex::new(3)).is_some());
}

#[test]
fn test_calculate_degree_centrality() {
    let tv_shows = vec![
        TvShow { title: "Stranger Things".to_string(), netflix: true, age: "16+".to_string(), hulu: false, prime_video: false, disney: false },
        TvShow { title: "The Handmaid's Tale".to_string(), netflix: false, age: "18+".to_string(), hulu: true, prime_video: false, disney: false },
        TvShow { title: "The Marvelous Mrs. Maisel".to_string(), netflix: false, age: "16+".to_string(), hulu: false, prime_video: true, disney: false },
        TvShow { title: "WandaVision".to_string(), netflix: false, age: "7+".to_string(), hulu: false, prime_video: false, disney: true },
    ];
    let graph = build_graph(&tv_shows);
    let degrees = calculate_degree_centrality(&graph);
    assert_eq!(degrees[0], 1);
    assert_eq!(degrees[1], 1);
    assert_eq!(degrees[2], 1);
    assert_eq!(degrees[3], 1);
}

#[test]
fn test_calculate_degree_distribution() {
    let mut graph = Graph::<String, bool, petgraph::Undirected>::new_undirected();
    let tv_shows = vec![
        TvShow {
            title: "The Crown".to_string(),
            netflix: true,
            age: "18+".to_string(),
            hulu: false,
            prime_video: false,
            disney: false,
        },
        TvShow {
            title: "Community".to_string(),
            netflix: false,
            age: "13+".to_string(),
            hulu: true,
            prime_video: true,
            disney: false,
        },
        TvShow {
            title: "Parks and Recreation".to_string(),
            netflix: true,
            age: "13+".to_string(),
            hulu: false,
            prime_video: true,
            disney: false,
        },
        TvShow {
            title: "The Mandalorian".to_string(),
            netflix: false,
            age: "7+".to_string(),
            hulu: false,
            prime_video: false,
            disney: true,
        },
    ];

    let node_indices = tv_shows
        .iter()
        .map(|tv_show| graph.add_node(tv_show.title.clone()))
        .collect::<Vec<_>>();

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

    let degree_distribution = calculate_degree_distribution(&graph, &tv_shows);
    assert_eq!(degree_distribution[0], 0); // No TV shows with 0 neighbors
    assert_eq!(degree_distribution[1], 1); // Only "The Mandalorian" has 1 neighbor
    assert_eq!(degree_distribution[2], 2); // "The Crown" and "Community" have 2 neighbors each
    assert_eq!(degree_distribution[3], 1); // Only "Parks and Recreation" has 3 neighbors
    assert_eq!(degree_distribution.len(), 4); // There are only 4 possible degrees in this graph
}

#[test]
fn test_calculate_exclusive_content() {
    let mut tv_shows = Vec::new();
    tv_shows.push(TvShow::new("The Office", true, true, false, false));
    tv_shows.push(TvShow::new("Parks and Recreation", false, true, true, false));
    tv_shows.push(TvShow::new("Brooklyn Nine-Nine", true, false, false, true));
    tv_shows.push(TvShow::new("The Mandalorian", false, false, true, true));
    tv_shows.push(TvShow::new("Stranger Things", true, false, true, false));
    tv_shows.push(TvShow::new("The Boys", false, true, true, false));
    tv_shows.push(TvShow::new("Jack Ryan", false, false, true, false));
    tv_shows.push(TvShow::new("WandaVision", false, false, false, true));

    let mut graph = Graph::<String, bool, petgraph::Undirected>::new_undirected();

    let node_indices = tv_shows.iter().enumerate().map(|(i, tv_show)| {
        graph.add_node(tv_show.title.clone());
        NodeIndex::new(i + 4)
    }).collect::<Vec<_>>();

    graph.add_node("Netflix".to_string());
    graph.add_node("Hulu".to_string());
    graph.add_node("Prime Video".to_string());
    graph.add_node("Disney+".to_string());

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

    let exclusive_counts = calculate_exclusive_content(&graph, &tv_shows);

    assert_eq!(exclusive_counts[0], 50);
    assert_eq!(exclusive_counts[1], 33);
    assert_eq!(exclusive_counts[2], 50);
    assert_eq!(exclusive_counts[3], 50);
}

#[test]
fn test_calculate_platform_overlap() {
    let mut graph = Graph::<String, bool, petgraph::Undirected>::new_undirected();
    let netflix = graph.add_node("Netflix".to_owned());
    let hulu = graph.add_node("Hulu".to_owned());
    let prime_video = graph.add_node("Prime Video".to_owned());
    let disney_plus = graph.add_node("Disney+".to_owned());
    for i in 0..10 {
        let tv_show = graph.add_node(format!("TV Show {}", i));
        if i % 2 == 0 {
            graph.add_edge(tv_show, netflix, true);
        }
        if i % 3 == 0 {
            graph.add_edge(tv_show, hulu, true);
        }
        if i % 4 == 0 {
            graph.add_edge(tv_show, prime_video, true);
        }
        if i % 5 == 0 {
            graph.add_edge(tv_show, disney_plus, true);
        }
    }

    let overlap_counts = calculate_platform_overlap(&graph);

    assert_eq!(overlap_counts[0][1], 2);
    assert_eq!(overlap_counts[0][2], 2);
    assert_eq!(overlap_counts[0][3], 1);
    assert_eq!(overlap_counts[1][2], 2);
    assert_eq!(overlap_counts[1][3], 1);
    assert_eq!(overlap_counts[2][3], 1);
}

#[test]
fn test_age_rating_sort() {
    let graph = Graph::<String, bool, petgraph::Undirected>::new();
    let tv_shows: Vec<TvShow> = vec![
        TvShow {
            title: "Avatar: The Last Airbender".to_string(),
            age: "7+".to_string(),
            netflix: false,
            hulu: true,
            prime_video: true,
            disney: false,
        },
        TvShow {
            title: "The Boys".to_string(),
            age: "18+".to_string(),
            netflix: false,
            hulu: false,
            prime_video: true,
            disney: false,
        },
    ];
    let result = age_rating_sort(&graph, tv_shows);
    assert_eq!(result.len(), 2);
    assert_eq!(result.contains_key("Children"), true);
    assert_eq!(result.contains_key("Adult"), true);
    assert_eq!(result.get("Children").unwrap().len(), 1);
    assert_eq!(result.get("Adult").unwrap().len(), 1);
    assert_eq!(result.get("Children").unwrap()[0].title, "Avatar: The Last Airbender");
    assert_eq!(result.get("Adult").unwrap()[0].title, "The Boys");
    assert_eq!(result.get("Children").unwrap()[0].age, "7+");
    assert_eq!(result.get("Adult").unwrap()[0].age, "18+");
    assert_eq!(result.get("Children").unwrap()[0].hulu, true);
    assert_eq!(result.get("Adult").unwrap()[0].prime_video, true);
}