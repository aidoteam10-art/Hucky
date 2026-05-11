import { error as kitError, redirect } from '@sveltejs/kit';
import { apiRequest, getAuthToken } from '$lib/server/api';

export const load = async ({ cookies, fetch }) => {
	const token = getAuthToken(cookies);
	if (!token) {
		throw redirect(303, '/login');
	}

	const profile = await apiRequest(fetch, '/api/users/me', { token });
	if (profile.role !== 'organiser') {
		throw kitError(403, 'Only organisers can choose jury');
	}

	return { profile };
};
