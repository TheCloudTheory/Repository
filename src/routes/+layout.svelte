<script>
    import Explorer from "../shared/Explorer.svelte";
    import Footer from "../shared/Footer.svelte";
    import Menu from "../shared/Menu.svelte";

	let { children } = $props();
</script>

<Menu />
<Explorer />
{@render children()}
<Footer />