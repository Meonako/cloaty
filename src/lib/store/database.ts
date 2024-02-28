import { writable, type Updater, type Writable } from 'svelte/store';
import type { Database } from '$lib/types';

export const DATABASE_STORE = initStore('database', {});

function initStore(
	key: string,
	defaultValue: Record<string, Database>
): Writable<Record<string, Database>> {
	const valueFromStorage = localStorage.getItem(key);
	const value: Record<string, Database> = valueFromStorage
		? JSON.parse(valueFromStorage)
		: defaultValue;

	const object_values = Object.values(value);

	if (object_values.length > 0) {
		for (const v of object_values) {
			v.connect = false;
		}
	}

	const { subscribe, set, update } = writable(value);

	return {
		subscribe,
		update: (f: Updater<Record<string, Database>>) => {
			update(f);

			for (const v of Object.values(value)) {
				if (typeof v.connect !== 'boolean') continue;

				delete v.connect;
			}

			localStorage.setItem(key, JSON.stringify(value));
		},
		set: (v: Record<string, Database>) => {
			set(v);
			localStorage.setItem(key, JSON.stringify(v));
		}
	};
}
