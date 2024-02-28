<script lang="ts">
	export let containerClass: string = '';

	import { SETTINGS } from '$lib/store/settings';
	import { ANIMATION } from '$lib/store/animation';

	let animation = $ANIMATION?.animation;
	$: if ($SETTINGS.animation.enabled) {
		animation = $ANIMATION?.animation;
	}
</script>

{#if $SETTINGS.animation.enabled}
	<div
		class={containerClass}
		in:animation|global={$ANIMATION.animationParams.in}
		out:animation|global={$ANIMATION.animationParams.out}
	>
		<slot />
	</div>
{:else}
	<slot />
{/if}
