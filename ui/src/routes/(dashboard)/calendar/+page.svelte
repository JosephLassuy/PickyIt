<script lang="ts">
    import { PUBLIC_API_DOMAIN } from "$env/static/public";
    import { Plus } from "lucide-svelte";
    import { onMount } from "svelte";

    interface MealItem {
        id: string;
        name: string;
    }

    interface CalendarDay {
        date: Date;
        isCurrentMonth: boolean;
        dayOfMonth: number;
    }

    interface CalendarItem {
        id: string;
        meal_item_id: string;
        start_date: Date;
        end_date: Date;
    }

    let meals: MealItem[] = [];
    let calendarItems: CalendarItem[] = [];
    let showMealModal = false;
    let selectedDate: number | null = null;
    let currentDate = new Date();
    let displayMonth = currentDate.getMonth();
    let displayYear = currentDate.getFullYear();
    let calendarDays = getCalendarDays(displayYear, displayMonth);
    let startDate: string | null = null;
    let endDate: string | null = null;
    let selectedMealId: string | null = null;
    let mealSearchQuery = "";
    let filteredMeals: MealItem[] = [];
    let showDeleteModal = false;
    let mealToDelete: { id: string; name: string; start_date: Date; end_date: Date; } | null = null;
    let showEditModal = false;
    let editingItem: { id: string; meal_item_id: string; start_date: string; end_date: string; } | null = null;

    onMount(async () => {
        let calendarResponse = await fetch(`${PUBLIC_API_DOMAIN}/calendar`, {
            credentials: "include",
            method: "GET",
            headers: {
                "Content-Type": "application/json",
            },
        });
        const calendarData = await calendarResponse.json();
        calendarItems = calendarData

        let mealResponse = await fetch(`${PUBLIC_API_DOMAIN}/meal`, {
            credentials: "include",
            method: "GET",
            headers: {
                "Content-Type": "application/json",
            },
        });
        const mealData = await mealResponse.json();
        meals = mealData.meal_items;
    });

    function closeMealModal() {
        showMealModal = false;
        startDate = null;
        endDate = null;
        selectedMealId = null;
    }

    function previousMonth() {
        if (displayMonth === 0) {
            displayMonth = 11;
            displayYear--;
        } else {
            displayMonth--;
        }
        calendarDays = getCalendarDays(displayYear, displayMonth);
    }

    function nextMonth() {
        if (displayMonth === 11) {
            displayMonth = 0;
            displayYear++;
        } else {
            displayMonth++;
        }
        calendarDays = getCalendarDays(displayYear, displayMonth);
    }

    function formatMonthYear(year: number, month: number): string {
        return new Date(year, month).toLocaleDateString("en-US", {
            month: "long",
            year: "numeric",
        });
    }

    function getCalendarDays(year: number, month: number): CalendarDay[] {
        const firstDayOfMonth = new Date(year, month, 1);
        const lastDayOfMonth = new Date(year, month + 1, 0);
        const daysInMonth = lastDayOfMonth.getDate();

        // Get the day of week for the first day (0-6, 0 = Sunday)
        const firstDayOfWeek = firstDayOfMonth.getDay();

        // Calculate days from previous month
        const daysFromPrevMonth = firstDayOfWeek;
        const prevMonth = new Date(year, month - 1, 1);
        const daysInPrevMonth = new Date(year, month, 0).getDate();

        const calendarDays: CalendarDay[] = [];

        // Add days from previous month
        for (
            let i = daysInPrevMonth - daysFromPrevMonth + 1;
            i <= daysInPrevMonth;
            i++
        ) {
            calendarDays.push({
                date: new Date(year, month - 1, i),
                isCurrentMonth: false,
                dayOfMonth: i,
            });
        }

        // Add days from current month
        for (let i = 1; i <= daysInMonth; i++) {
            calendarDays.push({
                date: new Date(year, month, i),
                isCurrentMonth: true,
                dayOfMonth: i,
            });
        }

        // Add days from next month
        const remainingDays = 35 - calendarDays.length; // 5 rows × 7 days = 35
        for (let i = 1; i <= remainingDays; i++) {
            calendarDays.push({
                date: new Date(year, month + 1, i),
                isCurrentMonth: false,
                dayOfMonth: i,
            });
        }

        return calendarDays;
    }

    async function addMealToCalendar() {
        if (!startDate || !endDate || !selectedMealId) {console.log("No start date, end date, or selected meal"); return};
        console.log(startDate, endDate, selectedMealId);

        try {
            const response = await fetch(`${PUBLIC_API_DOMAIN}/calendar`, {
                method: 'POST',
                credentials: 'include',
                headers: {
                    'Content-Type': 'application/json',
                },
                body: JSON.stringify({
                    meal_item_id: selectedMealId,
                    start_date: new Date(startDate).toISOString(),
                    end_date: new Date(endDate).toISOString(),
                    shared_with: [],
                }),
            });

            if (response.ok) {
                // Refresh calendar items
                const calendarResponse = await fetch(`${PUBLIC_API_DOMAIN}/calendar`, {
                    credentials: 'include',
                    method: 'GET',
                    headers: {
                        'Content-Type': 'application/json',
                    },
                });
                const data = await calendarResponse.json();
                calendarItems = data;
                closeMealModal();
            }
        } catch (error) {
            console.error('Failed to add meal to calendar:', error);
        }
    }

    async function deleteMealFromCalendar(itemId: string) {
        try {
            const response = await fetch(`${PUBLIC_API_DOMAIN}/calendar/${itemId}`, {
                credentials: 'include',
                method: 'DELETE',
                headers: {
                    'Content-Type': 'application/json',
                },
            });

            if (response.ok) {
                // Refresh calendar items
                const calendarResponse = await fetch(`${PUBLIC_API_DOMAIN}/calendar`, {
                    credentials: 'include',
                    method: 'GET',
                    headers: {
                        'Content-Type': 'application/json',
                    },
                });
                const data = await calendarResponse.json();
                calendarItems = data;
                closeDeleteModal();
            }
        } catch (error) {
            console.error('Failed to delete meal from calendar:', error);
        }
    }

    const toggleModal = () => {
        showMealModal = !showMealModal;
        console.log(showMealModal);
    };

    $: {
        if (mealSearchQuery) {
            filteredMeals = meals.filter(meal => 
                meal.name.toLowerCase().includes(mealSearchQuery.toLowerCase())
            );
        } else {
            filteredMeals = meals;
        }
    }

    function openDeleteModal(item: CalendarItem) {
        const meal = meals.find(m => m.id === item.meal_item_id);
        mealToDelete = {
            id: item.id,
            name: meal?.name || 'Unknown Meal',
            start_date: new Date(item.start_date),
            end_date: new Date(item.end_date)
        };
        showDeleteModal = true;
    }

    function closeDeleteModal() {
        showDeleteModal = false;
        mealToDelete = null;
    }

    function openEditModal(item: CalendarItem) {
        editingItem = {
            id: item.id,
            meal_item_id: item.meal_item_id,
            start_date: new Date(item.start_date).toISOString().slice(0, 16),
            end_date: new Date(item.end_date).toISOString().slice(0, 16)
        };
        showEditModal = true;
    }

    function closeEditModal() {
        showEditModal = false;
        editingItem = null;
    }

    async function updateCalendarItem() {
        if (!editingItem) return;

        try {
            const response = await fetch(`${PUBLIC_API_DOMAIN}/calendar/${editingItem.id}`, {
                method: 'PUT',
                credentials: 'include',
                headers: {
                    'Content-Type': 'application/json',
                },
                body: JSON.stringify({
                    meal_item_id: editingItem.meal_item_id,
                    start_date: new Date(editingItem.start_date).toISOString(),
                    end_date: new Date(editingItem.end_date).toISOString(),
                }),
            });

            if (response.ok) {
                const calendarResponse = await fetch(`${PUBLIC_API_DOMAIN}/calendar`, {
                    credentials: 'include',
                    method: 'GET',
                    headers: {
                        'Content-Type': 'application/json',
                    },
                });
                const data = await calendarResponse.json();
                calendarItems = data;
                closeEditModal();
            }
        } catch (error) {
            console.error('Failed to update calendar item:', error);
        }
    }
</script>

<div class="flex max-w-6xl mx-auto p-4 sm:p-6 lg:p-8 gap-6">
    <!-- Calendar Section -->
    <div class="flex-1">
        <div class="flex justify-between items-center mb-6">
            <div class="flex items-center gap-4">
                <h1 class="text-3xl font-bold text-white">
                    {formatMonthYear(displayYear, displayMonth)}
                </h1>
                <div class="flex gap-2">
                    <button
                        class="p-2 bg-gray-700 rounded-lg hover:bg-gray-600 transition-colors duration-200"
                        onclick={previousMonth}
                    >
                        ←
                    </button>
                    <button
                        class="p-2 bg-gray-700 rounded-lg hover:bg-gray-600 transition-colors duration-200"
                        onclick={nextMonth}
                    >
                        →
                    </button>
                </div>
            </div>
            <button
                class="flex items-center gap-2 bg-gray-700 text-white px-4 py-2 rounded-lg hover:bg-gray-600 transition-colors duration-200"
                onclick={toggleModal}
            >
                <Plus size={20} />
                Add Meal
            </button>
        </div>

        <!-- Calendar Grid -->
        <div class="grid grid-cols-7 gap-2 mb-2">
            {#each ["Sun", "Mon", "Tue", "Wed", "Thu", "Fri", "Sat"] as day}
                <div class="text-gray-400 text-center font-semibold text-sm">
                    {day}
                </div>
            {/each}
        </div>

        <div class="grid grid-cols-7 gap-2">
            {#each calendarDays as day}
                <div
                    class="aspect-square {day.isCurrentMonth
                        ? 'bg-gray-700'
                        : 'bg-gray-800'} 
                        rounded-lg p-2 flex flex-col relative"
                >
                    <div
                        class="text-sm {day.isCurrentMonth
                            ? 'text-white'
                            : 'text-gray-500'}"
                    >
                        {day.dayOfMonth}
                    </div>
                    <div class="flex-1 overflow-y-auto mt-1 space-y-1">
                        {#each (calendarItems || []).filter(item => {
                            const itemStart = new Date(item.start_date);
                            const itemEnd = new Date(item.end_date);
                            const dayDate = day.date;
                            
                            itemStart.setHours(0, 0, 0, 0);
                            itemEnd.setHours(0, 0, 0, 0);
                            dayDate.setHours(0, 0, 0, 0);
                            
                            return dayDate >= itemStart && dayDate <= itemEnd;
                        }) as item}
                            <div 
                                class="rounded-md px-2 py-1 bg-blue-500/10 border-l-2 border-blue-500 hover:bg-blue-500/20 transition-colors group relative cursor-pointer"
                                title={meals.find(m => m.id === item.meal_item_id)?.name || 'Unknown Meal'}
                                onclick={() => openEditModal(item)}
                                onkeydown={(e) => e.key === 'Enter' && openEditModal(item)}
                                tabindex="0"
                                role="button"
                            >
                                <div class="text-xs text-white truncate flex justify-between items-center">
                                    <span>{meals.find(m => m.id === item.meal_item_id)?.name || 'Unknown Meal'}</span>
                                    <button
                                        class="text-red-400 hover:text-red-300 transition-colors opacity-0 group-hover:opacity-100"
                                        onclick={() => openDeleteModal(item)}
                                        title="Delete meal"
                                    >
                                        ×
                                    </button>
                                </div>
                            </div>
                        {/each}
                    </div>
                </div>
            {/each}
        </div>
    </div>

    <!-- Side Panel -->
    <div class="w-auto bg-gray-700 rounded-lg p-4">
        <h2 class="text-xl font-semibold text-white mb-2">Overview</h2>
        <div class="text-gray-300 mb-4">
            {calendarItems.length} meal{calendarItems.length === 1 ? '' : 's'} scheduled
        </div>
        
        <div class="space-y-2">
            <h3 class="text-lg font-medium text-white">Scheduled Meals</h3>
            {#if calendarItems.length > 0}
                <div class="space-y-2 max-h-[60vh] overflow-y-auto">
                    {#each calendarItems as item}
                        <div class="bg-gray-800 rounded-lg p-3">
                            <div class="text-white font-medium">
                                {meals.find(m => m.id === item.meal_item_id)?.name || 'Unknown Meal'}
                            </div>
                            <div class="text-sm text-gray-400">
                                {new Date(item.start_date).toLocaleDateString()} - {new Date(item.end_date).toLocaleDateString()}
                            </div>
                        </div>
                    {/each}
                </div>
            {:else}
                <div class="text-gray-400 text-sm">
                    No meals scheduled
                </div>
            {/if}
        </div>
    </div>
</div>

<!-- Meal Selection Modal -->
{#if showMealModal}
    <div
        class="fixed inset-0 bg-black bg-opacity-50 flex items-center justify-center p-4"
    >
        <div class="bg-gray-800 p-6 rounded-xl shadow-lg w-full max-w-md">
            <h2 class="text-xl font-bold text-white mb-4">
                Add Meal to Calendar
            </h2>

            <div class="space-y-4">
                <div>
                    <label for="startDate" class="block text-sm text-gray-400 mb-1">Start Date</label>
                    <input 
                        id="startDate"
                        type="datetime-local" 
                        class="w-full bg-gray-700 border-gray-600 rounded-lg p-2 text-white"
                        bind:value={startDate}
                    />
                </div>

                <div>
                    <label for="endDate" class="block text-sm text-gray-400 mb-1">End Date</label>
                    <input 
                        id="endDate"
                        type="datetime-local" 
                        class="w-full bg-gray-700 border-gray-600 rounded-lg p-2 text-white"
                        bind:value={endDate}
                    />
                </div>

                <div class="space-y-2">
                    <label for="mealSearch" class="block text-sm text-gray-400 mb-1">Search Meals</label>
                    <input 
                        type="search" 
                        id="mealSearch"
                        class="w-full bg-gray-700 border-gray-600 rounded-lg p-2 text-white mb-2"
                        placeholder="Search for meals..."
                        bind:value={mealSearchQuery}
                    />
                    
                    <div class="max-h-64 overflow-y-auto">
                        {#each filteredMeals as meal}
                            <button
                                class="w-full text-left px-4 py-2 text-white hover:bg-gray-700 rounded-lg transition-colors duration-200 {selectedMealId === meal.id ? 'bg-gray-700' : ''}"
                                onclick={() => selectedMealId = meal.id}
                            >
                                {meal.name}
                            </button>
                        {/each}
                        
                        {#if filteredMeals.length === 0}
                            <div class="text-gray-400 text-center py-4">
                                {mealSearchQuery ? 'No meals found' : 'No meals available'}
                            </div>
                        {/if}
                    </div>
                </div>
            </div>

            <div class="flex justify-end gap-2 mt-4">
                <button
                    class="px-4 py-2 bg-gray-700 text-white rounded-lg hover:bg-gray-600 transition-colors duration-200"
                    onclick={closeMealModal}
                >
                    Cancel
                </button>
                <button
                    class="px-4 py-2 bg-blue-600 text-white rounded-lg hover:bg-blue-500 transition-colors duration-200"
                    onclick={addMealToCalendar}
                    disabled={!startDate || !endDate || !selectedMealId}
                >
                    Add to Calendar
                </button>
            </div>
        </div>
    </div>
{/if}

<!-- Delete Confirmation Modal -->
{#if showDeleteModal && mealToDelete}
    <div class="fixed inset-0 bg-black bg-opacity-50 flex items-center justify-center p-4">
        <div class="bg-gray-800 p-6 rounded-xl shadow-lg w-full max-w-md">
            <h2 class="text-xl font-bold text-white mb-4">
                Confirm Delete
            </h2>

            <p class="text-gray-300 mb-4">
                Are you sure you want to delete "{mealToDelete.name}" scheduled for:
            </p>
            
            <p class="text-white mb-6">
                {mealToDelete.start_date.toLocaleDateString()} - {mealToDelete.end_date.toLocaleDateString()}
            </p>

            <div class="flex justify-end gap-2">
                <button
                    class="px-4 py-2 bg-gray-700 text-white rounded-lg hover:bg-gray-600 transition-colors duration-200"
                    onclick={closeDeleteModal}
                >
                    Cancel
                </button>
                <button
                    class="px-4 py-2 bg-red-600 text-white rounded-lg hover:bg-red-500 transition-colors duration-200"
                    onclick={() => deleteMealFromCalendar(mealToDelete!.id)}
                >
                    Delete
                </button>
            </div>
        </div>
    </div>
{/if}

<!-- Edit Confirmation Modal -->
{#if showEditModal && editingItem}
    <div class="fixed inset-0 bg-black bg-opacity-50 flex items-center justify-center p-4">
        <div class="bg-gray-800 p-6 rounded-xl shadow-lg w-full max-w-md">
            <h2 class="text-xl font-bold text-white mb-4">
                Edit Calendar Item
            </h2>

            <div class="space-y-4">
                <div>
                    <label for="editStartDate" class="block text-sm text-gray-400 mb-1">Start Date</label>
                    <input 
                        id="editStartDate"
                        type="datetime-local" 
                        class="w-full bg-gray-700 border-gray-600 rounded-lg p-2 text-white"
                        bind:value={editingItem.start_date}
                    />
                </div>

                <div>
                    <label for="editEndDate" class="block text-sm text-gray-400 mb-1">End Date</label>
                    <input 
                        id="editEndDate"
                        type="datetime-local" 
                        class="w-full bg-gray-700 border-gray-600 rounded-lg p-2 text-white"
                        bind:value={editingItem.end_date}
                    />
                </div>

                <div class="space-y-2">
                    <label for="editMealSearch" class="block text-sm text-gray-400 mb-1">Change Meal</label>
                    <input 
                        type="search" 
                        id="editMealSearch"
                        class="w-full bg-gray-700 border-gray-600 rounded-lg p-2 text-white mb-2"
                        placeholder="Search for meals..."
                        bind:value={mealSearchQuery}
                    />
                    
                    <div class="max-h-64 overflow-y-auto">
                        {#each filteredMeals as meal}
                            <button
                                class="w-full text-left px-4 py-2 text-white hover:bg-gray-700 rounded-lg transition-colors duration-200 {editingItem?.meal_item_id === meal.id ? 'bg-gray-700' : ''}"
                                onclick={() => editingItem!.meal_item_id = meal.id}
                            >
                                {meal.name}
                            </button>
                        {/each}
                    </div>
                </div>
            </div>

            <div class="flex justify-end gap-2 mt-4">
                <button
                    class="px-4 py-2 bg-gray-700 text-white rounded-lg hover:bg-gray-600 transition-colors duration-200"
                    onclick={closeEditModal}
                >
                    Cancel
                </button>
                <button
                    class="px-4 py-2 bg-blue-600 text-white rounded-lg hover:bg-blue-500 transition-colors duration-200"
                    onclick={updateCalendarItem}
                >
                    Save Changes
                </button>
            </div>
        </div>
    </div>
{/if}
