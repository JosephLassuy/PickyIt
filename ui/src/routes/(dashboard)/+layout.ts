import { redirect } from '@sveltejs/kit';
import { PUBLIC_API_DOMAIN } from '$env/static/public';
import type { LayoutLoad } from './$types';

export const load: LayoutLoad = async ({ fetch }) => {
    try {
        const res = await fetch(`${PUBLIC_API_DOMAIN}/verify`, {
            credentials: 'include'
        });
        
        const data = await res.json();
        
        if (!data.authenticated) {
            throw redirect(302, '/login');
        }
        
        return {
            authenticated: true
        };
    } catch (error) {
        throw redirect(302, '/login');
    }
};
