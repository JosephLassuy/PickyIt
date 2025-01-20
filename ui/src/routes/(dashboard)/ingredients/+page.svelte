<script lang="ts">
    import { PUBLIC_API_DOMAIN } from "$env/static/public";
    import { Plus, Trash2, Filter } from "lucide-svelte";
    import { onMount } from "svelte";

    interface IngredientItem {
        id: string;
        name: string;
        creator_id: string;
    }

    interface IngredientItemsResponse {
        ingredient_items: IngredientItem[];
    }

    let ingredient_items: IngredientItem[] = [];
    let showOnlyMine = false;
    let currentUserId = '';

    const getIngredientItems = async () => {
        let url = `${PUBLIC_API_DOMAIN}/ingredient`;
        if (showOnlyMine) {
            url += '?mine=true';
        }
        
        let response = await fetch(url, {
            credentials: "include",
            method: "GET",
            headers: {
                "Content-Type": "application/json",
            },
        });
        const data: IngredientItemsResponse = await response.json();
        ingredient_items = data.ingredient_items;
    };

    const getCurrentUser = async () => {
        let response = await fetch(`${PUBLIC_API_DOMAIN}/verify`, {
            credentials: "include",
            method: "GET",
        });
        const data = await response.json();
        currentUserId = data.user_id;
    };

    onMount(async () => {
        await getCurrentUser();
        await getIngredientItems();
    });

    let ingredientName = "";
    let showModal = false;

    const addIngredient = async () => {
        let response = await fetch(`${PUBLIC_API_DOMAIN}/ingredient`, {
            credentials: "include",
            method: "POST",
            headers: {
                "Content-Type": "application/json",
            },
            body: JSON.stringify({
                name: ingredientName,
            }),
        });
        ingredientName = "";
        showModal = false;
        await getIngredientItems();
    };
    
    const toggleModal = () => {
        showModal = !showModal;
    };

    const deleteIngredient = async (id: string) => {
        if (!confirm('Are you sure you want to delete this ingredient?')) {
            return;
        }

        let response = await fetch(`${PUBLIC_API_DOMAIN}/ingredient/${id}`, {
            credentials: "include",
            method: "DELETE",
            headers: {
                "Content-Type": "application/json",
            },
        });

        if (response.ok) {
            await getIngredientItems();
        }
    };

    const toggleFilter = async () => {
        showOnlyMine = !showOnlyMine;
        await getIngredientItems();
    };
</script>

<div class="max-w-4xl mx-auto p-4 sm:p-6 lg:p-8">
    <div class="flex justify-between items-center mb-6">
        <div class="flex items-center gap-4">
            <h1 class="text-3xl font-bold text-white">Ingredients</h1>
            <button 
                class="flex items-center gap-2 bg-gray-700 text-white px-3 py-1 rounded-lg hover:bg-gray-600 transition-colors duration-200 text-sm"
                on:click={toggleFilter}
            >
                <Filter size={16} />
                {showOnlyMine ? 'Show All' : 'Show Mine'}
            </button>
        </div>
        <button 
            class="flex items-center gap-2 bg-gray-700 text-white px-4 py-2 rounded-lg hover:bg-gray-600 transition-colors duration-200" 
            on:click={toggleModal}
        >
            <Plus size={20} />
            Add Ingredient
        </button>
    </div>

    <div class="grid grid-cols-1 sm:grid-cols-2 md:grid-cols-3 gap-4">
        {#each ingredient_items as item}
            <div class="bg-gray-700 py-3 px-4 rounded-xl shadow-lg hover:bg-gray-600 transition-colors duration-200 flex items-center justify-between">
                <h3 class="text-lg font-semibold text-white truncate">{item.name}</h3>
                {#if item.creator_id === currentUserId}
                    <button 
                        class="text-gray-400 hover:text-red-400 transition-colors duration-200"
                        on:click={() => deleteIngredient(item.id)}
                        title="Delete ingredient"
                    >
                        <Trash2 size={18} />
                    </button>
                {/if}
            </div>
        {/each}
    </div>
</div>

{#if showModal}
    <div class="fixed inset-0 bg-black bg-opacity-50 flex items-center justify-center p-4">
        <div class="bg-gray-800 p-6 rounded-xl shadow-lg w-full max-w-md text-white">
            <h2 class="text-xl font-bold mb-4">Add New Ingredient</h2>

            <div>
                <label for="ingredientName" class="block mb-1">Ingredient Name</label>
                <input 
                    type="text" 
                    id="ingredientName" 
                    class="w-full bg-gray-700 border-gray-600 rounded-lg p-2 text-white"
                    placeholder="Enter ingredient name"
                    bind:value={ingredientName}
                >
            </div>
            <div class="flex justify-end gap-2 mt-4">
                <button 
                    type="button" 
                    class="px-4 py-2 bg-gray-700 rounded-lg hover:bg-gray-600 transition-colors duration-200"
                    on:click={toggleModal}
                >
                    Cancel
                </button>
                <button 
                    on:click={addIngredient}
                    type="button" 
                    class="px-4 py-2 bg-blue-600 text-white rounded-lg hover:bg-blue-500 transition-colors duration-200"
                >
                    Add
                </button>
            </div>
        </div>
    </div>
{/if}