<script lang="ts">
    import { page } from '$app/stores';
    import { PUBLIC_API_DOMAIN } from "$env/static/public";
    import { onMount } from "svelte";
    import { ArrowLeft } from 'lucide-svelte';

    interface IngredientItem {
        id: string;
        name: string;
    }

    interface MealItem {
        id: string;
        name: string;
        ingredient_items: IngredientItem[];
        instructions: string;
    }

    const { id } = $page.params;
    let meal: MealItem | null = null;

    onMount(async () => {
        let response = await fetch(`${PUBLIC_API_DOMAIN}/meal/${id}`, {
            credentials: "include",
            method: "GET",
            headers: {
                "Content-Type": "application/json",
            },
        });
        meal = await response.json();
    });
</script>

<div class="max-w-4xl mx-auto p-4 sm:p-6 lg:p-8">
    <a 
        href="/meals" 
        class="inline-flex items-center text-gray-300 hover:text-white mb-6 transition-colors duration-200"
    >
        <ArrowLeft size={20} class="mr-2" />
        Back to Meals
    </a>

    {#if meal}
        <div class="space-y-6">
            <h1 class="text-3xl font-bold text-white">{meal.name}</h1>
            
            <div class="bg-gray-700 rounded-xl shadow-lg p-6 space-y-6">
                <div>
                    <h2 class="text-xl font-semibold text-white mb-3">Instructions</h2>
                    <p class="text-gray-300 leading-relaxed whitespace-pre-wrap">{meal.instructions}</p>
                </div>

                <div>
                    <h2 class="text-xl font-semibold text-white mb-3">Ingredients</h2>
                    <div class="grid grid-cols-1 sm:grid-cols-2 gap-2">
                        {#each meal.ingredient_items as ingredient_item}
                            <div class="bg-gray-600 rounded-lg px-4 py-2 text-gray-200">
                                {ingredient_item.name}
                            </div>
                        {/each}
                    </div>
                </div>
            </div>
        </div>
    {:else}
        <div class="flex items-center justify-center h-64">
            <div class="animate-pulse text-gray-300">Loading...</div>
        </div>
    {/if}
</div>