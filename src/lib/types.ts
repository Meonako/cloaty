import type { fade, fly, scale } from 'svelte/transition';

export type Schema = Record<string, string[]>;

export type SupportDatabase = 'pg' | 'mysql';

export type Status = 'pass' | 'failed' | 'processing' | 'idle';

export type NewDatabase = {
	type: SupportDatabase;
	name: string;
	host: string;
	port: string;
	database: string;
	username: string;
	password: string;
};

export type EditingDatabse = {
	ogName: string;
	name: string;
	db: Database;
};

export type Database = {
	type: SupportDatabase;
	host: string;
	dbname: string;
	username: string;
	password: string;
	port: number;
	connect?: boolean;
};

export type Active = {
	type: SupportDatabase;
	db: string;
	schema: string;
	table?: string;
};

export type Settings = {
	database: {
		items_per_page: number;
		fetch_limit: number;
	};

	animation: {
		enabled: boolean;
		duration: number;
		type: 'fade' | 'fly' | 'scale';
		params: {
			fly: {
				in_x: number;
				out_x: number;
				in_y: number;
				out_y: number;
			};
		};
	};
};

export type AnimationParams =
	| {
			in: {
				duration: number;
				delay: number;
				x: number;
				y: number;
			};
			out: {
				duration: number;
				x: number;
				y: number;
			};
	  }
	| {
			in: {
				duration: number;
				delay: number;
			};
			out: {
				duration: number;
			};
	  };

export type ValidAnimation = typeof fade | typeof scale | typeof fly;

export type AnimationInfo = {
	animation: ValidAnimation;
	animationParams: AnimationParams;
};
