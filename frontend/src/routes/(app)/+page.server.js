import { ApiError, apiRequest, getAuthToken } from '$lib/server/api';

export const load = async ({ cookies, fetch }) => {
	const token = getAuthToken(cookies);
	if (!token) {
		return { profile: null };
	}

	try {
		const profile = await apiRequest(fetch, '/api/users/me', { token });
		return { profile };
	} catch (error) {
		if (error instanceof ApiError && error.status === 401) {
			cookies.delete('jwt', { path: '/' });
		}

		return { profile: null };
	}
};
