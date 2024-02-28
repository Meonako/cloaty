import { writable, type Updater, type Writable } from 'svelte/store';
import { fade, fly, scale } from 'svelte/transition';
import { ANIMATION } from './animation';
import { invoke } from '@tauri-apps/api';
import type { Settings, AnimationInfo, AnimationParams, ValidAnimation } from '$lib/types';

function initStore(value: Settings): Writable<Settings> {
	const { subscribe, set, update } = writable(value);

	return {
		subscribe,
		update: (f: Updater<Settings>) => {
			update(f);

			if (!value.animation.enabled) return;

			let anim = getAnimationInfo(value);
			ANIMATION.set(anim);
		},
		set: (v: Settings) => {
			set(v);

			if (!v.animation.enabled) return;

			let anim = getAnimationInfo(v);
			ANIMATION.set(anim);
		}
	};
}

export const SETTINGS = initStore({
	database: {
		items_per_page: 50,
		fetch_limit: 100
	},

	animation: {
		enabled: true,
		duration: 200,
		type: 'scale',
		params: {
			fly: {
				in_x: 100,
				out_x: -100,
				in_y: 0,
				out_y: 0
			}
		}
	}
});

function getAnimationInfo(settings: Settings): AnimationInfo {
	let animation: ValidAnimation;
	let animationParams: AnimationParams = {
		in: {
			duration: settings.animation.duration,
			delay: settings.animation.duration
		},
		out: {
			duration: settings.animation.duration
		}
	};

	switch (settings.animation.type) {
		case 'fly':
			animation = fly;
			// @ts-ignore
			animationParams.in.x = settings.animation.params.fly.in_x;
			// @ts-ignore
			animationParams.in.y = settings.animation.params.fly.in_y;
			// @ts-ignore
			animationParams.out.x = settings.animation.params.fly.out_x;
			// @ts-ignore
			animationParams.out.y = settings.animation.params.fly.out_y;
			break;
		case 'fade':
			animation = fade;
			break;
		case 'scale':
			animation = scale;
			break;
		default:
			animation = fade;
			break;
	}

	return {
		animation,
		animationParams
	};
}

invoke('get_settings').then((v) => {
	SETTINGS.set(v as Settings);
});
