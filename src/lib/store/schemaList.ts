import type { Schema } from '$lib/types';
import { writable } from 'svelte/store';

export const SCHEMA_LIST_STORE = writable<Schema>({});
