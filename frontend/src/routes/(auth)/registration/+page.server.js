import { fail, redirect } from '@sveltejs/kit';
import { ApiError, apiRequest } from '$lib/server/api';

export const actions = {
	default: async ({ request, fetch }) => {
		const formData = await request.formData();
		const full_name = String(formData.get('name') || '').trim();
		const email = String(formData.get('email') || '').trim();
		const password = String(formData.get('password') || '');
		const repeatPassword = String(formData.get('repeatPassword') || '');

		if (!full_name || !email || !password) {
			return fail(400, { email, full_name, message: "Заповніть обов'язкові поля" });
		}

		if (password !== repeatPassword) {
			return fail(400, { email, full_name, message: 'Паролі не співпадають' });
		}

		try {
			await apiRequest(fetch, '/api/users/register', {
				method: 'POST',
				body: { email, password, full_name }
			});
		} catch (error) {
			if (error instanceof ApiError) {
				return fail(error.status, { email, full_name, message: error.message });
			}

			return fail(500, { email, full_name, message: 'Backend недоступний' });
		}

		throw redirect(303, '/login');
	}
};
