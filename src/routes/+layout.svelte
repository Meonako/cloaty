<script lang="ts">
	export let data;
	import '../app.pcss';

	import { fly } from 'svelte/transition';
	import { invoke } from '@tauri-apps/api';

	import { DATABASE_STORE } from '$lib/store/database';
	import { SCHEMA_LIST_STORE } from '$lib/store/schemaList';
	import { SETTINGS } from '$lib/store/settings';
	import { ACTIVE_DB } from '$lib/store/activeDB';
	import { goto } from '$app/navigation';
	import type { NewDatabase, EditingDatabse, Database, Status } from '$lib/types';

	import { Pencil, Trash, Loader2, Plus, Check, Plug, Unplug, Cog, X } from 'lucide-svelte';

	import * as Accordion from '$lib/components/ui/accordion';
	import * as Dialog from '$lib/components/ui/dialog';
	import * as ContextMenu from '$lib/components/ui/context-menu';
	import * as Resizable from '$lib/components/ui/resizable';
	import * as Select from '$lib/components/ui/select';
	import { Button, buttonVariants } from '$lib/components/ui/button';
	import { Input } from '$lib/components/ui/input';
	import { NumberInput } from '$lib/components/ui/numberinput';
	import { Label } from '$lib/components/ui/label';
	import Required from '$lib/components/required.svelte';

	import TransitionMaker from '$lib/components/transitionMaker.svelte';
	import { dbDefaultDatabase, dbDefaultPort, dbDefaultUser, dbLabel } from '$lib/utils';

	async function getSettings() {
		SETTINGS.set(await invoke('get_settings'));
	}

	getSettings();

	let addingNewDb = false;
	let edittingDb = false;
	let edittingDbInfo: EditingDatabse;
	let dbConnecting = false;
	let testConnectionStatus: Status = 'idle';

	$: if (testConnectionStatus != 'idle' && testConnectionStatus != 'processing') {
		setTimeout(() => {
			testConnectionStatus = 'idle';
		}, 5000);
	}

	const databases = [
		{ value: 'pg', name: dbLabel('pg') },
		{ value: 'mysql', name: dbLabel('mysql') }
	];

	let newDB: NewDatabase = {
		type: 'pg',
		name: 'New Database',
		host: '127.0.0.1',
		port: '5432',
		database: 'postgres',
		username: 'postgres',
		password: ''
	};

	function addNewDB() {
		addingNewDb = true;
		if (!newDB.host || !newDB.name || !newDB.port || !newDB.database) {
			return;
		}

		let port: number;
		try {
			port = parseInt(newDB.port);
		} catch {
			return;
		}

		DATABASE_STORE.update((dbs) => {
			dbs[newDB.name] = {
				type: newDB.type,
				host: newDB.host,
				dbname: newDB.database,
				username: newDB.username,
				password: newDB.password,
				port,
				connect: false
			};

			return dbs;
		});

		addingNewDb = false;
	}

	function editDB() {
		edittingDb = true;

		DATABASE_STORE.update((dbs) => {
			if (edittingDbInfo.name != edittingDbInfo.ogName) {
				delete dbs[edittingDbInfo.ogName];
			}

			dbs[edittingDbInfo.name] = {
				type: edittingDbInfo.db.type,
				host: edittingDbInfo.db.host,
				dbname: edittingDbInfo.db.dbname,
				username: edittingDbInfo.db.username,
				password: edittingDbInfo.db.password,
				port: edittingDbInfo.db.port,
				connect: false
			};

			return dbs;
		});

		edittingDb = false;
	}

	async function testConnection(
		type: 'pg' | 'mysql',
		name: string,
		info: {
			host: string;
			port: number;
			database: string;
			user?: string;
			password?: string;
		}
	) {
		testConnectionStatus = 'processing';
		try {
			await invoke('test_connection', {
				databaseType: type,
				connName: name,
				info
			});
			testConnectionStatus = 'pass';
		} catch {
			testConnectionStatus = 'failed';
		}
	}
</script>

<Resizable.PaneGroup direction="horizontal" class="w-full rounded-lg border">
	<Resizable.Pane defaultSize={75} class="flex flex-col">
		<Accordion.Root>
			{#each Object.entries($DATABASE_STORE) as [name, db]}
				<ContextMenu.Root>
					<ContextMenu.Trigger>
						<Accordion.Item value={name}>
							<Accordion.Trigger>
								<span class="mx-auto">{name}</span>
							</Accordion.Trigger>
							{#if !db.connect}
								<Accordion.Content>
									<div class="flex flex-col">
										<Button
											disabled={dbConnecting}
											variant="default"
											class="mx-auto flex w-3/4 bg-green-400 transition-all{dbConnecting
												? ' w-full'
												: ''}"
											on:click={async () => {
												dbConnecting = true;
												let schemasList = [];
												try {
													const args = {
														databaseType: db.type,
														connName: name,
														info: {
															host: db.host,
															port: db.port,
															database: db.dbname,
															user: db.username,
															password: db.password
														}
													};
													// @ts-ignore
													schemasList = await invoke('connect_db', args);
												} catch (e) {
													dbConnecting = false;
													if (e == 'Already connected') {
														db.connect = true;
													} else {
														console.error(e);
													}
													return;
												}

												$SCHEMA_LIST_STORE[name] = schemasList;

												db.connect = true;
												dbConnecting = false;
											}}
										>
											{#if dbConnecting}
												<Loader2 class="mr-2 size-4 animate-spin" />
												Connecting...
											{:else}
												<Plug class="mr-2 size-4" />
												Connect
											{/if}
										</Button>
									</div>
								</Accordion.Content>
							{:else}
								<Accordion.Content>
									<div class="flex flex-col">
										{#if $SCHEMA_LIST_STORE[name] && $SCHEMA_LIST_STORE[name].length > 0}
											{#each $SCHEMA_LIST_STORE[name] as schema}
												<Button
													variant="outline"
													class="mx-auto w-3/4{$ACTIVE_DB?.db == name &&
													$ACTIVE_DB.schema == schema
														? ' bg-cyan-500 text-black'
														: ''}"
													on:click={() => {
														ACTIVE_DB.set({
															type: db.type,
															db: name,
															schema,
															table: undefined
														});

														goto(`/schema`);
													}}
												>
													{schema}
												</Button>
											{/each}
										{/if}
										<Button
											disabled={dbConnecting}
											variant="destructive"
											class="mx-auto flex w-3/4 transition-all{dbConnecting
												? ' w-full'
												: ''}"
											on:click={async () => {
												dbConnecting = true;

												try {
													await invoke('disconnect_db', {
														databaseType: db.type,
														connName: name
													});

													ACTIVE_DB.set(null);
												} catch (e) {
													console.error(e);
													dbConnecting = false;
													return;
												}

												db.connect = false;
												dbConnecting = false;

												goto('/');
											}}
										>
											{#if dbConnecting}
												<Loader2 class="mr-2 size-4 animate-spin" />
												Disconnecting...
											{:else}
												<Unplug class="mr-2 size-4" />
												Disconnect
											{/if}
										</Button>
									</div>
								</Accordion.Content>
							{/if}
						</Accordion.Item>
					</ContextMenu.Trigger>
					<ContextMenu.Content>
						<ContextMenu.Item
							class="bg-red-500 text-center text-white transition-colors hover:bg-red-400"
							on:click={() => {
								edittingDbInfo = {
									ogName: name,
									name,
									db
								};

								edittingDb = true;
							}}
						>
							<Pencil class="mr-2 size-5" />
							Edit
						</ContextMenu.Item>
						<ContextMenu.Item
							class="bg-red-500 text-center text-white transition-colors hover:bg-red-400"
							on:click={() => {
								DATABASE_STORE.update((dbs) => {
									delete dbs[name];
									return dbs;
								});
							}}
						>
							<Trash class="mr-2 size-5" />
							Delete
						</ContextMenu.Item>
					</ContextMenu.Content>
				</ContextMenu.Root>
			{/each}
			<Dialog.Root bind:open={addingNewDb}>
				<Dialog.Trigger class={`${buttonVariants({ variant: 'outline' })} w-full`}>
					<Plus class="mr-2 size-4" />
					Add DB
				</Dialog.Trigger>
				<Dialog.Content class="sm:max-w-[425px]">
					<Dialog.Header>
						<Dialog.Title>Add Database</Dialog.Title>
						<Dialog.Description>
							Add a new database for easy connection
						</Dialog.Description>
					</Dialog.Header>
					<div class="grid gap-4 py-4">
						<Select.Root
							selected={databases[0]}
							onSelectedChange={(v) => {
								if (!v) return;

								let oldDbType = newDB.type;
								let oldPort = newDB.port;
								let oldUser = newDB.username;
								let oldDb = newDB.database;

								// @ts-ignore
								newDB.type = v.value;

								if (oldPort == dbDefaultPort(oldDbType)) {
									newDB.port = dbDefaultPort(newDB.type);
								}

								if (oldUser == dbDefaultUser(oldDbType)) {
									newDB.username = dbDefaultUser(newDB.type);
								}

								if (oldDb == dbDefaultDatabase(oldDbType)) {
									newDB.database = dbDefaultDatabase(newDB.type);
								}
							}}
						>
							<Select.Trigger>
								<Select.Value placeholder="Select Database" />
							</Select.Trigger>
							<Select.Content>
								{#each databases as db}
									<Select.Item value={db.value}>{db.name}</Select.Item>
								{/each}
							</Select.Content>
						</Select.Root>
						<div class="grid grid-cols-4 items-center gap-4">
							<Label class="text-right">Name<Required /></Label>
							<Input
								type="text"
								id="name"
								bind:value={newDB.name}
								class="col-span-3"
								required
							/>
						</div>
						<div class="grid grid-cols-4 items-center gap-4">
							<Label class="text-right">Host<Required /></Label>
							<Input
								type="text"
								id="host"
								bind:value={newDB.host}
								class="col-span-3"
								required
							/>
						</div>
						<div class="grid grid-cols-4 items-center gap-4">
							<Label class="text-right">Port<Required /></Label>
							<NumberInput
								id="port"
								bind:value={newDB.port}
								class="col-span-3"
								required
							/>
						</div>
						<div class="grid grid-cols-4 items-center gap-4">
							<Label class="text-right">Database<Required /></Label>
							<Input
								type="text"
								id="database"
								bind:value={newDB.database}
								class="col-span-3"
								required
							/>
						</div>
						<div class="grid grid-cols-4 items-center gap-4">
							<Label class="text-right">Username</Label>
							<Input
								type="text"
								id="username"
								bind:value={newDB.username}
								class="col-span-3"
							/>
						</div>
						<div class="grid grid-cols-4 items-center gap-4">
							<Label class="text-right">Password</Label>
							<Input
								id="password"
								bind:value={newDB.password}
								type="password"
								class="col-span-3"
							/>
						</div>
					</div>
					<Dialog.Footer>
						<Button
							disabled={testConnectionStatus == 'processing'}
							type="submit"
							on:click={() => {
								testConnection(newDB.type, newDB.name, {
									host: newDB.host,
									port: +newDB.port,
									database: newDB.database,
									user: newDB.username,
									password: newDB.password
								});
							}}
						>
								Test Connection
						</Button>
						<Button type="submit" on:click={addNewDB}>
							Add
						</Button>
					</Dialog.Footer>
				</Dialog.Content>
			</Dialog.Root>
		</Accordion.Root>
		<Button class="mt-auto w-full" href="/settings" variant="outline">
			<Cog class="mr-2 size-5" />
			Settings
		</Button>
	</Resizable.Pane>
	<Resizable.Handle withHandle />
	<Resizable.Pane defaultSize={300}>
		{#key data.url}
			<TransitionMaker containerClass="h-full max-h-full w-full overflow-y-auto p-2">
				<slot />
			</TransitionMaker>
		{/key}
		<!-- <div
			class="h-full max-h-full w-full overflow-y-auto p-2"
			in:fly={{ x: -100, delay: 200 }}
			out:fly={{ x: -100, duration: 200 }}
		></div> -->
	</Resizable.Pane>
</Resizable.PaneGroup>

<Dialog.Root bind:open={edittingDb}>
	<Dialog.Content class="sm:max-w-[425px]">
		<Dialog.Header>
			<Dialog.Title>Edit DB: {edittingDbInfo?.name}</Dialog.Title>
			<Dialog.Description>Change database connection info</Dialog.Description>
		</Dialog.Header>
		<div class="grid gap-4 py-4">
			<Select.Root
				selected={{
					value: edittingDbInfo?.db.type,
					label: dbLabel(edittingDbInfo?.db.type)
				}}
			>
				<Select.Trigger>
					<Select.Value
						placeholder="Select Database (Current: {edittingDbInfo?.db.type})"
					/>
				</Select.Trigger>
				<Select.Content>
					{#each databases as db}
						<Select.Item value={db.value} label={db.value}>{db.name}</Select.Item>
					{/each}
				</Select.Content>
			</Select.Root>
			<div class="grid grid-cols-4 items-center gap-4">
				<Label class="text-right">Name<Required /></Label>
				<Input id="name" bind:value={edittingDbInfo.name} class="col-span-3" required />
			</div>
			<div class="grid grid-cols-4 items-center gap-4">
				<Label class="text-right">Host<Required /></Label>
				<Input id="host" bind:value={edittingDbInfo.db.host} class="col-span-3" required />
			</div>
			<div class="grid grid-cols-4 items-center gap-4">
				<Label class="text-right">Port<Required /></Label>
				<Input id="port" bind:value={edittingDbInfo.db.port} class="col-span-3" required />
			</div>
			<div class="grid grid-cols-4 items-center gap-4">
				<Label class="text-right">Database<Required /></Label>
				<Input
					id="database"
					bind:value={edittingDbInfo.db.dbname}
					class="col-span-3"
					required
				/>
			</div>
			<div class="grid grid-cols-4 items-center gap-4">
				<Label class="text-right">Username</Label>
				<Input id="username" bind:value={edittingDbInfo.db.username} class="col-span-3" />
			</div>
			<div class="grid grid-cols-4 items-center gap-4">
				<Label class="text-right">Password</Label>
				<Input
					id="password"
					type="password"
					bind:value={edittingDbInfo.db.password}
					class="col-span-3"
				/>
			</div>
		</div>
		<Dialog.Footer>
			{#if testConnectionStatus != 'idle'}
				{#if testConnectionStatus == 'processing'}
					<TransitionMaker containerClass="flex items-center text-yellow-500">
						<Loader2 class="mr-2 inline-block size-4 animate-spin" />
						Testing connection...
					</TransitionMaker>
				{:else if testConnectionStatus == 'pass'}
					<TransitionMaker containerClass="flex items-center text-green-500">
						<Check class="mr-2 inline-block size-4" /> Connection successful
					</TransitionMaker>
				{:else if testConnectionStatus == 'failed'}
					<TransitionMaker containerClass="flex items-center text-red-500">
						<X class="mr-2 inline-block size-4" /> Could not connect to the database
					</TransitionMaker>
				{/if}
			{/if}
			<Button
				disabled={testConnectionStatus == 'processing'}
				type="submit"
				on:click={() => {
					testConnection(edittingDbInfo.db.type, edittingDbInfo.name, {
						host: edittingDbInfo.db.host,
						port: edittingDbInfo.db.port,
						database: edittingDbInfo.db.dbname,
						user: edittingDbInfo.db.username,
						password: edittingDbInfo.db.password
					});
				}}
			>
				Test Connection
			</Button>
			<Button type="submit" on:click={editDB}>Save</Button>
		</Dialog.Footer>
	</Dialog.Content>
</Dialog.Root>
