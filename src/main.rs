use final_project::{read_file, Graph};

fn main() {
    let (n,edges) = read_file("soc-sign-bitcoinotc.csv");
    let graph1 = Graph::create_directed(n,&edges);

    let (in_deg,out_deg) = Graph::degree_centrality(&graph1,&edges);
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
    let closeness = Graph::closeness(&graph1);
    let mut sorted_closeness: Vec<(usize,f64)> = closeness.iter().enumerate().map(|(i,&c)|(i,c)).collect();
    sorted_closeness.sort_by(|a,b| b.1.partial_cmp(&a.1).unwrap());

    println!("\nTop 10 nodes by closeness centrality:");
    for (node, score) in sorted_closeness.iter().take(10) {
        println!("Node {}: {:.4}", node, score);
    }

    // Betweenness Centrality
    let betweenness = Graph::betweenness(&graph1);
    let mut sorted_betweenness: Vec<(usize, f64)> = betweenness.iter().enumerate().map(|(i, &b)| (i, b)).collect();
    sorted_betweenness.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap());

    println!("\nTop 10 nodes by betweenness centrality:");
    for (node, score) in sorted_betweenness.iter().take(10) {
        println!("Node {}: {:.4}", node, score);
    }
}

mod test;
