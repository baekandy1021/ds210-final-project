use std::fs::File;       
use std::io::BufRead;
use std::collections::VecDeque;
// node, list of directed edges, and adjacency list to represent graph
type Vertex = usize;
type ListOfEdges = Vec<(Vertex,Vertex)>;
type AdjacencyLists = Vec<Vec<Vertex>>;

//graph represented as a adjacency list of both inputs and outputs
#[derive(Debug)]
pub struct Graph {
    pub n: usize, 
    pub outedges: AdjacencyLists,
    pub inedges: AdjacencyLists,
}

impl Graph {
//reads file and outputs a the maxnode to identify and a list of edges
//inputs file name and outputs identifing node and the list of edges
    pub fn read_file(path: &str) -> (usize, ListOfEdges){
        let mut result: ListOfEdges = Vec::new();
        let file = File::open(path).expect("Could not open file");
        let mut buf_reader = std::io::BufReader::new(file).lines();
        let mut max_node = 0;
        buf_reader.next();
    
        for line in buf_reader {
            let line_str = line.expect("Error reading");
            let v: Vec<&str> = line_str.trim().split(',').collect();
    
            let x: Vertex = v[0].parse().expect("Invalid vertex");
            let y: Vertex = v[1].parse().expect("Invalid vertex");
    
            max_node = max_node.max(x).max(y);
            result.push((x,y));
        }
        (max_node +1,result)
    }
    //helper function to add directed edges
    pub fn add_directed_edges(&mut self, edges:&ListOfEdges) {
        for (u,v) in edges {
            self.outedges[*u].push(*v);
            self.inedges[*v].push(*u);
        }
    }

    //helper function to sort the list
    fn sort_graph_lists(&mut self) {
        for l in self.outedges.iter_mut() {
            l.sort();
        }
        for l in self.inedges.iter_mut() {
            l.sort();
        }
    }

    //helper function to add directed edges into the graph
    //inputs size and the list of edges and outputs the completed graph
    pub fn create_directed(n:usize, edges:&ListOfEdges)-> Graph {
        let mut g = Graph{n,outedges:vec![vec![];n],inedges:vec![vec![];n]};
        g.add_directed_edges(edges);
        g.sort_graph_lists();
        g                                        
    }

    //function that measures the degree centrality and outputs the in degree and the out degree
    //inputs graph and the list of edges and outputs both the in degree and the out degree in a tuple
    pub fn degree_centrality(graph: &Graph, edges: &ListOfEdges) -> (Vec<usize>,Vec<usize>) {
        let n  = graph.outedges.len();
        let mut out_degree = vec![0;n];
        let mut in_degree = vec![0;n];

        for u in 0..n {
            out_degree[u] = graph.outedges[u].len();
        }

        for &(_, v) in edges {
            if v <n {
                in_degree[v] +=1;
            }
        }

        (in_degree, out_degree)
    }
    // reorders the nodes by highest degree to lowest
    pub fn sort_nodes_by_degree(degrees: &Vec<usize>) -> Vec<(usize, usize)> {
        let mut nodes: Vec<(usize, usize)> = degrees.iter().enumerate().map(|(i, &d)| (i, d)).collect();
        nodes.sort_by(|a, b| b.1.cmp(&a.1));
        nodes
    }
    
    //calculates the closeness centrality by running bfs on each node. 
    //and sums all the distances
    //input is the graph and the output is a vec of each measure of centrality
    pub fn closeness(graph: &Graph) -> Vec<f64> {
        let mut cent = vec![0.0;graph.n];

        for i in 0..graph.n {
            let distances = bfs(i, &graph.outedges);
            let mut total_distance = 0;
            let mut reachable = 0;

            for &d in &distances {
                if let Some(dist) = d{
                    if dist > 0 {
                        total_distance +=dist;
                        reachable +=1;
                    }
                }
            }
            if total_distance >0 {
                cent[i] = (reachable as f64 -1.0) / total_distance as f64
            }
        }
        cent
    }
    //calculates betweenness centrality using Brandes algorithm 
    //by using bfs to find shortest paths and the node before(pred)
    //back-propagates dependencies from the shortest-path DAG using a stack to accumulate centrality values.
    //input is the graph the out put is a vec of each measure of centrality
    pub fn betweenness(graph: &Graph) -> Vec<f64> {
        let n = graph.n;
        let mut centrality = vec![0.0; n];

        for s in 0..n {
            let mut stack = Vec::new();
            let mut pred: Vec<Vec<usize>> = vec![Vec::new(); n];
            let mut sigma = vec![0;n];
            let mut dist = vec![-1;n];

            sigma[s] = 1;
            dist[s] = 0;

            let mut queue = VecDeque::new();
            queue.push_back(s);
            //bfs
            while let Some(v) = queue.pop_front() {
                stack.push(v);
                for &w in &graph.outedges[v] {
                    if dist[w] <0 {
                        queue.push_back(w);
                        dist[w] = dist[v] +1;
                    }
                    if dist[w] ==dist[v] +1 {
                        sigma[w] += sigma[v];
                        pred[w].push(v)
                    }
                }
            }
            //backprop
            let mut delta = vec![0.0;n];
            while let Some(w) = stack.pop() {
                for &v in &pred[w] {
                    delta[v] += (sigma[v] as f64 /sigma[w] as f64) * (1.0 + delta[w]);
                }
                if w!=s {
                    centrality[w] += delta[w];
                }
            }
        }
        centrality
    }
}


// bfs to compute the shortest path distances from a starting node to all other reachable nodes 
// uses a queue to explore the graph layer by layer and when the node is 
// visited for the first time, the distance is set to its parent's plus 1
//input is the starting node and the graph and the output is a vec of the distances
pub fn bfs(start: usize, graph: &AdjacencyLists) -> Vec<Option<usize>> {
    let mut queue = VecDeque::new();
    let mut distance = vec![None; graph.len()];
    queue.push_back(start);
    distance[start] = Some(0);

    while let Some(v) = queue.pop_front() {
        for &u in &graph[v] {
            if distance[u].is_none() {
                distance[u] = Some(distance[v].unwrap() +1);
                queue.push_back(u);
            }
        }
    }
    distance 
}