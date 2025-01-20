<script lang="ts">
    import { goto } from '$app/navigation';
    import { PUBLIC_API_DOMAIN } from '$env/static/public';
    import { onMount } from 'svelte';

    onMount(async () => {
        try {
            await fetch(`${PUBLIC_API_DOMAIN}/logout`, {
                credentials: 'include',
                method: 'GET'
            });
        } catch (error) {
            console.error('Logout error:', error);
        }
        
        goto('/login');
    });
</script>

<div class="flex items-center justify-center h-screen">
    <div class="text-white">Logging out...</div>
</div>
