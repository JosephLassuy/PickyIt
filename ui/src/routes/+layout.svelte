<script lang="ts">
    import type { Snippet } from "svelte";
    import type { LayoutData } from "./$types";
    import "../app.css";
    let { data, children }: { data: LayoutData; children: Snippet } = $props();
</script>

<div class="w-screen h-screen flex">
    {@render children()}
</div>
