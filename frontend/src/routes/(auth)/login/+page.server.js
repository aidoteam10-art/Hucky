import { fail, redirect } from '@sveltejs/kit';
import { ApiError, apiRequest } from '$lib/server/api';

export const actions = {
	default: async ({ request, cookies, fetch }) => {
		const formData = await request.formData();
		const email = String(formData.get('email') || '').trim();
		const password = String(formData.get('password') || '');

		if (!email || !password) {
			return fail(400, { email, message: 'Заповніть email і пароль' });
		}

		try {
			const data = await apiRequest(fetch, '/api/users/login', {
				method: 'POST',
				body: { email, password }
			});

			cookies.set('jwt', data.token, {
				path: '/',
				httpOnly: true,
				sameSite: 'strict',
				secure: false,
				maxAge: 60 * 60 * 24
			});
		} catch (error) {
			if (error instanceof ApiError) {
				return fail(error.status, { email, message: error.message });
			}

			return fail(500, { email, message: 'Backend недоступний' });
		}

		throw redirect(303, '/tournaments');
	}
};
