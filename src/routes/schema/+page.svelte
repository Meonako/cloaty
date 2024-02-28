<script lang="ts">
	import { goto } from '$app/navigation';
	import { invoke } from '@tauri-apps/api';
	import { ACTIVE_DB } from '$lib/store/activeDB';
	import type { Active } from '$lib/types';

	import TransitionMaker from '$lib/components/transitionMaker.svelte';
	import { Button } from '$lib/components/ui/button';

	async function getTables(db: Active): Promise<string[]> {
		if (!db.type) throw new Error('No database type specified');
		if (!db.db) throw new Error('No database specified');
		if (!db.schema) throw new Error('No schema specified');

		return await invoke('get_tables', {
			databaseType: db.type,
			connName: db.db,
			schema: db.schema
		});
	}

	let promise: Promise<string[]>;

	$: if ($ACTIVE_DB && !$ACTIVE_DB!.table) {
		promise = getTables($ACTIVE_DB!);
	}
</script>

{#await promise}
	<h1 class="flex h-full w-full items-center justify-center text-center text-3xl">Loading...</h1>
{:then tables}
	<TransitionMaker containerClass="max-h-full w-full overflow-y-auto">
		<div class="flex w-full">
			<Button
				variant="link"
				class="ml-auto"
				on:click={() => {
					// @ts-ignore
					promise = getTables($ACTIVE_DB);
				}}
			>
				Refresh
			</Button>
		</div>
		<div class="grid w-full grid-cols-3 gap-1">
			{#each tables as table}
				<Button
					variant="outline"
					on:click={() => {
						// @ts-ignore
						$ACTIVE_DB.table = table;

						goto(`/table`);
					}}
				>
					{table}
				</Button>
			{/each}
		</div>
	</TransitionMaker>
{:catch error}
	<h2 class="text-2xl">{error}</h2>
{/await}
