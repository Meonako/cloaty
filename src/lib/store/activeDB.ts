import { writable } from 'svelte/store';
import type { Active } from '$lib/types';

export const ACTIVE_DB = writable<Active | null>(null);
