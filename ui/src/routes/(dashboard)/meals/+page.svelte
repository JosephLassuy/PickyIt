<script lang="ts">
    import { PUBLIC_API_DOMAIN } from "$env/static/public";
    import { Plus, Trash2 } from "lucide-svelte";
    import { onMount } from "svelte";

    interface MealItem {
        id: string;
        name: string;
        ingredients: string[];
        instructions: string;
        creator_id: string;
    }

    interface MealItemsResponse {
        meal_items: MealItem[];
    }

    interface IngredientItem {
        id: string;
        name: string;
    }

    interface IngredientItemsResponse {
        ingredient_items: IngredientItem[];
    }

    let mealName = "";
    let mealInstructions = "";
    let mealIngredients: IngredientItem[] = [];
    let searchResults: IngredientItem[] = [];
    let searchQuery = "";
    let showModal = false;
    let searchTimeout: ReturnType<typeof setTimeout>;
    let meals: MealItem[] = [];
    let filteredMeals: MealItem[] = [];
    let mealSearchQuery = "";
    let currentUserId = '';

    const getCurrentUser = async () => {
        let response = await fetch(`${PUBLIC_API_DOMAIN}/verify`, {
            credentials: "include",
            method: "GET",
        });
        const data = await response.json();
        currentUserId = data.user_id;
    };

    const getMeals = async () => {
        let response = await fetch(`${PUBLIC_API_DOMAIN}/meal`, {
            credentials: "include",
            method: "GET",
            headers: {
                "Content-Type": "application/json",
            },
        });
        const data: MealItemsResponse = await response.json();
        meals = data.meal_items;
        filteredMeals = meals;
    };

    const deleteMeal = async (id: string) => {
        if (!confirm('Are you sure you want to delete this meal?')) {
            return;
        }

        let response = await fetch(`${PUBLIC_API_DOMAIN}/meal/${id}`, {
            credentials: "include",
            method: "DELETE",
            headers: {
                "Content-Type": "application/json",
            },
        });

        if (response.ok) {
            await getMeals();
        }
    };

    onMount(async () => {
        await getCurrentUser();
        await getMeals();
    });
    
    const toggleModal = () => {
        showModal = !showModal;
    };

    // Add reactive statement for meal search
    $: {
        if (mealSearchQuery) {
            filteredMeals = meals.filter(meal => 
                meal.name.toLowerCase().includes(mealSearchQuery.toLowerCase())
            );
        } else {
            filteredMeals = meals;
        }
    }

    async function searchIngredient() {
        if (searchQuery.length < 1) {
            searchResults = [];
            return;
        }

        try {
            const response = await fetch(
                `${PUBLIC_API_DOMAIN}/ingredient?search=${encodeURIComponent(searchQuery)}`, 
                {
                    credentials: "include",
                    method: "GET",
                    headers: {
                        "Content-Type": "application/json",
                    },
                }
            );

            if (response.ok) {
                const data: IngredientItemsResponse = await response.json();
                searchResults = data.ingredient_items;
            }
        } catch (error) {
            console.error("Error searching ingredient:", error);
        }
    }

    function debounceSearch() {
        clearTimeout(searchTimeout);
        searchTimeout = setTimeout(() => {
            searchIngredient();
        }, 300);
    }

    function addIngredient(item: IngredientItem) {
        if (mealIngredients.some(ingredient => ingredient.id === item.id)) {
            searchQuery = "";
            searchResults = [];
            return;
        }
        mealIngredients = [...mealIngredients, item];
        searchQuery = "";
        searchResults = [];
    }

    function removeIngredient(id: string) {
        mealIngredients = mealIngredients.filter(item => item.id !== id);
    }

    $: {
        if (searchQuery) {
            debounceSearch();
        } else {
            searchResults = [];
        }
    }

    const addMeal = async () => {
        let response = await fetch(`${PUBLIC_API_DOMAIN}/meal`, {
            credentials: "include",
            method: "POST",
            headers: {
                "Content-Type": "application/json",
            },
            body: JSON.stringify({
                name: mealName,
                ingredients: mealIngredients.map(ingredient => ingredient.id),
                instructions: mealInstructions,
            }),
        });
        toggleModal();
        await getMeals();
    };
</script>

<div class="max-w-4xl mx-auto p-4 sm:p-6 lg:p-8">
    <div class="flex justify-between items-center mb-6">
        <h1 class="text-3xl font-bold text-white">Meals</h1>
        <button 
            class="flex items-center gap-2 bg-gray-700 text-white px-4 py-2 rounded-lg hover:bg-gray-600 transition-colors duration-200" 
            on:click={toggleModal}
        >
            <Plus size={20} />
            Add Meal
        </button>
    </div>

    <!-- Add Search Input -->
    <div class="mb-6">
        <input 
            type="search" 
            class="w-full bg-gray-700 border-gray-600 rounded-lg p-2 text-white"
            placeholder="Search meals..."
            bind:value={mealSearchQuery}
        >
    </div>

    <!-- Updated Meals Grid -->
    <div class="grid grid-cols-1 sm:grid-cols-2 md:grid-cols-3 gap-4">
        {#each filteredMeals as meal}
            <div class="bg-gray-700 p-4 rounded-xl shadow-lg hover:bg-gray-600 transition-colors duration-200">
                <div class="flex justify-between items-start">
                    <a href={`/meals/${meal.id}`} class="flex-1">
                        <h3 class="text-lg font-semibold text-white truncate">{meal.name}</h3>
                    </a>
                    {#if meal.creator_id === currentUserId}
                        <button 
                            class="text-gray-400 hover:text-red-400 transition-colors duration-200 ml-2"
                            on:click={() => deleteMeal(meal.id)}
                            title="Delete meal"
                        >
                            <Trash2 size={18} />
                        </button>
                    {/if}
                </div>
            </div>
        {/each}
        
        {#if filteredMeals.length === 0}
            <div class="col-span-full text-center py-8 text-gray-400">
                {mealSearchQuery ? 'No meals found' : 'No meals available'}
            </div>
        {/if}
    </div>
</div>

{#if showModal}
    <div class="fixed inset-0 bg-black bg-opacity-50 flex items-center justify-center p-4">
        <div class="bg-gray-800 p-6 rounded-xl shadow-lg w-full max-w-md text-white">
            <h2 class="text-xl font-bold mb-4">Add New Meal</h2>

            <div class="space-y-4">
                <div>
                    <label for="mealName" class="block mb-1">Name</label>
                    <input 
                        type="text" 
                        id="mealName" 
                        class="w-full bg-gray-700 border-gray-600 rounded-lg p-2 text-white"
                        placeholder="Enter meal name"
                        bind:value={mealName}
                    >
                </div>
                <div>
                    <label for="mealInstructions" class="block mb-1">Instructions</label>
                    <input 
                        type="text" 
                        id="mealInstructions" 
                        class="w-full bg-gray-700 border-gray-600 rounded-lg p-2 text-white"
                        placeholder="Enter meal instructions"
                        bind:value={mealInstructions}
                    >
                </div>
                <div class="relative">
                    <label for="searchIngredients" class="block mb-1">Search Ingredients</label>
                    <input 
                        type="search" 
                        id="searchIngredients" 
                        class="w-full bg-gray-700 border-gray-600 rounded-lg p-2 text-white"
                        placeholder="Search for ingredients"
                        bind:value={searchQuery}
                    >
                    {#if searchResults.length > 0}
                        <div class="absolute z-10 w-full bg-gray-700 border border-gray-600 rounded-lg mt-1 max-h-48 overflow-y-auto">
                            {#each searchResults as item}
                                {@const isSelected = mealIngredients.some(ingredient => ingredient.id === item.id)}
                                <button
                                    class="w-full text-left px-4 py-2 hover:bg-gray-600 flex justify-between items-center {isSelected ? 'bg-gray-600 text-gray-400' : ''}"
                                    on:click={() => addIngredient(item)}
                                    disabled={isSelected}
                                >
                                    <span>{item.name}</span>
                                    {#if isSelected}
                                        <span class="text-sm text-gray-400">Already added</span>
                                    {/if}
                                </button>
                            {/each}
                        </div>
                    {/if}
                </div>

                {#if mealIngredients.length > 0}
                    <div class="mt-4">
                        <h3 class="font-semibold mb-2">Selected Ingredients:</h3>
                        <div class="flex flex-wrap gap-2">
                            {#each mealIngredients as ingredient}
                                <div class="bg-gray-700 px-3 py-1 rounded-full flex items-center gap-2">
                                    <span>{ingredient.name}</span>
                                    <button
                                        class="text-red-400 hover:text-red-300"
                                        on:click={() => removeIngredient(ingredient.id)}
                                    >
                                        ×
                                    </button>
                                </div>
                            {/each}
                        </div>
                    </div>
                {/if}

                <div class="flex justify-end gap-2 mt-6">
                    <button 
                        type="button" 
                        class="px-4 py-2 bg-gray-700 rounded-lg hover:bg-gray-600 transition-colors duration-200"
                        on:click={toggleModal}
                    >
                        Cancel
                    </button>
                    <button 
                        on:click={addMeal}
                        type="submit" 
                        class="px-4 py-2 bg-blue-600 text-white rounded-lg hover:bg-blue-500 transition-colors duration-200"
                    >
                        Save
                    </button>
                </div>
            </div>
        </div>
    </div>
{/if}