import { error as kitError } from '@sveltejs/kit';
import { ApiError, apiRequest, getAuthToken } from '$lib/server/api';

export const load = async ({ url, cookies, fetch }) => {
	const tournamentId = url.searchParams.get('tournament_id');
	if (!tournamentId) {
		return {
			leaderboard: null,
			message: 'Оберіть турнір, щоб переглянути таблицю лідерів.'
		};
	}

	const token = getAuthToken(cookies);

	try {
		const leaderboard = await apiRequest(fetch, `/api/tournaments/${tournamentId}/leaderboard`, {
			token
		});

		return { leaderboard, message: null };
	} catch (error) {
		if (error instanceof ApiError) {
			throw kitError(error.status, error.message);
		}

		throw kitError(500, 'Backend недоступний');
	}
};
