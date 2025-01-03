import type { EdgeSortId } from "src/const/graph";
import type { LinkKind } from "src/interfaces/links";
import type { ShowNodeOptions } from "src/interfaces/settings";
import type BreadcrumbsPlugin from "src/main";
import { Links } from "src/utils/links";
import { toNodeStringifyOptions, type EdgeAttribute } from "src/graph/utils";
import { TraversalOptions, create_edge_sorter, type RecTraversalData } from "wasm/pkg/breadcrumbs_graph_wasm";

export namespace ListIndex {
	export type Options = {
		// TODO: merge_fields: boolean;
		indent: string;
		fields: string[];
		// TODO
		max_depth?: number;
		link_kind: LinkKind;
		edge_sort_id: EdgeSortId;
		field_group_labels: string[];
		show_attributes: EdgeAttribute[];
		show_node_options: ShowNodeOptions;
	};

	export const DEFAULT_OPTIONS: Options = {
		fields: [],
		indent: "\\t",
		link_kind: "wiki",
		show_attributes: [],
		field_group_labels: [],
		edge_sort_id: {
			order: 1,
			field: "basename",
		},
		show_node_options: {
			ext: false,
			alias: true,
			folder: false,
		},
	};

	export const edge_tree_to_list_index = (
		plugin: BreadcrumbsPlugin,
		tree: RecTraversalData[],
		options: Pick<
			Options,
			"link_kind" | "indent" | "show_node_options" | "show_attributes"
		>,
	) => {
		let index = "";
		const real_indent = options.indent.replace(/\\t/g, "\t");

		tree.forEach(({ children, depth, edge }) => {
			const display = edge.stringify_target(
				plugin.graph, 
				toNodeStringifyOptions(plugin, options.show_node_options)
			);

			const link = Links.ify(edge.target_path(plugin.graph), display, {
				link_kind: options.link_kind,
			});

			const attr = edge.get_attribute_label(plugin.graph, options.show_attributes);

			index += real_indent.repeat(depth) + `- ${link}${attr}\n`;

			index += edge_tree_to_list_index(plugin, children, options);
		});

		return index;
	};

	export const build = (
		plugin: BreadcrumbsPlugin,
		start_node: string,
		options: Options,
	) => {
		const traversal_options = new TraversalOptions(
			[start_node],
			options.fields,
			options.max_depth ?? 100,
			false,
		);

		const traversal_result = plugin.graph.rec_traverse(traversal_options);
		const edge_sorter = create_edge_sorter(options.edge_sort_id.field, options.edge_sort_id.order === -1);
		traversal_result.sort(plugin.graph, edge_sorter);

		return edge_tree_to_list_index(
			plugin,
			traversal_result.data,
			options,
		);
	}
}
