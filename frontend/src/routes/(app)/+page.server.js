import { ApiError, apiRequest, getAuthToken } from '$lib/server/api';

export const load = async ({ cookies, fetch }) => {
	const token = getAuthToken(cookies);
	let profile = null;
	let tournaments = {
		items: [],
		page: 1,
		per_page: 8,
		total: 0,
		total_pages: 0
	};
	let tournamentsError = null;

	try {
		tournaments = await apiRequest(
			fetch,
			'/api/tournaments?status=registration&page=1&per_page=8'
		);
	} catch (error) {
		tournamentsError = error.message || 'Не вдалося завантажити турніри';
	}

	if (token) {
		try {
			profile = await apiRequest(fetch, '/api/users/me', { token });
		} catch (error) {
			if (error instanceof ApiError && error.status === 401) {
				cookies.delete('jwt', { path: '/' });
			}
		}
	}

	return { profile, tournaments, tournamentsError };
};
