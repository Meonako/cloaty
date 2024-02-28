<script lang="ts">
	import { invoke } from '@tauri-apps/api';
	import { ACTIVE_DB } from '$lib/store/activeDB';
	import { SETTINGS } from '$lib/store/settings';

	import DataTable from './data-table.svelte';

	let page = 0;

	async function getData(): Promise<[string[], Record<string, any[]>[]]> {
		if (!$ACTIVE_DB!.type) throw new Error('No database type specified');
		if (!$ACTIVE_DB!.db) throw new Error('No database specified');
		if (!$ACTIVE_DB!.schema) throw new Error('No schema specified');
		if (!$ACTIVE_DB!.table) throw new Error('No table specified');

		return await invoke('get_data', {
			databaseType: $ACTIVE_DB!.type,
			connName: $ACTIVE_DB!.db,
			schema: $ACTIVE_DB!.schema,
			table: $ACTIVE_DB!.table,
			limit: $SETTINGS.database.fetch_limit,
			page,
		});
	}

	let promise = getData();

	$: if (page > 0) {
		promise = getData();
	}
</script>

{#await promise}
	<h1 class="flex h-full w-full items-center justify-center text-center text-3xl">Loading...</h1>
{:then data}
	<DataTable bind:page data={data[1]} columnHeaders={data[0]} />
{:catch error}
	<h2 class="text-2xl">{error}</h2>
{/await}
