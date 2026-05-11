import { error as kitError, fail, redirect } from '@sveltejs/kit';
import { ApiError, apiRequest, getAuthToken } from '$lib/server/api';

export const load = async ({ params, cookies, fetch }) => {
	const token = getAuthToken(cookies);

	try {
		const [tournament, rounds] = await Promise.all([
			apiRequest(fetch, `/api/tournaments/${params.tournament_id}`, { token }),
			apiRequest(fetch, `/api/tournaments/${params.tournament_id}/rounds`, { token })
		]);

		return {
			tournament,
			rounds: rounds.items,
			isAuthenticated: Boolean(token)
		};
	} catch (error) {
		if (error instanceof ApiError) {
			throw kitError(error.status, error.message);
		}

		throw kitError(500, 'Backend недоступний');
	}
};

export const actions = {
	changeTournamentStatus: async ({ request, params, cookies, fetch }) => {
		const token = getAuthToken(cookies);
		if (!token) return fail(401, { message: 'Потрібно увійти' });

		const formData = await request.formData();
		const status = String(formData.get('status') || '');

		try {
			await apiRequest(fetch, `/api/tournaments/${params.tournament_id}/status`, {
				method: 'PATCH',
				token,
				body: { status }
			});
		} catch (error) {
			return actionError(error);
		}

		throw redirect(303, `/tournaments/${params.tournament_id}`);
	},

	changeRoundStatus: async ({ request, params, cookies, fetch }) => {
		const token = getAuthToken(cookies);
		if (!token) return fail(401, { message: 'Потрібно увійти' });

		const formData = await request.formData();
		const roundId = String(formData.get('round_id') || '');
		const status = String(formData.get('status') || '');

		try {
			await apiRequest(fetch, `/api/rounds/${roundId}/status`, {
				method: 'PATCH',
				token,
				body: { status }
			});
		} catch (error) {
			return actionError(error);
		}

		throw redirect(303, `/tournaments/${params.tournament_id}`);
	},

	createRound: async ({ request, params, cookies, fetch }) => {
		const token = getAuthToken(cookies);
		if (!token) return fail(401, { message: 'Потрібно увійти' });

		const formData = await request.formData();
		const payload = buildRoundPayload(formData);

		if (payload.error) {
			return fail(400, { message: payload.error });
		}

		try {
			await apiRequest(fetch, `/api/tournaments/${params.tournament_id}/rounds`, {
				method: 'POST',
				token,
				body: payload.data
			});
		} catch (error) {
			return actionError(error);
		}

		throw redirect(303, `/tournaments/${params.tournament_id}`);
	}
};

function buildRoundPayload(formData) {
	const title = readString(formData, 'title');
	const task_description = readString(formData, 'task_description');
	const technology_requirements = readString(formData, 'technology_requirements');
	const starts_at = readDate(formData, 'starts_at');
	const deadline_at = readDate(formData, 'deadline_at');
	const position = readOptionalNumber(formData, 'position');
	const must_have = formData
		.getAll('must_have')
		.map((value) => String(value).trim())
		.filter(Boolean);

	if (!title || !task_description || !starts_at || !deadline_at) {
		return { error: 'Заповніть назву, опис і дати раунду' };
	}

	return {
		data: {
			title,
			task_description,
			technology_requirements: technology_requirements || null,
			must_have,
			starts_at,
			deadline_at,
			position
		}
	};
}

function actionError(error) {
	if (error instanceof ApiError) {
		return fail(error.status, { message: error.message });
	}

	return fail(500, { message: 'Backend недоступний' });
}

function readString(formData, name) {
	return String(formData.get(name) || '').trim();
}

function readDate(formData, name) {
	const value = readString(formData, name);
	if (!value) return null;
	const date = new Date(value);
	if (Number.isNaN(date.getTime())) return null;
	return date.toISOString();
}

function readOptionalNumber(formData, name) {
	const value = readString(formData, name);
	if (!value) return null;
	const number = Number(value);
	return Number.isFinite(number) ? number : null;
}
