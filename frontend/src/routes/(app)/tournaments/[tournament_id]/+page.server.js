import { error as kitError, fail, redirect } from '@sveltejs/kit';
import { ApiError, apiRequest, getAuthToken } from '$lib/server/api';

export const load = async ({ params, cookies, fetch }) => {
	const token = getAuthToken(cookies);

	try {
		const [tournament, rounds, myTeams, profile] = await Promise.all([
			apiRequest(fetch, `/api/tournaments/${params.tournament_id}`, { token }),
			apiRequest(fetch, `/api/tournaments/${params.tournament_id}/rounds`, { token }),
			token
				? apiRequest(fetch, '/api/me/teams', { token })
				: Promise.resolve({ items: [] }),
			token ? apiRequest(fetch, '/api/users/me', { token }) : Promise.resolve(null)
		]);
		const jury = token ? await loadJury(fetch, token, params.tournament_id) : { items: [], canManage: false };
		const canParticipate = profile?.role !== 'organiser';
		const userTeam =
			canParticipate
				? myTeams.items?.find(
						(item) => item.tournament.id === params.tournament_id && item.status === 'accepted'
					) || null
				: null;
		const canRegisterTeam = Boolean(token) && canParticipate;

		return {
			tournament,
			rounds: rounds.items,
			jury: jury.items || [],
			canManage: jury.canManage,
			canRegisterTeam,
			userTeam,
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

	deleteTournament: async ({ params, cookies, fetch }) => {
		const token = getAuthToken(cookies);
		if (!token) return fail(401, { message: 'Потрібно увійти' });

		try {
			await apiRequest(fetch, `/api/tournaments/${params.tournament_id}`, {
				method: 'DELETE',
				token
			});
		} catch (error) {
			return actionError(error);
		}

		throw redirect(303, '/tournaments');
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
	},

	lockSubmissions: async ({ request, params, cookies, fetch }) => {
		const token = getAuthToken(cookies);
		if (!token) return fail(401, { message: 'Потрібно увійти' });

		const formData = await request.formData();
		const roundId = String(formData.get('round_id') || '').trim();
		if (!roundId) return fail(400, { message: 'Round id is required' });

		try {
			await apiRequest(fetch, `/api/rounds/${roundId}/submissions/lock`, {
				method: 'POST',
				token
			});
		} catch (error) {
			return actionError(error);
		}

		throw redirect(303, `/tournaments/${params.tournament_id}`);
	},

	generateAssignments: async ({ request, params, cookies, fetch }) => {
		const token = getAuthToken(cookies);
		if (!token) return fail(401, { message: 'Потрібно увійти' });

		const formData = await request.formData();
		const roundId = String(formData.get('round_id') || '').trim();
		const reviews_per_submission = readOptionalNumber(formData, 'reviews_per_submission') ?? 3;
		const max_assignments_per_jury = readOptionalNumber(formData, 'max_assignments_per_jury') ?? 5;

		if (!roundId) return fail(400, { message: 'Round id is required' });

		try {
			await apiRequest(fetch, `/api/rounds/${roundId}/assignments/generate`, {
				method: 'POST',
				token,
				body: { reviews_per_submission, max_assignments_per_jury }
			});
		} catch (error) {
			return actionError(error);
		}

		throw redirect(303, `/tournaments/${params.tournament_id}`);
	},

};

async function loadJury(fetch, token, tournamentId) {
	try {
		const response = await apiRequest(fetch, `/api/tournaments/${tournamentId}/jury`, { token });
		return { ...response, canManage: true };
	} catch (error) {
		if (error instanceof ApiError && (error.status === 403 || error.status === 404)) {
			return { items: [], canManage: false };
		}
		throw error;
	}
}

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
