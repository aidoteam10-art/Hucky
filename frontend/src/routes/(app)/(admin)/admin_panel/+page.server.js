import { error as kitError, fail, redirect } from '@sveltejs/kit';
import { ApiError, apiRequest, getAuthToken } from '$lib/server/api';

const allowedRoles = new Set(['participant', 'organiser', 'jury', 'admin']);

export const load = async ({ url, cookies, fetch }) => {
	const token = getAuthToken(cookies);
	if (!token) throw redirect(303, '/login');

	const role = url.searchParams.get('role') || 'all';
	const search = url.searchParams.get('search') || '';
	const page = Math.max(Number(url.searchParams.get('page') || 1), 1);
	const perPage = 50;
	const params = new URLSearchParams({
		page: String(page),
		per_page: String(perPage)
	});

	if (allowedRoles.has(role)) params.set('role', role);
	if (search.trim()) params.set('search', search.trim());

	try {
		const profile = await apiRequest(fetch, '/api/users/me', { token });
		if (profile.role !== 'admin') {
			throw kitError(403, 'Only admin can access this page');
		}

		const users = await apiRequest(fetch, `/api/admin/users?${params.toString()}`, { token });

		return {
			profile,
			users,
			filters: { role, search }
		};
	} catch (error) {
		if (error instanceof ApiError && error.status === 401) {
			throw redirect(303, '/login');
		}
		if (error instanceof ApiError) {
			throw kitError(error.status, error.message);
		}
		if (error?.status) {
			throw error;
		}

		throw kitError(500, 'Backend недоступний');
	}
};

export const actions = {
	updateRole: async ({ request, cookies, fetch }) => {
		const token = getAuthToken(cookies);
		if (!token) return fail(401, { message: 'Потрібно увійти' });

		const formData = await request.formData();
		const userId = String(formData.get('user_id') || '').trim();
		const role = String(formData.get('role') || '').trim();

		if (!userId) return fail(400, { message: 'User id is required' });
		if (!allowedRoles.has(role)) return fail(400, { message: 'Invalid role' });

		try {
			await apiRequest(fetch, `/api/admin/users/${userId}/role`, {
				method: 'PATCH',
				token,
				body: { role }
			});
		} catch (error) {
			if (error instanceof ApiError) {
				return fail(error.status, { message: error.message });
			}

			return fail(500, { message: 'Backend недоступний' });
		}

		throw redirect(303, '/admin_panel');
	}
};
