import { error as kitError, fail, redirect } from '@sveltejs/kit';
import { ApiError, apiRequest, getAuthToken } from '$lib/server/api';

export const load = async ({ params, cookies, fetch }) => {
	const token = getAuthToken(cookies);
	if (!token) {
		throw redirect(303, '/login');
	}

	try {
		const [user, team] = await Promise.all([
			apiRequest(fetch, '/api/users/me', { token }),
			apiRequest(fetch, `/api/teams/${params.team_id}`, { token })
		]);

		let tournament = null;
		try {
			tournament = await apiRequest(fetch, `/api/tournaments/${team.tournament_id}`, { token });
		} catch {
			tournament = null;
		}

		return { user, team, tournament };
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
	updateTeam: async ({ request, params, cookies, fetch }) => {
		const token = getAuthToken(cookies);
		if (!token) return fail(401, { message: 'Потрібно увійти' });

		const formData = await request.formData();
		const values = {
			name: readString(formData, 'name'),
			organization: readString(formData, 'organization'),
			contact: readString(formData, 'contact')
		};

		if (!values.name || values.name.length < 2) {
			return fail(400, { values, message: 'Назва команди повинна містити мінімум 2 символи' });
		}

		try {
			await apiRequest(fetch, `/api/teams/${params.team_id}`, {
				method: 'PATCH',
				token,
				body: {
					name: values.name,
					organization: values.organization || null,
					contact: values.contact || null
				}
			});
		} catch (error) {
			return actionError(error, values);
		}

		throw redirect(303, `/teams/${params.team_id}`);
	},

	createInvitation: async ({ request, params, cookies, fetch }) => {
		const token = getAuthToken(cookies);
		if (!token) return fail(401, { message: 'Потрібно увійти' });

		const formData = await request.formData();
		const email = readString(formData, 'email');
		if (!email) return fail(400, { message: 'Email is required' });

		try {
			await apiRequest(fetch, `/api/teams/${params.team_id}/invitations`, {
				method: 'POST',
				token,
				body: { email }
			});
		} catch (error) {
			return actionError(error);
		}

		throw redirect(303, `/teams/${params.team_id}`);
	},

	cancelInvitation: async ({ request, params, cookies, fetch }) => {
		const token = getAuthToken(cookies);
		if (!token) return fail(401, { message: 'Потрібно увійти' });

		const invitationId = await readId(request, 'invitation_id');
		if (!invitationId) return fail(400, { message: 'Invitation id is required' });

		try {
			await apiRequest(fetch, `/api/invitations/${invitationId}`, {
				method: 'DELETE',
				token
			});
		} catch (error) {
			return actionError(error);
		}

		throw redirect(303, `/teams/${params.team_id}`);
	},

	removeMember: async ({ request, params, cookies, fetch }) => {
		const token = getAuthToken(cookies);
		if (!token) return fail(401, { message: 'Потрібно увійти' });

		const userId = await readId(request, 'user_id');
		if (!userId) return fail(400, { message: 'User id is required' });

		try {
			await apiRequest(fetch, `/api/teams/${params.team_id}/members/${userId}`, {
				method: 'DELETE',
				token
			});
		} catch (error) {
			return actionError(error);
		}

		throw redirect(303, `/teams/${params.team_id}`);
	}
};

async function readId(request, name) {
	const formData = await request.formData();
	return String(formData.get(name) || '').trim();
}

function readString(formData, name) {
	return String(formData.get(name) || '').trim();
}

function actionError(error, values = undefined) {
	if (error instanceof ApiError) {
		return fail(error.status, { values, message: error.message });
	}

	return fail(500, { values, message: 'Backend недоступний' });
}
