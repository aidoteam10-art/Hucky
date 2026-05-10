import { error as kitError } from '@sveltejs/kit';
import { ApiError, apiRequest, getAuthToken } from '$lib/server/api';

export const load = async ({ params, cookies, fetch }) => {
	const token = getAuthToken(cookies);

	try {
		const round = await apiRequest(fetch, `/api/rounds/${params.round_id}`, { token });
		return { round };
	} catch (error) {
		if (error instanceof ApiError) {
			throw kitError(error.status, error.message);
		}

		throw kitError(500, 'Backend недоступний');
	}
};
