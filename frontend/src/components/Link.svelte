<script>
    export let href;

    export let variant = "primary";

    export let className = '';


</script>

<a 
    {href}
    class = "btn no-underline font-semibold text-[0.8rem] py-3 px-5 rounded-[0.4rem] transition-all duration-300 ease-out
    {variant === 'primary' ? 'bg-[#191F00] text-white hover:bg-[#344100]' : ''}
    {variant === 'ghost' ? 'hover:ring-1 hover:ring-[#191F00]' : ''}
    {className}
    "
>

<slot/>
</a>