import { redirect } from '@sveltejs/kit';

export const load = async ({ locals, fetch, cookies }) => {
    if (!locals.user) {
        throw redirect(303, '/login');
    }

    const token = cookies.get('jwt');

    try {
        const response = await fetch("http://127.0.0.1:8080/api/users/me", {
            method: 'GET',
            headers: {
                'Authorization': `Bearer ${token}`
            }
        });

        if (!response.ok) {
            throw new Error("Unauthorized");
        }

        const profileData = await response.json();

        return {
            profile: profileData
        };
    } catch (e) {
        // Якщо токен прострочений або сервер впав
        throw redirect(303, '/login');
    }
};