import { fail, redirect } from '@sveltejs/kit';

export const actions = {
    default: async ({ request }) => {
        const formData = await request.formData();
        const full_name = formData.get("name");
        const email = formData.get("email");
        const password = formData.get("password");
        const repeatPassword = formData.get("repeatPassword");

        if (!full_name || !email || !password) {
            return fail(400, { email, full_name, message: "Заповніть всі обов'язкові поля" });
        }

        if (password !== repeatPassword) {
            return fail(400, { email, full_name, message: "Паролі не співпадають" });
        }

        try {
            const response = await fetch("http://127.0.0.1:8080/api/users/register", {
                method: 'POST',
                headers: { 'Content-Type': 'application/json' },
                body: JSON.stringify({ email, password, full_name })
            });

            if (!response.ok) {
                const errorText = await response.text();
                return fail(401, { email, full_name, message: errorText });
            }
        } catch (error) {
            return fail(500, { email, full_name, message: "Сервер недоступний" });
        }
        
        throw redirect(303, '/login');
    }
}
