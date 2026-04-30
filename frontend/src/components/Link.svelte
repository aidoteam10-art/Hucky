<script>
    export let href;
    export let variant = "primary";
    export let size = "md";
    export let className = '';
    export let isHomePage = false;

    $: ringClass = isHomePage ? 'active:ring-[#191F00]' : 'active:ring-white';
</script>

<a
        {href}
        on:click
        class="btn no-underline transition-all duration-300 ease-out flex items-center justify-center w-fit
    /* Адаптивні розміри: sm залишається маленьким, md стає адаптивним */
    {size === 'sm' ? 'text-[0.9rem] lg:text-[1rem] py-2 px-4 rounded-[0.3rem]' : ''}
    {size === 'md' ? 'text-[1rem] lg:text-[1.2rem] py-2.5 px-4 lg:py-3 lg:px-5 rounded-[0.4rem]' : ''}
    {size === 'lg' ? 'text-[1.1rem] lg:text-[1.25rem] py-4 px-12 lg:py-5 lg:px-17.5 rounded-[0.6rem]' : ''}

    /* Варіанти стилів */
    {variant === 'primary' ? 'font-semibold bg-[#191F00] text-white hover:bg-[#344100]' : ''}
    {variant === 'ghost' ? 'font-semibold hover:ring-1 hover:ring-[#191F00]' : ''}
    {variant === 'green' ? 'font-bold text-[#191F00] bg-linear-to-br from-[#BCEB01] to-[#EEFF00] hover:bg-linear-to-tl' : ''}
    {variant === 'outline' ? 'font-bold text-white bg-[#191F00] ring-1 ring-white hover:ring-2' : ''}
    {variant === 'second' ? 'font-bold text-white bg-[#191F00] hover:ring-1' : ''}
    {variant === 'white' ? 'font-bold text-[#191F00] bg-white' : ''}

    active:ring-1 {ringClass} md:active:ring-0

    {className}"
>
    <slot/>
</a>