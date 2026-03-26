<script>
    export let href;

    export let variant = "primary";

    export let className = '';


</script>

<a 
    {href}
    class = "btn no-underline font-bold text-[0.9rem] py-2 px-4 rounded transition-all duration-300 ease-out
    {variant === 'primary' ? 'bg-[#191F00] text-white hover:bg-[#344100]' : ''}
    {variant === 'ghost' ? 'hover:ring-1 hover:ring-[#191F00]' : ''}
    {className}
    "
>

<slot/>
</a>