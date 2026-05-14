import { apiRequest, getAuthToken } from '$lib/server/api';

const allowedStatuses = new Set(['draft', 'registration', 'running', 'finished']);

export const load = async ({ url, cookies, fetch }) => {
	let status = url.searchParams.get('status') || 'all';
	const search = url.searchParams.get('search') || '';
	const page = Math.max(Number(url.searchParams.get('page') || 1), 1);
	const perPage = 6;
	const token = getAuthToken(cookies);
	let profile = null;

	try {
		profile = token ? await apiRequest(fetch, '/api/users/me', { token }) : null;
	} catch {
		profile = null;
	}

	if (status === 'draft' && profile?.role !== 'organiser') {
		status = 'all';
	}

	const params = new URLSearchParams({
		page: String(page),
		per_page: String(perPage)
	});

	if (allowedStatuses.has(status)) params.set('status', status);
	if (search.trim()) params.set('search', search.trim());

	try {
		const tournaments = await apiRequest(fetch, `/api/tournaments?${params.toString()}`, { token });

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
