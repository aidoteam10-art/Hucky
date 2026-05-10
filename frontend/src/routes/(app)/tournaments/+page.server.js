import { apiRequest } from '$lib/server/api';

const allowedStatuses = new Set(['draft', 'registration', 'running', 'finished']);

export const load = async ({ url, fetch }) => {
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

	try {
		const tournaments = await apiRequest(fetch, `/api/tournaments?${params.toString()}`);

		return {
			tournaments,
			filters: { status, search }
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
			error: error.message || 'Backend недоступний'
		};
	}
};
