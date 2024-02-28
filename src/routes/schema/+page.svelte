<script lang="ts">
	import { goto } from '$app/navigation';
	import { invoke } from '@tauri-apps/api';
	import { ACTIVE_DB } from '$lib/store/activeDB';
	import type { Active } from '$lib/types';

	import TransitionMaker from '$lib/components/transitionMaker.svelte';
	import { Button } from '$lib/components/ui/button';
	import { Input } from '$lib/components/ui/input';

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

	let filter = '';
	let promise: Promise<string[]>;

	$: if ($ACTIVE_DB && !$ACTIVE_DB!.table) {
		promise = getTables($ACTIVE_DB!);
	}
</script>

{#await promise}
	<h1 class="flex h-full w-full items-center justify-center text-center text-3xl">Loading...</h1>
{:then tables}
	<TransitionMaker containerClass="max-h-full w-full overflow-y-auto">
		<div class="relative mb-1 flex w-full flex-row">
			<Input class="mx-auto w-1/2" bind:value={filter} placeholder="Search" />
			<Button
				variant="link"
				class="absolute right-0 ml-auto"
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
					{#if filter && table.toLowerCase().includes(filter.toLowerCase())}
						<TransitionMaker containerClass="w-full">
							<Button
								variant="outline"
								class="w-full"
								on:click={() => {
									// @ts-ignore
									$ACTIVE_DB.table = table;

									goto(`/table`);
								}}
							>
								{table}
							</Button>
						</TransitionMaker>
					{:else if filter == ''}
						<TransitionMaker containerClass="w-full">
							<Button
								variant="outline"
								class="w-full"
								on:click={() => {
									// @ts-ignore
									$ACTIVE_DB.table = table;

									goto(`/table`);
								}}
							>
								{table}
							</Button>
						</TransitionMaker>
					{/if}
				{/each}
		</div>
	</TransitionMaker>
{:catch error}
	<h2 class="text-2xl">{error}</h2>
{/await}
