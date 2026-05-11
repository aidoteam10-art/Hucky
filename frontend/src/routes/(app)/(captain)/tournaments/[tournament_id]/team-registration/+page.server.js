import { error as kitError, fail, redirect } from '@sveltejs/kit';
import { ApiError, apiRequest, getAuthToken } from '$lib/server/api';

export const load = async ({ params, cookies, fetch }) => {
	const token = getAuthToken(cookies);
	if (!token) {
		throw redirect(303, '/login');
	}

	try {
		const [user, tournament] = await Promise.all([
			apiRequest(fetch, '/api/users/me', { token }),
			apiRequest(fetch, `/api/tournaments/${params.tournament_id}`, { token })
		]);

		return { user, tournament };
	} catch (error) {
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
			name: readString(formData, 'name'),
			organization: readString(formData, 'organization'),
			contact: readString(formData, 'contact'),
			member_emails: formData
				.getAll('member_emails')
				.map((value) => String(value).trim())
				.filter(Boolean)
		};

		if (!values.name || values.name.length < 2) {
			return fail(400, { values, message: 'Назва команди повинна містити мінімум 2 символи' });
		}

		if (values.member_emails.length === 0) {
			return fail(400, { values, message: 'Додайте хоча б один email учасника' });
		}

		if (values.member_emails.length > 5) {
			return fail(400, { values, message: 'Можна запросити максимум 5 учасників' });
		}

		const normalizedEmails = values.member_emails.map((email) => email.toLowerCase());
		if (new Set(normalizedEmails).size !== normalizedEmails.length) {
			return fail(400, { values, message: 'Email-інвайти не повинні повторюватися' });
		}

		try {
			const currentUser = await apiRequest(fetch, '/api/users/me', { token });
			if (normalizedEmails.includes(String(currentUser.email || '').toLowerCase())) {
				return fail(400, { values, message: 'Email капітана не потрібно додавати до інвайтів' });
			}

			await apiRequest(fetch, `/api/tournaments/${params.tournament_id}/teams`, {
				method: 'POST',
				token,
				body: {
					name: values.name,
					organization: values.organization || null,
					contact: values.contact || null,
					member_emails: values.member_emails
				}
			});
		} catch (error) {
			if (error instanceof ApiError) {
				return fail(error.status, { values, message: error.message });
			}

			return fail(500, { values, message: 'Backend недоступний' });
		}

		throw redirect(303, `/tournaments/${params.tournament_id}`);
	}
};

function readString(formData, name) {
	return String(formData.get(name) || '').trim();
}
