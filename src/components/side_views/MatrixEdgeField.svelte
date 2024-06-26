<script lang="ts">
	import type { EdgeAttribute } from "src/graph/MyMultiGraph";
	import type { EdgeField } from "src/interfaces/settings";
	import type BreadcrumbsPlugin from "src/main";
	import { NodeStringifyOptions, type EdgeStruct } from "wasm/pkg/breadcrumbs_graph_wasm";
	import EdgeLink from "../EdgeLink.svelte";
	import ChevronOpener from "../button/ChevronOpener.svelte";
	import TreeItemFlair from "../obsidian/TreeItemFlair.svelte";

	export let open: boolean;
	export let field: EdgeField;
	export let edges: EdgeStruct[];
	export let plugin: BreadcrumbsPlugin;
	// NOTE: These are available on settings, but they're modified in the parent component,
	// 	so rather pass them in to receive updates
	export let show_attributes: EdgeAttribute[];

	let { show_node_options } = plugin.settings.views.side.matrix;

	const { dendron_note } = plugin.settings.explicit_edge_sources;

	let node_stringify_options = new NodeStringifyOptions(
		show_node_options.ext,
		show_node_options.folder,
		show_node_options.alias,
		dendron_note.enabled && dendron_note.display_trimmed
			? dendron_note.delimiter
			: undefined,
	);

</script>

<details
	class="BC-matrix-view-field BC-matrix-view-field-{field.label} tree-item"
	bind:open
>
	<!-- svelte-ignore a11y-click-events-have-key-events -->
	<!-- svelte-ignore a11y-no-static-element-interactions -->
	<summary class="tree-item-self is-clickable mod-collapsible text-lg">
		<div class="tree-item-icon collapse-icon">
			<ChevronOpener {open} />
		</div>

		<div class="tree-item-inner">
			<span class="tree-item-inner-text">
				{field.label}
			</span>
		</div>

		<div class="tree-item-flair-outer">
			<span class="tree-item-flair font-mono text-lg">
				{edges.length}
			</span>
		</div>
	</summary>

	<div class="tree-item-children flex flex-col">
		{#key edges}
			{#each edges as edge}
				<div class="tree-item">
					<div class="tree-item-self is-clickable">
						<div class="tree-item-inner flex grow">
							<EdgeLink
								{edge}
								{plugin}
								{node_stringify_options}
								cls="grow tree-item-inner-text"
							/>
						</div>

						<TreeItemFlair
							cls="font-mono"
							label={edge.explicit ? "x" : "i"}
							aria_label={edge.get_attribute_label(
								show_attributes,
							)}
						/>
					</div>
				</div>
			{/each}
		{/key}
	</div>
</details>
