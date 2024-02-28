<script lang="ts">
	import { goto } from '$app/navigation';
	import { invoke } from '@tauri-apps/api';
	import { ACTIVE_DB } from '$lib/store/activeDB';

	import TransitionMaker from '$lib/components/transitionMaker.svelte';
	import { Button } from '$lib/components/ui/button';

	async function getTables(): Promise<string[]> {
		if (!$ACTIVE_DB!.type) throw new Error('No database type specified');
		if (!$ACTIVE_DB!.db) throw new Error('No database specified');
		if (!$ACTIVE_DB!.schema) throw new Error('No schema specified');

		return await invoke('get_tables', {
			databaseType: $ACTIVE_DB!.type,
			connName: $ACTIVE_DB!.db,
			schema: $ACTIVE_DB!.schema
		});
	}

	let promise = getTables();

	$: if ($ACTIVE_DB) {
		promise = getTables();
	}
</script>

{#await promise}
	<!-- <TransitionMaker> -->
		<h1 class="text-center text-3xl">Loading...</h1>
	<!-- </TransitionMaker> -->
{:then tables}
	<TransitionMaker containerClass="max-h-full w-full overflow-y-auto">
		<div class="flex w-full">
			<Button
				variant="link"
				class="ml-auto"
				on:click={() => {
					promise = getTables();
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
