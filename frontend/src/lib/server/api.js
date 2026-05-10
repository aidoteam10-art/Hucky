import { env } from '$env/dynamic/private';

const API_BASE_URL = env.BACKEND_URL || 'http://127.0.0.1:8080';

export class ApiError extends Error {
	constructor(status, message, body) {
		super(message);
		this.status = status;
		this.body = body;
	}
}

export function getAuthToken(cookies) {
	return cookies.get('jwt');
}

export async function apiRequest(fetch, path, options = {}) {
	const headers = new Headers(options.headers);

	if (options.body !== undefined) {
		headers.set('Content-Type', 'application/json');
	}

	if (options.token) {
		headers.set('Authorization', `Bearer ${options.token}`);
	}

	const response = await fetch(`${API_BASE_URL}${path}`, {
		method: options.method || 'GET',
		headers,
		body: options.body === undefined ? undefined : JSON.stringify(options.body)
	});

	const text = await response.text();
	let body = null;

	if (text) {
		try {
			body = JSON.parse(text);
		} catch {
			body = text;
		}
	}

	if (!response.ok) {
		throw new ApiError(response.status, extractErrorMessage(body), body);
	}

	return body;
}

export function extractErrorMessage(body) {
	if (!body) return 'Backend request failed';
	if (typeof body === 'string') return body;
	if (typeof body.error === 'string') return body.error;
	if (typeof body.message === 'string') return body.message;
	return 'Backend request failed';
}
