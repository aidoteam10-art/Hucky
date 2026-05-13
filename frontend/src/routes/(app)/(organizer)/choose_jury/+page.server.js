import { error as kitError, fail, redirect } from '@sveltejs/kit';
import { ApiError, apiRequest, getAuthToken } from '$lib/server/api';

export const load = async ({ cookies, fetch, url }) => {
	const token = getAuthToken(cookies);
	if (!token) {
		throw redirect(303, '/login');
	}

	try {
		const profile = await apiRequest(fetch, '/api/users/me', { token });
		if (profile.role !== 'organiser') {
			throw kitError(403, 'Only organisers can choose jury');
		}

		const management = await apiRequest(fetch, '/api/organizer/jury-management', { token });

		return {
			profile,
			tournaments: management.tournaments || [],
			juries: management.juries || [],
			selectedTournamentId: url.searchParams.get('tournament_id') || ''
		};
	} catch (error) {
		if (error instanceof ApiError) {
			throw kitError(error.status, error.message);
		}

		throw error;
	}
};

export const actions = {
	addJury: async ({ request, cookies, fetch }) => {
		const token = getAuthToken(cookies);
		if (!token) return fail(401, { message: 'Потрібно увійти' });

		const formData = await request.formData();
		const tournamentId = readString(formData, 'tournament_id');
		const userId = readString(formData, 'user_id');

		if (!tournamentId) return fail(400, { message: 'Tournament id is required' });
		if (!userId) return fail(400, { message: 'Оберіть журі' });

		try {
			await apiRequest(fetch, `/api/tournaments/${tournamentId}/jury`, {
				method: 'POST',
				token,
				body: { user_id: userId }
			});
		} catch (error) {
			return actionError(error);
		}

		throw redirect(303, `/choose_jury?tournament_id=${tournamentId}`);
	},

	removeJury: async ({ request, cookies, fetch }) => {
		const token = getAuthToken(cookies);
		if (!token) return fail(401, { message: 'Потрібно увійти' });

		const formData = await request.formData();
		const tournamentId = readString(formData, 'tournament_id');
		const userId = readString(formData, 'user_id');

		if (!tournamentId) return fail(400, { message: 'Tournament id is required' });
		if (!userId) return fail(400, { message: 'Jury user id is required' });

		try {
			await apiRequest(fetch, `/api/tournaments/${tournamentId}/jury/${userId}`, {
				method: 'DELETE',
				token
			});
		} catch (error) {
			return actionError(error);
		}

		throw redirect(303, `/choose_jury?tournament_id=${tournamentId}`);
	}
};

function actionError(error) {
	if (error instanceof ApiError) {
		return fail(error.status, { message: error.message });
	}

	return fail(500, { message: 'Backend недоступний' });
}

function readString(formData, name) {
	return String(formData.get(name) || '').trim();
}
