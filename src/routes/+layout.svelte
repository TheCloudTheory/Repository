<script>
    import Explorer from "../shared/Explorer.svelte";
    import Menu from "../shared/Menu.svelte";

	let { children } = $props();
</script>

<Menu />
<Explorer />
{@render children()}