import { fail, redirect } from '@sveltejs/kit';

export const actions = {
    default: async ({ request, cookies }) => {
        const formData = await request.formData();
        const email = formData.get("email");
        const password = formData.get("password");

        if (!email || !password) {
            return fail(400, { email, message: "Заповніть всі поля" });
        }

        try {
            const response = await fetch("http://127.0.0.1:8080/api/users/login", {
                method: 'POST',
                headers: { 'Content-Type': 'application/json' },
                body: JSON.stringify({ email, password })
            });

            if (!response.ok) {
                const errorText = await response.text();
                return fail(401, { email, message: errorText });
            }

            const data = await response.json();
            const token = data.token;

            cookies.set('jwt', token, {
                path: '/',
                httpOnly: true, 
                sameSite: 'strict',
                secure: import.meta.env.PROD, 
                maxAge: 60 * 60 * 24
            });
        } catch (error) { 
            return fail(500, { email, message: "Сервер недоступний" });
        }
        
        throw redirect(303, '/');
    }
}