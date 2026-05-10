import {redirect} from '@sveltejs/kit';


export const actions = {
    default: async ({ cookies, locals }) => {

        cookies.delete('jwt', { path: '/' });

        locals.user = null;

        throw redirect(303, '/');
    }
}