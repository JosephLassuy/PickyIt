<script lang="ts">
	import { Home, LogOut, UtensilsCrossed, Calendar, Egg } from 'lucide-svelte';

	export let name: string;
	export let path: string;
	export let active: boolean;
	export let preload = 'hover';
	export let sidebar_minimized: boolean;
	export let icon: 'home' | 'logout' | 'meals' | 'calendar' | 'ingredients';

	const iconComponents = {
		home: Home,
		logout: LogOut,
		meals: UtensilsCrossed,
		calendar: Calendar,
		ingredients: Egg
	};

	const IconComponent = iconComponents[icon];
</script>

<div class="sidebar-item {active ? 'active' : ''}">
	<a
		data-sveltekit-preload-data={preload}
		href={path}
		class="flex items-center p-3 md:p-4 hover:bg-gray-700 transition-colors duration-200"
	>
		<svelte:component this={IconComponent} size={20} class="text-gray-300" />
		{#if !sidebar_minimized}
			<span class="ml-3 text-sm md:text-base">{name}</span>
		{/if}
	</a>
</div>

<style>
	.sidebar-item {
		width: 100%;
	}

	.sidebar-item a {
		color: #e2e8f0;
		text-decoration: none;
	}

	.active {
		background-color: #4a5568;
	}

	.active a {
		font-weight: bold;
	}
</style>
