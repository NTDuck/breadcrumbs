<script lang="ts">
	import "../app.css";
	import favicon from "$lib/assets/favicon.svg";

	import { addTwoNumbers } from "../../../../backend/bindings/output/breadcrumbs";

	const sum = addTwoNumbers(1, 2);

	let { children } = $props();
</script>

<svelte:head>
	<link rel="icon" href={favicon} />
</svelte:head>

<div class="container mx-auto">
	<h1 class="text-3xl font-bold underline">Hello world!</h1>
	<p>1 + 2 = {sum}</p>
</div>

{@render children?.()}
