# Blockchain Centrality Analysis (Rust Project)

## A. Project Overview

**Goal**  
Identify the most "popular" or "trusted" users in the Bitcoin OTC trust network using centrality measures such as out-degree, closeness, and betweenness.

**Dataset**  
- **Source:** [Bitcoin OTC Trust Network (SNAP @ Stanford)](https://snap.stanford.edu/data/soc-sign-bitcoin-otc.html)  
- **Filename:** `soc-sign-bitcoinotc.csv`  
- **Size:** ~35,000 edges; ~6,000 nodes  
- **Note:** File was cleaned and seperated by years for subgraph analysis. The subgraphs analyzed were 2011,2012, and 2012 as they had the most nodes.

---

## B. Data Processing

**Loading Method**  
Data is cleaned and split using the Python script `clean.py`. Each yearly CSV is then loaded in Rust using `Graph::read_file()`.

**Transformations Applied**  
- Rows with invalid or corrupt data (e.g., missing/invalid timestamps) are removed.
- Edges are split into yearly subgraphs based on timestamp parsing.
- Cleaned data is written as individual yearly CSV files for modular analysis.

---

## C. Code Structure

### Modules

- `main.rs`  
  Controls the workflow: loads per-year graphs, runs centrality algorithms, prints output.

- `lib.rs`  
  Contains the `Graph` struct and all algorithm implementations (degree, closeness, betweenness).

- `test.rs`  
  Contains unit tests for graph construction and centrality logic.

---

### Key Functions & Types

#### `Graph`
- **Purpose:** Stores graph data as adjacency lists.
- **Fields:**
  - `n`: number of nodes
  - `outedges`: outgoing neighbors
  - `inedges`: incoming neighbors

#### `Graph::read_file(path: &str)`
- **Purpose:** Load a CSV file into a list of edges and node count.
- **Inputs:** File path.
- **Outputs:** `(usize, ListOfEdges)` tuple.

#### `Graph::create_directed(n, edges)`
- **Purpose:** Build a directed graph from edges.
- **Logic:** Initializes adjacency lists and populates both in- and outedges.

#### `Graph::degree_centrality()`
- **Purpose:** Calculates in-degree and out-degree for each node.
- **Output:** Tuple of `(in_degrees, out_degrees)`.

#### `Graph::closeness()`
- **Purpose:** Computes closeness centrality using BFS from each node.

#### `Graph::betweenness()`
- **Purpose:** Computes betweenness centrality using Brandes' algorithm.

#### `bfs(start, graph)`
- **Purpose:** Helper function for computing shortest paths via BFS.
- **Used In:** `closeness()`.

---

### Main Workflow

1. Python script preprocesses and splits data into yearly files.
2. `main.rs` reads each CSV and builds a `Graph`.
3. Runs centrality measures for whole graph.
4. Results are printed showing top-ranked users.
5. Creates subgraphs per years. 
6. Results are printed showing top-ranked users per year. 

---

## D. Tests

**Command:**  
```bash
cargo test
```

**Output:**  
```
running 5 tests
test test::tests::test_degree_centrality ... ok
test test::tests::test_bfs ... ok
test test::tests::test_graph_construction ... ok
test test::tests::test_closeness_centrality ... ok
test test::tests::test_betweenness_centrality_small_graph ... ok

test result: ok. 5 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s
```

**What They Check:**
- `test_degree_centrality`:  Checks correct computation of in-degree and out-degree for all nodes.
- `test_bfs`: Checks that bfs correctly calculates shortest distances from the start node.
- `test_graph_construction`: Checks that the graph is built correctly from the edge list
- `test_closeness_centrality`: Checks that closeness centrality scores are correctly calculated using BFS-based distances.
- `test_betweenness_centrality_small_graph`: Checks that betweenness centrality is accurately computed on a known small graph.
---

## E. Results

**Sample Output (2011):**
```
Sorted by out-degrees:
1. Node 35 -> out-degree: 763
2. Node 2642 -> out-degree: 406
3. Node 1810 -> out-degree: 404
4. Node 2125 -> out-degree: 397
5. Node 2028 -> out-degree: 293
6. Node 905 -> out-degree: 264
7. Node 4172 -> out-degree: 264
8. Node 7 -> out-degree: 232
9. Node 1 -> out-degree: 215
10. Node 3129 -> out-degree: 212

Sorted by in-degrees:
1. Node 35 <- in-degree: 535
2. Node 2642 <- in-degree: 412
3. Node 1810 <- in-degree: 311
4. Node 2028 <- in-degree: 279
5. Node 905 <- in-degree: 264
6. Node 1 <- in-degree: 226
7. Node 4172 <- in-degree: 222
8. Node 7 <- in-degree: 216
9. Node 4197 <- in-degree: 203
10. Node 13 <- in-degree: 191

Top 10 nodes by closeness centrality:
Node 5067: 0.8000
Node 5068: 0.8000
Node 5069: 0.8000
Node 5070: 0.8000
Node 5072: 0.8000
Node 3565: 0.7500
Node 4927: 0.7500
Node 3556: 0.6667
Node 4074: 0.6667
Node 5071: 0.6667

Top 10 nodes by betweenness centrality:
Node 35: 4912524.0931
Node 2642: 2150250.4002
Node 1810: 1712259.4040
Node 905: 1664247.0983
Node 1: 1555774.6852
Node 4172: 1448359.9509
Node 2125: 1439193.0197
Node 7: 1376713.8548
Node 2028: 1340650.1317
Node 1953: 1116749.4335
------------------------------------------------------
2011 Subgraph

Top 10 nodes by closeness centrality:
Node 696: 0.5000
Node 7: 0.4130
Node 1: 0.3810
Node 64: 0.3717
Node 537: 0.3633
Node 41: 0.3631
Node 35: 0.3627
Node 592: 0.3611
Node 4: 0.3605
Node 202: 0.3602

Top 10 nodes by betweenness centrality:
Node 7: 440661.5568
Node 1: 286288.0921
Node 35: 247335.5206
Node 832: 179453.5175
Node 1162: 137621.9712
Node 202: 113440.6969
Node 41: 111772.7955
Node 775: 110326.0751
Node 64: 108234.4880
Node 1386: 106236.4527
------------------------------------------------------
2012 Subgraph

Top 10 nodes by closeness centrality:
Node 1669: 0.5000
Node 2383: 0.5000
Node 2622: 0.5000
Node 2623: 0.5000
Node 2694: 0.5000
Node 2712: 0.5000
Node 2717: 0.5000
Node 2745: 0.5000
Node 2746: 0.5000
Node 2028: 0.4187

Top 10 nodes by betweenness centrality:
Node 35: 460536.2992
Node 2028: 429810.5492
Node 1810: 334379.7896
Node 1953: 233526.1228
Node 905: 187526.2517
Node 1899: 176680.7360
Node 2067: 159206.3416
Node 2642: 125506.3224
Node 304: 120370.6486
Node 13: 114722.0592
------------------------------------------------------
2013 Subgraph

Top 10 nodes by closeness centrality:
Node 5067: 0.8000
Node 5068: 0.8000
Node 5069: 0.8000
Node 5070: 0.8000
Node 5072: 0.8000
Node 3565: 0.7500
Node 4927: 0.7500
Node 3556: 0.6667
Node 4074: 0.6667
Node 5071: 0.6667

Top 10 nodes by betweenness centrality:
Node 2642: 947650.6175
Node 4172: 798444.6046
Node 35: 704145.9563
Node 3735: 372188.2790
Node 2388: 349770.7489
Node 2125: 345766.5738
Node 3129: 316347.0684
Node 1018: 270007.5958
Node 4197: 261997.9569
Node 1810: 257808.4406
```

**Interpretation**  
## E. Results & Interpretation
The analysis of centrality metrics in the Bitcoin OTC trust network reveals that certain users—especially Node 35—consistently held central roles across all years. High out-degree nodes were active in rating others, suggesting participation and influence as trust-givers, while high in-degree nodes were widely trusted by peers. Closeness centrality highlighted users who could efficiently interact across the network, often part of tightly connected clusters. Betweenness centrality identified key intermediaries—such as Nodes 35, 2642, and 905—who served as bridges between different groups. Over time, new users began to emerge as central figures, reflecting an evolving trust structure and shifting network dynamics.
## F. Usage Instructions

**Step 1: Preprocess data (Python)**
```bash
python3 clean.py
```

**Step 2: Build project**
```bash
cargo build --release
```

**Step 3: Run**
```bash
cargo run --release
```

**Expected Runtime:**  
Approximately 1–3 seconds per yearly file.

---

## G. AI-Assistance Disclosure and Other Citations

**ChatGPT Assistance:**  
- Helped generate documentation and code comments.
- Refined `README.md` structure and centrality function annotations.
- Various debugging/syntax errors help.

**External Sources:**  
- [SNAP Dataset Documentation](https://snap.stanford.edu/data/soc-sign-bitcoin-otc.html)
- Brandes, Ulrik. "A faster algorithm for betweenness centrality." Journal of Mathematical Sociology (2001).