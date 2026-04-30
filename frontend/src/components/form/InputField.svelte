<script>
    export let type = "text";
    export let placeholder = "Example text";
    export let header = "header";
    export let variant = "big-header"; // small-header | big-header
    export let src_icon = "";
</script>

<div class="flex flex-col gap-2 {$$props.class ?? ''}">
    <div class="flex items-center gap-1.5">
        {#if src_icon}
            <img src={src_icon} alt="" class="w-4 h-4 object-contain" />
        {/if}
        <h3 class="font-medium text-[#32221B] {variant === 'big-header' ? 'text-sm font-semibold' : 'text-[0.625rem]'}">{header}</h3>
    </div>
    <input {type} {placeholder} class="w-full rounded-lg border border-[#B4B4B4] bg-[#F9FFE5] p-2 pl-2 text-[0.75rem] text-[#756157]">
</div>
