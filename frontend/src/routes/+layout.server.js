import { ApiError, apiRequest, getAuthToken } from '$lib/server/api';

export const load = async ({ cookies, fetch, locals }) => {
	const token = getAuthToken(cookies);
	let profile = null;

	if (token) {
		try {
			profile = await apiRequest(fetch, '/api/users/me', { token });
		} catch (error) {
			if (error instanceof ApiError && error.status === 401) {
				cookies.delete('jwt', { path: '/' });
			}
		}
	}

	return {
		user: locals.user,
		profile
	};
};
