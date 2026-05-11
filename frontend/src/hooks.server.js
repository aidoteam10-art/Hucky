export async function handle({event, resolve}){
    const token = event.cookies.get('jwt');

    if (token){
        try{
            const base64url = token.split('.')[1];

            const base64 = base64url.replace(/-/g, '+').replace(/_/g, '/');

            const jsonPayload = Buffer.from(base64, 'base64').toString('utf-8');
            const decoded = JSON.parse(jsonPayload);

            if (Date.now() >= decoded.exp * 1000){
                throw new Error("Token expired");
            }

            event.locals.user = {
                id: decoded.sub
            };
        }
        catch(error){
            event.cookies.delete('jwt', { path: '/' });
            event.locals.user = null;
        }
    }
    else{
        event.locals.user = null;
    }

    return await resolve(event)

}