<script lang="ts">
	export let data: Record<string, any[]>[];
	export let columnHeaders: string[];
	export let page = 0;

	import { createTable, Render, Subscribe, createRender } from 'svelte-headless-table';
	import {
		addPagination,
		addSortBy,
		addTableFilter,
		addHiddenColumns,
		addSelectedRows
	} from 'svelte-headless-table/plugins';
	import { readable } from 'svelte/store';
	import { SETTINGS } from '$lib/store/settings';

	import * as Table from '$lib/components/ui/table';
	import { Button } from '$lib/components/ui/button';
	import { Input } from '$lib/components/ui/input';
	import { ArrowUpDown, ChevronDown } from 'lucide-svelte';
	import * as DropdownMenu from '$lib/components/ui/dropdown-menu';
	import TransitionMaker from '$lib/components/transitionMaker.svelte';
	import { NumberInput } from '$lib/components/ui/numberinput';

	const table = createTable(readable(data), {
		page: addPagination({
			initialPageSize: $SETTINGS.database.items_per_page
		}),
		sort: addSortBy({ disableMultiSort: true }),
		filter: addTableFilter({
			fn: ({ filterValue, value }) => value.includes(filterValue)
		}),
		hide: addHiddenColumns(),
		select: addSelectedRows()
	});

	const columns = table.createColumns(
		columnHeaders.map((col) => table.column({ accessor: col, header: col }))
	);

	const { headerRows, pageRows, tableAttrs, tableBodyAttrs, pluginStates, flatColumns, rows } =
		table.createViewModel(columns);

	const { pageIndex, hasNextPage, hasPreviousPage, pageCount, pageSize } = pluginStates.page;
	const { filterValue } = pluginStates.filter;
	const { hiddenColumnIds } = pluginStates.hide;
	const { selectedDataIds } = pluginStates.select;

	const ids = flatColumns.map((col) => col.id);
	let hideForId = Object.fromEntries(ids.map((id) => [id, true]));

	$: $hiddenColumnIds = Object.entries(hideForId)
		.filter(([, hide]) => !hide)
		.map(([id]) => id);

	$: if ($pageSize && $pageSize !== $SETTINGS.database.items_per_page) {
		$SETTINGS.database.items_per_page = $pageSize;
	}
</script>

<TransitionMaker>
	<div class="flex items-center py-4">
		<Input class="max-w-sm" placeholder="Search..." type="text" bind:value={$filterValue} />
		<NumberInput class="ml-auto max-w-sm" type="text" bind:value={$pageSize} />
		<DropdownMenu.Root>
			<DropdownMenu.Trigger asChild let:builder>
				<Button variant="outline" builders={[builder]}>
					Columns <ChevronDown class="ml-2 h-4 w-4" />
				</Button>
			</DropdownMenu.Trigger>
			<DropdownMenu.Content>
				{#each flatColumns as col}
					{#if columnHeaders.includes(col.id)}
						<DropdownMenu.CheckboxItem bind:checked={hideForId[col.id]}>
							{col.header}
						</DropdownMenu.CheckboxItem>
					{/if}
				{/each}
			</DropdownMenu.Content>
		</DropdownMenu.Root>
	</div>
	<div class="rounded-md border">
		<Table.Root {...$tableAttrs}>
			<Table.Header>
				{#each $headerRows as headerRow}
					<Subscribe rowAttrs={headerRow.attrs()}>
						<Table.Row>
							{#each headerRow.cells as cell (cell.id)}
								<Subscribe
									attrs={cell.attrs()}
									let:attrs
									props={cell.props()}
									let:props
								>
									<Table.Head {...attrs} class="[&:has([role=checkbox])]:pl-3">
										<Button variant="ghost" on:click={props.sort.toggle}>
											<Render of={cell.render()} />
											<ArrowUpDown class={'ml-2 h-4 w-4'} />
										</Button>
									</Table.Head>
								</Subscribe>
							{/each}
						</Table.Row>
					</Subscribe>
				{/each}
			</Table.Header>
			<Table.Body {...$tableBodyAttrs}>
				{#each $pageRows as row (row.id)}
					<Subscribe rowAttrs={row.attrs()} let:rowAttrs>
						<Table.Row
							{...rowAttrs}
							data-state={$selectedDataIds[row.id] && 'selected'}
						>
							{#each row.cells as cell (cell.id)}
								<Subscribe attrs={cell.attrs()} let:attrs>
									<Table.Cell {...attrs}>
										<Render of={cell.render()} />
									</Table.Cell>
								</Subscribe>
							{/each}
						</Table.Row>
					</Subscribe>
				{/each}
			</Table.Body>
		</Table.Root>
	</div>
	<div class="flex flex-row w-full items-center py-4">
		<div class="text-sm text-muted-foreground">
			{($pageIndex + 1) * $pageSize} of {$rows.length}
		</div>

		{#if $rows.length >= Object.keys(data).length}
			<Button variant="outline" size="sm" class="mx-auto text-sm text-muted-foreground" on:click={() => (page += 1)}>
				Fetch Next {$SETTINGS.database.fetch_limit} items
			</Button>
		{/if}

		<div class="flex flex-row ml-auto">
			<Button
				variant="secondary"
				size="sm"
				on:click={() => ($pageIndex = $pageIndex - 1)}
				disabled={!$hasPreviousPage}
			>
				Previous
			</Button>

			{#if $pageIndex != 0}
				<Button variant="outline" size="sm" on:click={() => ($pageIndex = 0)}>1</Button>
			{/if}

			{#if $pageIndex > 2 && $pageIndex < $pageCount - 1}
				<!-- <Input
				type="number"
				placeholder="Jump to"
				class="w-[10%] max-w-xs"
				on:input={(e) => {
					// @ts-ignore
					if (!e.target || !e.target.value) return;
					// @ts-ignore
					$pageIndex = parseInt(e.target.value) - 1;
				}}
			/> -->
				<div>&nbsp; ... &nbsp;</div>
			{/if}

			{#if $hasPreviousPage && $pageIndex > 1}
				<Button
					variant="outline"
					size="sm"
					on:click={() => ($pageIndex = $pageIndex - 1)}
					disabled={!$hasPreviousPage}
				>
					{$pageIndex}
				</Button>
			{/if}

			<Button variant="outline" size="sm" disabled={true}>
				{$pageIndex + 1}
			</Button>

			{#if $hasNextPage && $pageIndex + 2 < $pageCount}
				{@const nextPage = $pageIndex + 1}
				<Button
					variant="outline"
					size="sm"
					disabled={!$hasNextPage}
					on:click={() => ($pageIndex = nextPage)}
				>
					{nextPage + 1}
				</Button>
			{/if}

			{#if $pageIndex + 2 < $pageCount - 1}
				{@const nextPage = $pageIndex + 2}
				<Button
					variant="outline"
					size="sm"
					disabled={!$hasNextPage}
					on:click={() => ($pageIndex = nextPage)}
				>
					{nextPage + 1}
				</Button>
			{/if}

			{#if $pageIndex + 3 < $pageCount - 1}
				<!-- <Input
				type="number"
				placeholder="Jump to"
				class="w-[10%] max-w-xs"
				on:input={(e) => {
					// @ts-ignore
					if (!e.target || !e.target.value) return;
					// @ts-ignore
					$pageIndex = parseInt(e.target.value) - 1;
				}}
			/> -->
				<div>&nbsp; ... &nbsp;</div>
			{/if}

			{#if $pageCount != 1 && $pageIndex < $pageCount - 1}
				<Button
					variant="outline"
					size="sm"
					disabled={!$hasNextPage}
					on:click={() => ($pageIndex = $pageCount - 1)}
				>
					{$pageCount}
				</Button>
			{/if}

			<Button
				variant="outline"
				size="sm"
				disabled={!$hasNextPage}
				on:click={() => ($pageIndex = $pageIndex + 1)}
			>
				Next
			</Button>
		</div>
	</div>
</TransitionMaker>
