<script lang="ts">
	import { type EdgeSortId } from "src/const/graph";
	import EdgeSortIdSelector from "../selector/EdgeSortIdSelector.svelte";
	import SettingItem from "./SettingItem.svelte";

	interface Props {
		edge_sort_id: EdgeSortId;
		select_cb?: (value: EdgeSortId) => void;
	}

	let { edge_sort_id = $bindable(), select_cb = () => {} }: Props = $props();

	$effect(() => {
		if (edge_sort_id) {
			select_cb(edge_sort_id);
		}
	});
</script>

<SettingItem
	name="Edge Sort"
	description="Select the sorting method for the edges in the graph."
>
	<EdgeSortIdSelector bind:edge_sort_id />
</SettingItem>
