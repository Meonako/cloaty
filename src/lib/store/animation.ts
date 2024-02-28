import { writable } from 'svelte/store';
import type { AnimationInfo } from '$lib/types';
import { fade } from 'svelte/transition';

export const ANIMATION = writable<AnimationInfo>({
	animation: fade,
	animationParams: {
		in: {
			duration: 200,
			delay: 200
		},
		out: {
			duration: 200
		}
	}
});
