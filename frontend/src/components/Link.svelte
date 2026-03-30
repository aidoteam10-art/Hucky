<script>
    export let href;
    export let variant = "primary"; // primary | ghost | green | outline
    export let size = "md"; // md | lg
    export let className = '';
</script>

<a 
    {href}
    class = "btn no-underline transition-all duration-300 ease-out flex items-center justify-center w-fit
    {size === 'md' ? 'text-[1.2rem] py-3 px-5 rounded-[0.4rem]' : ''}
    {size === 'lg' ? 'text-[1.25rem] py-5 px-17.5 rounded' : ''}
    {variant === 'primary' ? 'font-semibold bg-[#191F00] text-white hover:bg-[#344100]' : ''}
    {variant === 'ghost' ? 'font-semibold hover:ring-1 hover:ring-[#191F00]' : ''}
    {variant === 'green' ? 'font-bold text-[#191F00] bg-linear-to-br from-[#BCEB01] to-[#EEFF00] hover:bg-linear-to-tl' : ''}
    {variant === 'outline' ? 'font-bold text-white bg-[#191F00] ring-1 ring-white hover:ring-2' : ''}
    {className}"
>

<slot/>
</a>