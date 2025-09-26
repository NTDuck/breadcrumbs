<script lang="ts">
	import "../app.css";
	import favicon from "$lib/assets/favicon.svg";

	import { addTwoNumbers } from "$lib";

	const sum = addTwoNumbers(1, 2);

	let { children } = $props();
</script>

<svelte:head>
	<link rel="icon" href={favicon} />
</svelte:head>

{@render children?.()}
