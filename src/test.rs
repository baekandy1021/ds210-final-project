
#[cfg(test)]
mod tests {
    use super::*;
    use crate::Graph;
    use final_project::bfs;
    //test for graph construction
    #[test]
    fn test_graph_construction() {
        let edges = vec![(0, 1), (1, 2), (2, 0)];
        let graph = Graph::create_directed(3, &edges);

        assert_eq!(graph.outedges[0], vec![1]);
        assert_eq!(graph.outedges[1], vec![2]);
        assert_eq!(graph.outedges[2], vec![0]);

        assert_eq!(graph.inedges[0], vec![2]);
        assert_eq!(graph.inedges[1], vec![0]);
        assert_eq!(graph.inedges[2], vec![1]);
    }
    //check to make sure in and out degrees are calculated correctly
    #[test]
    fn test_degree_centrality() {
        let edges = vec![(0, 1), (0, 2), (2, 1)];
        let graph = Graph::create_directed(3, &edges);
        let (in_deg, out_deg) = Graph::degree_centrality(&graph, &edges);

        assert_eq!(out_deg, vec![2, 0, 1]); 
        assert_eq!(in_deg, vec![0, 2, 1]);  
    }
    //check to make sure bfs returns the right distance
    #[test]
    fn test_bfs() {
        let edges = vec![(0, 1), (1, 2)];
        let graph = Graph::create_directed(3, &edges);
        let distances = bfs(0, &graph.outedges);
    
        assert_eq!(distances[0], Some(0)); 
        assert_eq!(distances[1], Some(1)); 
        assert_eq!(distances[2], Some(2)); 
    }
    //check to make sure closeness is calculated correctly
    #[test]
    fn test_closeness_centrality() {
        let edges = vec![(0, 1), (1, 2), (2, 3)];
        let graph = Graph::create_directed(4, &edges);
        let closeness = Graph::closeness(&graph);
        
        assert!((closeness[1] - (2.0 / 3.0)) < 1e-6);
        assert!((closeness[0] - 0.5) < 1e-6);
        assert_eq!(closeness[3], 0.0);
    }
    //check to make sure betweenness is calculated correctly
    #[test]
    fn test_betweenness_centrality_small_graph() {
        let edges = vec![(0, 1), (1, 2), (2, 3)];
        let graph = Graph::create_directed(4, &edges);
        let bc = Graph::betweenness(&graph);

        assert!(bc[1] > 0.0); 
        assert!(bc[0] == 0.0); 
    }
}