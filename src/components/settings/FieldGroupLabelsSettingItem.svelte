<script lang="ts">
	import type { EdgeFieldGroup } from "src/interfaces/settings";
	import FieldGroupLabelsSelector from "../selector/FieldGroupLabelsSelector.svelte";
	import SettingItem from "./SettingItem.svelte";

	interface Props {
		name?: string;
		description?: string;
		field_group_labels: string[];
		edge_field_groups: EdgeFieldGroup[];
		select_cb?: (value: string[]) => void;
	}

	let {
		name = "Field Groups",
		description = "Select the field groups to use for this traversal.",
		field_group_labels = $bindable(),
		edge_field_groups,
		select_cb = () => {},
	}: Props = $props();

	$effect(() => {
		if (field_group_labels) {
			select_cb(field_group_labels);
		}
	});
</script>

<SettingItem {name} {description}>
	<FieldGroupLabelsSelector {edge_field_groups} bind:field_group_labels />
</SettingItem>
