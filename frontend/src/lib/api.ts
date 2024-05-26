export function encodeQueryString(params: any): string {
	return Object.entries(params)
		.filter(([_key, value]) => value != undefined && value != null)
		.map((kv) => kv.map(encodeURIComponent).join('='))
		.join('&');
}
