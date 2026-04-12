<script>
    export let href = "#";
    export let icon = "";
    export let text = "";
</script>

<a {href} on:click|preventDefault class="flex items-center gap-3 text-[#2563EB] hover:text-[#1D4ED8] w-max font-medium text-[1rem] transition-colors">
    <img src={icon} alt={text} class="w-5 h-5" />
    {text}
    <svg width="14" height="14" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" class="opacity-70 ml-1">
        <path d="M18 13v6a2 2 0 0 1-2 2H5a2 2 0 0 1-2-2V8a2 2 0 0 1 2-2h6"></path>
        <polyline points="15 3 21 3 21 9"></polyline>
        <line x1="10" y1="14" x2="21" y2="3"></line>
    </svg>
</a>