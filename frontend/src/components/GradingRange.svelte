<script>
    export let item;
    export let isGraded = false;
</script>

<div class="w-full">
    <div class="flex justify-between items-start mb-3.5">
        <div>
            <h4 class="text-[1.1rem] font-medium text-[#191F00] mb-0.5">{item.name}</h4>
            <p class="text-[0.875rem] font-regular text-[#696969]">{item.desc}</p>
        </div>
        <span class="text-[1.25rem] font-black leading-none">{item.score}</span>
    </div>

    <input 
        type="range" 
        min="0" max="100" 
        bind:value={item.score}
        disabled={isGraded}
        class="w-full h-1.5 rounded-full appearance-none outline-none transition-all bg-[#E5E7EB]
                [&::-webkit-slider-thumb]:appearance-none [&::-webkit-slider-thumb]:w-4 [&::-webkit-slider-thumb]:h-4 [&::-webkit-slider-thumb]:rounded-full [&::-webkit-slider-thumb]:bg-[#9BC200] [&::-webkit-slider-thumb]:cursor-pointer hover:[&::-webkit-slider-thumb]:scale-110
                disabled:[&::-webkit-slider-thumb]:bg-[#9BC200] disabled:[&::-webkit-slider-thumb]:cursor-default disabled:hover:[&::-webkit-slider-thumb]:scale-100
                [&::-moz-range-thumb]:w-4 [&::-moz-range-thumb]:h-4 [&::-moz-range-thumb]:border-none [&::-moz-range-thumb]:rounded-full [&::-moz-range-thumb]:bg-[#9BC200] [&::-moz-range-thumb]:cursor-pointer hover:[&::-moz-range-thumb]:scale-110
                disabled:[&::-moz-range-thumb]:bg-[#9BC200] disabled:[&::-moz-range-thumb]:cursor-default disabled:hover:[&::-moz-range-thumb]:scale-100"
    />
    
    <div class="flex justify-between items-center mt-3 text-[0.75rem] text-[#A3A3A3] font-medium">
        <span>0</span>
        <span>100</span>
    </div>
</div>