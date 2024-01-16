import type { MultiGraph } from "graphology";
import type { Direction } from "src/const/hierarchies";
import type { AllFiles } from "src/graph/builders/explicit/files";
import type BreadcrumbsPlugin from "src/main";
import type { Hierarchy } from "./hierarchies";

export type BCNodeAttributes = {
	/** .md file exists  */
	resolved: boolean;
	aliases?: string[];
};

export type BCEdgeAttributes = {
	/** The hierarchy index */
	hierarchy_i: number;
	/** The direction of the field in the hierarchy */
	dir: Direction;
	/** The hierarchy field
	 * null if the implied edge has no opposite field
	 */
	field: string | null;
} & (
	| {
			explicit: true;
			source: // Just a regular `up: [[link]]` or `down:: [[link]]` in the content/frontmatter of a note
			// The two are not distinguished, because Dataview doesn't distinguish them
			"typed_link" | "tag_note";
	  }
	| {
			explicit: false;
			implied_kind: keyof Hierarchy["implied_relationships"];
	  }
);

export type BCGraph = MultiGraph<BCNodeAttributes, BCEdgeAttributes>;

/** "Extension" system. Takes in current state of plugin & graph, and adds to the graph */
export type GraphBuilder = (
	graph: BCGraph,
	plugin: BreadcrumbsPlugin,
	all_files: AllFiles,
) => BCGraph;

export type GraphNode = {
	id: string;
	attr: BCNodeAttributes;
};

export type GraphEdge = {
	id: string;
	attr: BCEdgeAttributes;
	source_id: string;
	target_id: string;
	source_attr: BCNodeAttributes;
	target_attr: BCNodeAttributes;
	undirected: boolean;
};