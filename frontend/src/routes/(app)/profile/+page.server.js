import { error as kitError, fail, redirect } from '@sveltejs/kit';
import { ApiError, apiRequest, getAuthToken } from '$lib/server/api';

export const load = async ({ cookies, fetch }) => {
	const token = getAuthToken(cookies);
	if (!token) {
		throw redirect(303, '/login');
	}

	try {
		const profile = await apiRequest(fetch, '/api/users/me', { token });
		const shouldLoadManagedTournaments = profile.role === 'organiser';
		const shouldLoadParticipantData = profile.role !== 'organiser';
		const [teams, invitations, juryAssignments, createdTournaments] = await Promise.all([
			shouldLoadParticipantData
				? apiRequest(fetch, '/api/me/teams', { token })
				: Promise.resolve({ items: [] }),
			shouldLoadParticipantData
				? apiRequest(fetch, '/api/me/invitations', { token })
				: Promise.resolve({ items: [] }),
			apiRequest(fetch, '/api/jury/assignments', { token }),
			shouldLoadManagedTournaments
				? apiRequest(fetch, '/api/me/tournaments', { token })
				: Promise.resolve({ items: [] })
		]);

		return {
			profile,
			teams: teams.items || [],
			invitations: invitations.items || [],
			juryAssignments: juryAssignments.items || [],
			createdTournaments: createdTournaments.items || []
		};
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
	acceptInvitation: async ({ request, cookies, fetch }) => {
		const token = getAuthToken(cookies);
		if (!token) return fail(401, { message: 'Потрібно увійти' });

		const invitationId = await readId(request, 'invitation_id');
		if (!invitationId) return fail(400, { message: 'Invitation id is required' });

		try {
			await apiRequest(fetch, `/api/invitations/${invitationId}/accept`, {
				method: 'POST',
				token
			});
		} catch (error) {
			return actionError(error);
		}

		throw redirect(303, '/profile');
	},

	declineInvitation: async ({ request, cookies, fetch }) => {
		const token = getAuthToken(cookies);
		if (!token) return fail(401, { message: 'Потрібно увійти' });

		const invitationId = await readId(request, 'invitation_id');
		if (!invitationId) return fail(400, { message: 'Invitation id is required' });

		try {
			await apiRequest(fetch, `/api/invitations/${invitationId}/decline`, {
				method: 'POST',
				token
			});
		} catch (error) {
			return actionError(error);
		}

		throw redirect(303, '/profile');
	}
};

async function readId(request, name) {
	const formData = await request.formData();
	return String(formData.get(name) || '').trim();
}

function actionError(error) {
	if (error instanceof ApiError) {
		return fail(error.status, { message: error.message });
	}

	return fail(500, { message: 'Backend недоступний' });
}
