<script lang="ts">
	import { SETTINGS } from '$lib/store/settings';
	import { ANIMATION } from '$lib/store/animation';
	import { invoke } from '@tauri-apps/api';

	import { ArrowLeft } from 'lucide-svelte';
	import { Button } from '$lib/components/ui/button';
	import { Input } from '$lib/components/ui/input';
	import { Checkbox } from '$lib/components/ui/checkbox';
	import { Label } from '$lib/components/ui/label';
	import { Separator } from '$lib/components/ui/separator';
	import * as Select from '$lib/components/ui/select';
	import { NumberInput } from '$lib/components/ui/numberinput';

	$: invoke('save_settings', {
		settings: JSON.stringify($SETTINGS)
	});

	let animation = $ANIMATION?.animation;
	$: if ($SETTINGS.animation.enabled) {
		animation = $ANIMATION?.animation;
	}
</script>

<div class="flex h-full w-full flex-col">
	<div class="relative w-full">
		<Button variant="link" href="/" class="absolute left-0 top-0">
			<ArrowLeft class="mr-2 size-5" />
			Back
		</Button>
		<h1 class="text-center text-3xl">Settings</h1>
	</div>

	<div class="mx-auto mt-4 flex w-3/4 flex-row justify-evenly">
		<div>
			<h2 class="text-center text-xl">Database</h2>

			<Separator class="my-2" />

			<div class="flex w-full max-w-sm flex-col gap-1.5">
				<Label for="itemsPerPage">Display Items per Page</Label>
				<NumberInput
					id="itemsPerPage"
					bind:value={$SETTINGS.database.items_per_page}
				/>
				<p class="text-sm text-muted-foreground">
					How many rows to display per page (in table)
				</p>
			</div>

			<Separator class="my-2" />

			<div class="flex w-full max-w-sm flex-col gap-1.5">
				<Label for="fetchLimit">Fetch Limit</Label>
				<NumberInput id="fetchLimit" bind:value={$SETTINGS.database.fetch_limit} />
				<p class="text-sm text-muted-foreground">
					Limit how many rows to fetch (The <code
						class="bg-gray-900 text-muted-foreground">LIMIT</code
					> clause)
				</p>
			</div>
		</div>
		<div class="max-w-[50%]">
			<h2 class="text-center text-xl">Appearance</h2>

			<Separator class="my-2" />

			<div class="flex items-center justify-center space-x-2">
				<div
					class="flex cursor-pointer items-center p-2 transition-colors hover:bg-secondary"
				>
					<Checkbox
						id="animationEnable"
						bind:checked={$SETTINGS.animation.enabled}
						aria-labelledby="animationEnable"
					/>
					<Label
						id="animationEnable"
						for="animationEnable"
						class="cursor-pointer pl-2 text-sm font-medium leading-none peer-disabled:cursor-not-allowed peer-disabled:opacity-70"
					>
						Animation
					</Label>
				</div>
			</div>

			{#if $SETTINGS.animation.enabled}
				<div
					in:animation={$ANIMATION.animationParams.in}
					out:animation={$ANIMATION.animationParams.out}
				>
					<Separator class="my-2" />

					<div class="flex w-full max-w-sm flex-col gap-1.5">
						<Label for="animationDuration">Duration <strong>(ms)</strong></Label>
						<NumberInput
							id="animationDuration"
							bind:value={$SETTINGS.animation.duration}
						/>
						<p class="text-sm text-muted-foreground">
							Duration of the animation ({$SETTINGS.animation.type})
						</p>
					</div>

					<Separator class="my-2" />

					<Select.Root
						selected={{
							label: $SETTINGS.animation.type,
							value: $SETTINGS.animation.type
						}}
						onSelectedChange={(selected) => {
							if (!selected) return;
							$SETTINGS.animation.type = selected.value;
						}}
					>
						<Select.Trigger class="w-full">
							<Select.Value placeholder="Animation Type" />
						</Select.Trigger>
						<Select.Content>
							<Select.Group>
								{#each ['fly', 'fade', 'scale'] as animationType}
									<Select.Item value={animationType} label={animationType}>
										{animationType}
									</Select.Item>
								{/each}
							</Select.Group>
						</Select.Content>
						<Select.Input name="favoriteFruit" />
					</Select.Root>

					<div class="my-2" />

					<Separator class="my-2" />

					{#if $SETTINGS.animation.type === 'fly'}
						<div
							class="grid grid-cols-2"
							in:animation={$ANIMATION.animationParams.in}
							out:animation={$ANIMATION.animationParams.out}
						>
							<div class="mb-4 flex w-full max-w-sm flex-col gap-1.5">
								<Label for="animationInX">Fly In X</Label>
								<NumberInput
									id="animationInX"
									bind:value={$SETTINGS.animation.params.fly.in_x}
								/>
								<p class="pr-1 text-sm text-muted-foreground">
									<strong>[X Axis]</strong> Direction and how far to move when entering
								</p>
							</div>

							<div class="flex w-full max-w-sm flex-col gap-1.5">
								<Label for="animationOutX">Fly Out X</Label>
								<NumberInput
									id="animationOutX"
									bind:value={$SETTINGS.animation.params.fly.out_x}
								/>
								<p class="pr-1 text-sm text-muted-foreground">
									<strong>[X Axis]</strong> Direction and how far to move when exiting
								</p>
							</div>

							<div class="mb-4 flex w-full max-w-sm flex-col gap-1.5">
								<Label for="animationInY">Fly In Y</Label>
								<NumberInput
									id="animationInY"
									bind:value={$SETTINGS.animation.params.fly.in_y}
								/>
								<p class="pr-1 text-sm text-muted-foreground">
									<strong>[Y Axis]</strong> Direction and how far to move when entering
								</p>
							</div>

							<div class="flex w-full max-w-sm flex-col gap-1.5">
								<Label for="animationOutY">Fly Out Y</Label>
								<NumberInput
									id="animationOutY"
									bind:value={$SETTINGS.animation.params.fly.out_y}
								/>
								<p class="pr-1 text-sm text-muted-foreground">
									<strong>[Y Axis]</strong> Direction and how far to move when exiting
								</p>
							</div>
						</div>

						<Button
							variant="secondary"
							class="w-full"
							on:click={() => {
								const temp1 = $SETTINGS.animation.params.fly.in_x;
								const temp2 = $SETTINGS.animation.params.fly.out_x;

								$SETTINGS.animation.params.fly.in_x =
									$SETTINGS.animation.params.fly.in_y;
								$SETTINGS.animation.params.fly.out_x =
									$SETTINGS.animation.params.fly.out_y;

								$SETTINGS.animation.params.fly.in_y = temp1;
								$SETTINGS.animation.params.fly.out_y = temp2;
							}}
						>
							Swap X and Y
						</Button>
					{/if}
				</div>
			{/if}
		</div>
	</div>
</div>
