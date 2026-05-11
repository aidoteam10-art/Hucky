import { apiRequest, getAuthToken } from '$lib/server/api';

const allowedStatuses = new Set(['draft', 'registration', 'running', 'finished']);

export const load = async ({ url, cookies, fetch }) => {
	const status = url.searchParams.get('status') || 'all';
	const search = url.searchParams.get('search') || '';
	const page = Math.max(Number(url.searchParams.get('page') || 1), 1);
	const perPage = 6;

	const params = new URLSearchParams({
		page: String(page),
		per_page: String(perPage)
	});

	if (allowedStatuses.has(status)) params.set('status', status);
	if (search.trim()) params.set('search', search.trim());
	const token = getAuthToken(cookies);

	try {
		const [tournaments, profile] = await Promise.all([
			apiRequest(fetch, `/api/tournaments?${params.toString()}`),
			token ? apiRequest(fetch, '/api/users/me', { token }) : Promise.resolve(null)
		]);

		return {
			tournaments,
			filters: { status, search },
			profile
		};
	} catch (error) {
		return {
			tournaments: {
				items: [],
				page,
				per_page: perPage,
				total: 0,
				total_pages: 0
			},
			filters: { status, search },
			profile: null,
			error: error.message || 'Backend недоступний'
		};
	}
};
