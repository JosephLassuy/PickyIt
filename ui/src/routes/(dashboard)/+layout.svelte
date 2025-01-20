<script lang="ts">
	import type { Snippet } from "svelte";
	import type { LayoutData } from "../$types";
	import SidebarItem from "../../components/SidebarItem.svelte";
	import { writable } from "svelte/store";

	import { Menu, ChevronsRight, ChevronsLeft } from "lucide-svelte";
	import { page } from "$app/state";
	let sidebar_minimized = writable(true);
	let showMobileMenu = writable(false);

	function toggleSidebar() {
		sidebar_minimized.update((value) => !value);
	}

	function toggleMobileMenu() {
		showMobileMenu.update((value) => !value);
	}

	let { data, children }: { data: LayoutData; children: Snippet } = $props();
</script>

<div class="flex h-screen w-screen overflow-hidden bg-gray-800">
	<button
		onclick={toggleMobileMenu}
		class="fixed top-4 left-4 z-[100] block md:hidden bg-gray-700 p-2 rounded-md hover:bg-gray-600 transition-colors duration-200"
	>
		<Menu size={24} class="text-white" />
	</button>

	<aside
		class="fixed md:relative z-40 h-full transition-all duration-300 ease-in-out bg-gray-800
		{$showMobileMenu ? 'translate-x-0' : '-translate-x-full md:translate-x-0'}
		{$sidebar_minimized ? 'w-16 md:w-20' : 'w-64'}"
	>
		<div class="flex flex-col h-full text-white">
			<div
				class="flex items-center justify-center border-b border-gray-700 p-4"
			>
				<div class="text-xl font-bold">
					{#if !$sidebar_minimized}
						PickyIt
					{:else}
						<span class="md:inline">P</span>
					{/if}
				</div>
			</div>
			<div class="flex-1 mt-4">
				<SidebarItem
					name="Home"
					path="/"
					active={page.url.pathname === "/"}
					icon="home"
					sidebar_minimized={$sidebar_minimized}
				/>
				<SidebarItem
					name="Ingredients"
					path="/ingredients"
					active={page.url.pathname.includes("/ingredients")}
					icon="ingredients"
					sidebar_minimized={$sidebar_minimized}
				/>
				<SidebarItem
					name="Meals"
					path="/meals"
					active={page.url.pathname.includes("/meals")}
					icon="meals"
					sidebar_minimized={$sidebar_minimized}
				/>
				<SidebarItem
					name="Calendar"
					path="/calendar"
					active={page.url.pathname.includes("/calendar")}
					icon="calendar"
					sidebar_minimized={$sidebar_minimized}
			/>
				<SidebarItem
					name="Logout"
					path="/logout"
					active={false}
					preload="off"
					icon="logout"
					sidebar_minimized={$sidebar_minimized}
				/>
			</div>
			<button
				onclick={toggleSidebar}
				class="mt-auto hidden md:flex items-center justify-center p-4 hover:bg-gray-700 transition-colors duration-200 mx-auto w-full"
			>
				{#if $sidebar_minimized}
					<ChevronsRight size={24} />
				{:else}
					<ChevronsLeft size={24} />
				{/if}
			</button>
		</div>
	</aside>

	<main
		class="flex-1 relative w-full md:ml-0 overflow-x-hidden pl-16 md:pl-0"
	>
		<div
			class="min-h-screen p-4 md:p-6 {$showMobileMenu
				? 'blur-sm md:blur-none'
				: ''}"
		>
			{@render children()}
		</div>
	</main>
</div>
