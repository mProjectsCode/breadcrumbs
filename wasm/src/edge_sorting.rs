use std::str::FromStr;

use enum_dispatch::enum_dispatch;
use petgraph::visit::EdgeRef;
use wasm_bindgen::prelude::*;

use crate::{
    graph::NoteGraph,
    graph_data::EdgeStruct,
    graph_traversal::RecTraversalData,
    utils::{NoteGraphError, Result},
};

#[derive(Clone, Debug)]
pub enum SortField {
    Path,
    Basename,
    EdgeType,
    Implied,
    Neighbour(String),
}

impl FromStr for SortField {
    type Err = NoteGraphError;

    fn from_str(s: &str) -> Result<Self> {
        match s {
            "path" => Ok(SortField::Path),
            "basename" => Ok(SortField::Basename),
            "field" => Ok(SortField::EdgeType),
            "explicit" => Ok(SortField::Implied),
            s if s.starts_with("neighbour-field:") => Ok(SortField::Neighbour(
                s["neighbour-field:".len()..].to_string(),
            )),
            _ => Err(NoteGraphError::new("Invalid sort field")),
        }
    }
}

#[wasm_bindgen]
pub fn create_edge_sorter(field: String, reverse: bool) -> Result<EdgeSorter> {
    let sort_field = SortField::from_str(&field)?;
    Ok(EdgeSorter::new(sort_field, reverse))
}

#[wasm_bindgen]
pub fn sort_traversal_data(
    graph: &NoteGraph,
    traversal_data: Vec<RecTraversalData>,
    sorter: &EdgeSorter,
) -> Vec<RecTraversalData> {
    let mut traversal_data = traversal_data.clone();
    sorter.sort_traversal_data(graph, &mut traversal_data);

    traversal_data
}

#[wasm_bindgen]
pub fn sort_edges(
    graph: &NoteGraph,
    edges: Vec<EdgeStruct>,
    sorter: &EdgeSorter,
) -> Vec<EdgeStruct> {
    // utils::log(format!("Sorting edges: {:?}", edges));

    let mut edges = edges.clone();
    sorter.sort_edges(graph, &mut edges);

    edges
}

#[wasm_bindgen]
#[derive(Clone, Debug)]
pub struct EdgeSorter {
    field: SortField,
    reverse: bool,
}

impl EdgeSorter {
    pub fn new(field: SortField, reverse: bool) -> Self {
        EdgeSorter { field, reverse }
    }

    pub fn sort_edges(&self, graph: &NoteGraph, edges: &mut [EdgeStruct]) {
        let comparer = self.get_edge_comparer(graph);

        edges.sort_by(|a, b| self.apply_edge_ordering(graph, &comparer, a, b));
    }

    pub fn sort_traversal_data(&self, graph: &NoteGraph, edges: &mut [RecTraversalData]) {
        let comparer = self.get_edge_comparer(graph);

        edges.sort_by(|a, b| self.apply_edge_ordering(graph, &comparer, &a.edge, &b.edge));
    }

    fn get_edge_comparer<'a>(&self, graph: &'a NoteGraph) -> Comparer<'a> {
        match self.field.clone() {
            SortField::Path => PathComparer.into(),
            SortField::Basename => BasenameComparer.into(),
            SortField::EdgeType => EdgeTypeComparer.into(),
            SortField::Implied => ImpliedComparer.into(),
            SortField::Neighbour(neighbour_field) => {
                NeighbourComparer::new(neighbour_field, graph).into()
            }
        }
    }

    fn apply_edge_ordering(
        &self,
        graph: &NoteGraph,
        comparer: &impl EdgeComparer,
        a: &EdgeStruct,
        b: &EdgeStruct,
    ) -> std::cmp::Ordering {
        let ordering = comparer.compare(graph, a, b);

        if self.reverse {
            ordering.reverse()
        } else {
            ordering
        }
    }
}

#[enum_dispatch]
pub trait EdgeComparer {
    fn compare(&self, graph: &NoteGraph, a: &EdgeStruct, b: &EdgeStruct) -> std::cmp::Ordering;
}

#[enum_dispatch(EdgeComparer)]
pub enum Comparer<'a> {
    PathComparer,
    BasenameComparer,
    EdgeTypeComparer,
    ImpliedComparer,
    NeighbourOrdering(NeighbourComparer<'a>),
}

#[derive(Default)]
pub struct PathComparer;

impl EdgeComparer for PathComparer {
    fn compare(&self, graph: &NoteGraph, a: &EdgeStruct, b: &EdgeStruct) -> std::cmp::Ordering {
        a.target_path_ref(graph).cmp(b.target_path_ref(graph))
    }
}

#[derive(Default)]
pub struct BasenameComparer;

impl EdgeComparer for BasenameComparer {
    fn compare(&self, graph: &NoteGraph, a: &EdgeStruct, b: &EdgeStruct) -> std::cmp::Ordering {
        let a_target = a.target_path_ref(graph);
        let b_target = b.target_path_ref(graph);
        let a_basename = a_target.split('/').last().unwrap();
        let b_basename = b_target.split('/').last().unwrap();

        a_basename.cmp(b_basename)
    }
}

#[derive(Default)]
pub struct EdgeTypeComparer;

impl EdgeComparer for EdgeTypeComparer {
    fn compare(&self, _graph: &NoteGraph, a: &EdgeStruct, b: &EdgeStruct) -> std::cmp::Ordering {
        a.edge_type.cmp(&b.edge_type)
    }
}

#[derive(Default)]
pub struct ImpliedComparer;

impl EdgeComparer for ImpliedComparer {
    fn compare(&self, graph: &NoteGraph, a: &EdgeStruct, b: &EdgeStruct) -> std::cmp::Ordering {
        if a.explicit(graph) == b.explicit(graph) {
            a.target_path_ref(graph).cmp(b.target_path_ref(graph))
        } else if a.explicit(graph) {
            std::cmp::Ordering::Less
        } else {
            std::cmp::Ordering::Greater
        }
    }
}

pub struct NeighbourComparer<'a> {
    neighbour_field: String,
    graph: &'a NoteGraph,
}

impl<'a> NeighbourComparer<'a> {
    pub fn new(neighbour_field: String, graph: &'a NoteGraph) -> Self {
        NeighbourComparer {
            neighbour_field,
            graph,
        }
    }
}

impl EdgeComparer for NeighbourComparer<'_> {
    fn compare(&self, graph: &NoteGraph, a: &EdgeStruct, b: &EdgeStruct) -> std::cmp::Ordering {
        let neighbour_field = vec![self.neighbour_field.clone()];

        let a_neighbour = self
            .graph
            .int_iter_outgoing_edges(a.target_index)
            .find(|edge| {
                edge.weight()
                    .matches_edge_filter_string(Some(&neighbour_field))
            })
            .and_then(|x| self.graph.int_get_node_weight(x.target()).ok());

        let b_neighbour = self
            .graph
            .int_iter_outgoing_edges(b.target_index)
            .find(|edge| {
                edge.weight()
                    .matches_edge_filter_string(Some(&neighbour_field))
            })
            .and_then(|x| self.graph.int_get_node_weight(x.target()).ok());

        match (a_neighbour, b_neighbour) {
            (Some(a_neighbour), Some(b_neighbour)) => a_neighbour.path.cmp(&b_neighbour.path),
            (Some(_), None) => std::cmp::Ordering::Less,
            (None, Some(_)) => std::cmp::Ordering::Greater,
            (None, None) => a.target_path_ref(graph).cmp(b.target_path_ref(graph)),
        }
    }
}
