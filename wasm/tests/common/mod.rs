use breadcrumbs_graph_wasm::{
    graph::NoteGraph,
    graph_construction::{GraphConstructionEdgeData, GraphConstructionNodeData},
};

/// Generate a tree of nodes with explicit down edges for testing purposes.
pub fn tdata_generate_tree(
    depth: u32,
    branches: u32,
) -> (
    Vec<GraphConstructionNodeData>,
    Vec<GraphConstructionEdgeData>,
) {
    let root = GraphConstructionNodeData::new("root".to_string(), vec![], true, false, false);
    let mut edges = vec![];

    let mut stack = vec![];

    for i in 0..branches {
        stack.push(i.to_string());
        edges.push(GraphConstructionEdgeData::new(
            "root".to_string(),
            i.to_string(),
            "down".to_string(),
        ));
    }

    let mut nodes = vec![root];

    while !stack.is_empty() {
        let current = stack.pop().unwrap();

        if current.len() < depth as usize {
            for i in 0..branches {
                let next = format!("{}{}", current, i);
                edges.push(GraphConstructionEdgeData::new(
                    current.clone(),
                    next.clone(),
                    "down".to_string(),
                ));
                stack.push(next);
            }
        }

        nodes.push(GraphConstructionNodeData::new(
            current,
            vec![],
            true,
            false,
            false,
        ));
    }

    (nodes, edges)
}

pub fn tdata_to_graph(
    data: (
        Vec<GraphConstructionNodeData>,
        Vec<GraphConstructionEdgeData>,
    ),
) -> NoteGraph {
    let mut graph = NoteGraph::new();

    graph.build_graph(data.0, data.1);

    graph
}
