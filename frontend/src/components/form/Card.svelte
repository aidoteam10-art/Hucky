<script>
    export let as = "div";
</script>

<svelte:element this={as} class="flex flex-col bg-[#FBFFEF] w-fit rounded-2xl outline-1 outline-[#EADDD7] p-8 {$$props.class ?? ''}">
    <slot/>
</svelte:element>
