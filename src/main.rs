use final_project::Graph;

fn main() {
    // Full Graph analysis
    let (n,edges) = Graph::read_file("soc-sign-bitcoinotc.csv");
    let graph = Graph::create_directed(n,&edges);

    let (in_deg,out_deg) = Graph::degree_centrality(&graph,&edges);
    let sorted_out = Graph::sort_nodes_by_degree(&out_deg);
    println!("Sorted by out-degrees:");
    for (i, (node, degree)) in sorted_out.iter().enumerate().take(10) {
        println!("{}. Node {} -> out-degree: {}", i + 1, node, degree);
    }

    let sorted_in = Graph::sort_nodes_by_degree(&in_deg);
    println!("\nSorted by in-degrees:");
    for (i, (node, degree)) in sorted_in.iter().enumerate().take(10) {
        println!("{}. Node {} <- in-degree: {}", i + 1, node, degree);
    }

    //Closeness Centrality
    let closeness = Graph::closeness(&graph);
    let mut sorted_closeness: Vec<(usize,f64)> = closeness.iter().enumerate().map(|(i,&c)|(i,c)).collect();
    sorted_closeness.sort_by(|a,b| b.1.partial_cmp(&a.1).unwrap());

    println!("\nTop 10 nodes by closeness centrality:");
    for (node, score) in sorted_closeness.iter().take(10) {
        println!("Node {}: {:.4}", node, score);
    }

    // Betweenness Centrality
    let betweenness = Graph::betweenness(&graph);
    let mut sorted_betweenness: Vec<(usize, f64)> = betweenness.iter().enumerate().map(|(i, &b)| (i, b)).collect();
    sorted_betweenness.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap());

    println!("\nTop 10 nodes by betweenness centrality:");
    for (node, score) in sorted_betweenness.iter().take(10) {
        println!("Node {}: {:.4}", node, score);
    }

    println!("------------------------------------------------------");
    println!("2011 Subgraph");

    // Closeness and Betweenness for 2011 subgraph
    let (n,edges) = Graph::read_file("bitcoinotc_by_year/bitcoinotc_2011.csv");
    let graph2011 = Graph::create_directed(n,&edges);

    let closeness = Graph::closeness(&graph2011);
    let mut sorted_closeness: Vec<(usize,f64)> = closeness.iter().enumerate().map(|(i,&c)|(i,c)).collect();
    sorted_closeness.sort_by(|a,b| b.1.partial_cmp(&a.1).unwrap());

    println!("\nTop 10 nodes by closeness centrality:");
    for (node, score) in sorted_closeness.iter().take(10) {
        println!("Node {}: {:.4}", node, score);
    }

    let betweenness = Graph::betweenness(&graph2011);
    let mut sorted_betweenness: Vec<(usize, f64)> = betweenness.iter().enumerate().map(|(i, &b)| (i, b)).collect();
    sorted_betweenness.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap());

    println!("\nTop 10 nodes by betweenness centrality:");
    for (node, score) in sorted_betweenness.iter().take(10) {
        println!("Node {}: {:.4}", node, score);
    }
    println!("------------------------------------------------------");
    println!("2012 Subgraph");

    // closeness and betweenness for 2012 subgraph
    let (n,edges) = Graph::read_file("bitcoinotc_by_year/bitcoinotc_2012.csv");
    let graph2012 = Graph::create_directed(n,&edges);

    let closeness = Graph::closeness(&graph2012);
    let mut sorted_closeness: Vec<(usize,f64)> = closeness.iter().enumerate().map(|(i,&c)|(i,c)).collect();
    sorted_closeness.sort_by(|a,b| b.1.partial_cmp(&a.1).unwrap());

    println!("\nTop 10 nodes by closeness centrality:");
    for (node, score) in sorted_closeness.iter().take(10) {
        println!("Node {}: {:.4}", node, score);
    }

    let betweenness = Graph::betweenness(&graph2012);
    let mut sorted_betweenness: Vec<(usize, f64)> = betweenness.iter().enumerate().map(|(i, &b)| (i, b)).collect();
    sorted_betweenness.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap());

    println!("\nTop 10 nodes by betweenness centrality:");
    for (node, score) in sorted_betweenness.iter().take(10) {
        println!("Node {}: {:.4}", node, score);
    }
    println!("------------------------------------------------------");
    println!("2013 Subgraph");

    // closeness and betweenness for 2013 subgraph
    let (n,edges) = Graph::read_file("bitcoinotc_by_year/bitcoinotc_2013.csv");
    let graph2013 = Graph::create_directed(n,&edges);

    let closeness = Graph::closeness(&graph2013);
    let mut sorted_closeness: Vec<(usize,f64)> = closeness.iter().enumerate().map(|(i,&c)|(i,c)).collect();
    sorted_closeness.sort_by(|a,b| b.1.partial_cmp(&a.1).unwrap());

    println!("\nTop 10 nodes by closeness centrality:");
    for (node, score) in sorted_closeness.iter().take(10) {
        println!("Node {}: {:.4}", node, score);
    }

    let betweenness = Graph::betweenness(&graph2013);
    let mut sorted_betweenness: Vec<(usize, f64)> = betweenness.iter().enumerate().map(|(i, &b)| (i, b)).collect();
    sorted_betweenness.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap());

    println!("\nTop 10 nodes by betweenness centrality:");
    for (node, score) in sorted_betweenness.iter().take(10) {
        println!("Node {}: {:.4}", node, score);
    }
}

mod test;
