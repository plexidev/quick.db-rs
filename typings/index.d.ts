declare module 'quick.db' {
	export function set(key: string, value: string | number | object): undefined;

	export function get<T>(key: string): T | undefined;
	export function get(key: string): any;

	export function remove(key: string): undefined;
}
