<script lang="ts">
	import type { ICodeblock } from "src/codeblocks/schema";
	import { ListIndex } from "src/commands/list_index";
	import type { BreadcrumbsError } from "src/interfaces/graph";
	import { log } from "src/logger";
	import type BreadcrumbsPlugin from "src/main";
	import { active_file_store } from "src/stores/active_file";
	import { Timer } from "src/utils/timer";
	import { onMount } from "svelte";
	import {
		FlatTraversalResult,
		NoteGraphError,
		TraversalOptions,
		TraversalPostprocessOptions,
		create_edge_sorter,
	} from "wasm/pkg/breadcrumbs_graph_wasm";
	import NestedEdgeList from "../NestedEdgeList.svelte";
	import CopyToClipboardButton from "../button/CopyToClipboardButton.svelte";
	import CodeblockErrors from "./CodeblockErrors.svelte";

	interface Props {
		plugin: BreadcrumbsPlugin;
		options: ICodeblock["Options"];
		errors: BreadcrumbsError[];
		file_path: string;
	}

	let { plugin, options, errors, file_path }: Props = $props();

	const sort = create_edge_sorter(
		options.sort.field,
		options.sort.order === -1,
	);
	const { show_node_options } = plugin.settings.views.codeblocks;

	const DEFAULT_MAX_DEPTH = 5;

	let data: FlatTraversalResult | undefined = $state(undefined);
	let error: string | undefined = $state(undefined);

	let active_file = $derived($active_file_store);

	export function update() {
		const max_depth =
			options.depth[1] === Infinity
				? DEFAULT_MAX_DEPTH
				: (options.depth[1] ?? DEFAULT_MAX_DEPTH);

		const source_path =
			options["start-note"] || file_path || active_file?.path || "";

		if (!plugin.graph.has_node(source_path)) {
			data = undefined;
			error = "The file does not exist in the graph.";
			return;
		}

		const traversal_options = new TraversalOptions(
			[source_path],
			options.fields,
			max_depth,
			100, // max nodes to traverse
			!options["merge-fields"],
		);

		const postprocess_options = new TraversalPostprocessOptions(
			sort,
			options.flat,
		);

		try {
			data = plugin.graph.rec_traverse_and_process(
				traversal_options,
				postprocess_options,
			);

			error = undefined;
		} catch (e) {
			log.error("Error updating codeblock tree", e);

			data = undefined;
			if (e instanceof NoteGraphError) {
				error = e.message;
			} else {
				error =
					"An error occurred while updating the codeblock tree. Check the console for more information (Ctrl + Shift + I).";
			}
		}
	}

	onMount(() => {
		const timer = new Timer();

		update();

		log.debug(timer.elapsedMessage("CodeblockTree initial traversal"));
	});
</script>

<div class="BC-codeblock-tree">
	<CodeblockErrors {plugin} {errors} />

	{#if options.title}
		<h3 class="BC-codeblock-tree-title">
			{options.title}
		</h3>
	{/if}

	{#if data && !data.is_empty()}
		<div class="BC-codeblock-tree-items relative">
			<div class="absolute bottom-2 right-2 flex">
				<CopyToClipboardButton
					cls="clickable-icon nav-action-button"
					text={() =>
						ListIndex.edge_tree_to_list_index(
							plugin.graph,
							data,
							plugin.settings,
							{
								...plugin.settings.commands.list_index
									.default_options,
								show_attributes:
									options["show-attributes"] ?? [],
							},
						)}
				/>
			</div>

			<!-- NOTE: Padded so that the flair doesn't interfere with the floating buttons -->
			<div class="pr-10">
				<NestedEdgeList
					{plugin}
					{show_node_options}
					{data}
					items={data.entry_nodes}
					open_signal={!options.collapse}
					show_attributes={options["show-attributes"]}
				/>
			</div>
		</div>
	{:else if error}
		<p class="search-empty-state">{error}</p>
	{:else}
		<!-- TODO(HELP-MSG) -->
		<p class="search-empty-state">No paths found.</p>
	{/if}
</div>
