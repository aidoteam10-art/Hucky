import { error as kitError, fail, redirect } from '@sveltejs/kit';
import { ApiError, apiRequest, getAuthToken } from '$lib/server/api';

export const load = async ({ params, cookies, fetch }) => {
	const token = getAuthToken(cookies);
	if (!token) {
		throw redirect(303, '/login');
	}

	try {
		const round = await apiRequest(fetch, `/api/rounds/${params.round_id}`, { token });
		let submission = null;

		try {
			submission = await apiRequest(fetch, `/api/rounds/${params.round_id}/submission`, { token });
		} catch (error) {
			if (!(error instanceof ApiError && error.status === 404)) {
				throw error;
			}
		}

		return { round, submission };
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
		const values = {
			github_url: readString(formData, 'github_url'),
			video_demo_url: readString(formData, 'video_demo_url'),
			live_demo_url: readString(formData, 'live_demo_url'),
			description: readString(formData, 'description')
		};

		const validationError = validateSubmission(values);
		if (validationError) {
			return fail(400, { message: validationError, values });
		}

		try {
			await apiRequest(fetch, `/api/rounds/${params.round_id}/submission`, {
				method: 'PUT',
				token,
				body: {
					github_url: values.github_url,
					video_demo_url: values.video_demo_url,
					live_demo_url: values.live_demo_url || null,
					description: values.description || null
				}
			});
		} catch (error) {
			return actionError(error, values);
		}

		throw redirect(
			303,
			`/tournaments/${params.tournament_id}/rounds/${params.round_id}/submission/success`
		);
	}
};

function validateSubmission(values) {
	if (!values.github_url) return "Репозиторій обов'язковий";
	if (!isHttpUrl(values.github_url)) return 'GitHub Repository має бути валідним URL';
	if (!values.video_demo_url) return "Відео демо обов'язкове";
	if (!isHttpUrl(values.video_demo_url)) return 'Video Demo має бути валідним URL';
	if (values.live_demo_url && !isHttpUrl(values.live_demo_url)) return 'Live Demo має бути валідним URL';
	if (values.description.length > 2000) return 'Опис не може перевищувати 2000 символів';
	return null;
}

function isHttpUrl(value) {
	try {
		const url = new URL(value);
		return url.protocol === 'http:' || url.protocol === 'https:';
	} catch {
		return false;
	}
}

function readString(formData, name) {
	return String(formData.get(name) || '').trim();
}

function actionError(error, values) {
	if (error instanceof ApiError) {
		return fail(error.status, { message: error.message, values });
	}

	return fail(500, { message: 'Backend недоступний', values });
}
