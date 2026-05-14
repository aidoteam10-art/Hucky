import { error as kitError, fail, redirect } from '@sveltejs/kit';
import { ApiError, apiRequest, getAuthToken } from '$lib/server/api';

export const load = async ({ params, cookies, fetch }) => {
	const token = getAuthToken(cookies);
	if (!token) {
		throw redirect(303, '/login');
	}

	try {
		const assignment = await apiRequest(fetch, `/api/jury/assignments/${params.task_id}`, { token });
		return { assignment };
	} catch (error) {
		if (error instanceof ApiError && error.status === 401) {
			throw redirect(303, '/login');
		}
		if (error instanceof ApiError) {
			throw kitError(error.status, error.message);
		}

		throw kitError(500, 'Backend недоступний');
	}
};

export const actions = {
	default: async ({ request, params, cookies, fetch }) => {
		const token = getAuthToken(cookies);
		if (!token) return fail(401, { message: 'Потрібно увійти' });

		const formData = await request.formData();
		const criterionIds = formData.getAll('criterion_id').map((value) => String(value));
		const scores = formData.getAll('score').map((value) => Number(value));
		const comment = String(formData.get('comment') || '').trim();

		if (criterionIds.length === 0 || criterionIds.length !== scores.length) {
			return fail(400, { message: 'Потрібно оцінити всі критерії' });
		}

		if (scores.some((score) => !Number.isInteger(score) || score < 0 || score > 100)) {
			return fail(400, { message: 'Оцінки мають бути цілими числами від 0 до 100' });
		}

		try {
			await apiRequest(fetch, `/api/jury/assignments/${params.task_id}/evaluation`, {
				method: 'POST',
				token,
				body: {
					scores: criterionIds.map((criterion_id, index) => ({
						criterion_id,
						score: scores[index]
					})),
					comment: comment || null
				}
			});
		} catch (error) {
			if (error instanceof ApiError) {
				return fail(error.status, { message: error.message });
			}
			return fail(500, { message: 'Backend недоступний' });
		}

		throw redirect(303, `/tournaments/results/${params.task_id}`);
	}
};
