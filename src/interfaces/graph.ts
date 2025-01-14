// import type { BCEdgeAttributes, BCNode } from "src/graph/MyMultiGraph";
import type { AllFiles } from "src/graph/builders/explicit/files";
import type BreadcrumbsPlugin from "src/main";
import type { MaybePromise } from ".";
import { GCEdgeData, GCNodeData } from "wasm/pkg/breadcrumbs_graph_wasm";

export type BreadcrumbsError = {
	// TODO: Differentiate between invalid edge-field and invalid metadata-field values
	// BUT: Some errors might be a metadata field with an invalid edge-field value
	code:
		| "deprecated_field"
		| "invalid_field_value"
		| "invalid_setting_value"
		| "invalid_yaml"
		| "missing_other_plugin";
	message: string;
	path: string;
};

/** The values passed into safe_add_edge */
// export type EdgeToAdd = {
// 	source_id: string;
// 	target_id: string;
// 	attr: BCEdgeAttributes;
// };

export type EdgeBuilderResults = {
	nodes: GCNodeData[];
	edges: GCEdgeData[];
	errors: BreadcrumbsError[];
};

// NOTE: A completely different approach is to do it on a single node level
//   This way, we could rebuild the edges for a particular node as needed
/** "Extension" system. Takes in current state of plugin & graph, and adds to the graph */
export type ExplicitEdgeBuilder = (
	plugin: BreadcrumbsPlugin,
	all_files: AllFiles,
) => MaybePromise<EdgeBuilderResults>;
