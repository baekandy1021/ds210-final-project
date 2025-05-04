# Blockchain Centrality Analysis (Rust Project)

## A. Project Overview

**Goal**  
Identify the most "popular" or "trusted" users in the Bitcoin OTC trust network using centrality measures on a directed graph.

**Dataset**  
- **Source:** [Bitcoin OTC Trust Network (SNAP @ Stanford)](https://snap.stanford.edu/data/soc-sign-bitcoin-otc.html)  
- **File:** `soc-sign-bitcoinotc.csv`  
- **Size:** ~35,000 edges; ~6,000 nodes

---

## B. Data Processing

**Loading Method**  
The original dataset is preprocessed using a Python script (`clean.py`) to clean timestamps and ensure CSV validity.

**Transformations Applied**  
- Invalid or corrupt rows are dropped.
- Dataset is split into yearly CSVs (based on UNIX timestamps).
- Each yearly CSV is then used to generate a subgraph in Rust.

---

## C. Code Structure

### Modules

- `main.rs` — Runs full pipeline: reads files, builds graph, computes centrality.
- `lib.rs` — Contains `Graph` struct and core graph algorithms.
- `test.rs` — Unit tests for data parsing and centrality logic.

---

### Key Functions & Types

#### `build_graph_from_file(path: &str) -> Graph`
- **Purpose:** Construct a directed trust graph from CSV data.
- **Inputs:** CSV file path with (source, target, weight, timestamp).
- **Outputs:** `Graph` object with edge list.
- **Logic:** Parses each row and populates an adjacency list with weighted edges.

#### `calculate_out_degree_centrality(&self) -> Vec<(u32, usize)>`
- **Purpose:** Rank users by number of trust ratings they gave.
- **Inputs:** Graph reference.
- **Outputs:** Vector of (user ID, count), sorted by count descending.
- **Logic:** Iterates through the adjacency map and counts edges per source node.

---

### Structs

#### `Graph`
- **Purpose:** Represents a directed network of trust.
- **Usage:** Stores edges as an adjacency list using `HashMap<u32, Vec<(u32, f64)>>`.

---

### Main Workflow

1. `clean.py` splits the master CSV into yearly files.
2. `main.rs` loads each CSV into a `Graph`.
3. `calculate_out_degree_centrality` determines the most connected trust-givers.
4. Results printed to stdout per year.

---

## D. Tests

### Output from `cargo test`