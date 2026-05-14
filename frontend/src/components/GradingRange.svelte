<script>
    export let item;
    export let score = 0;
    export let maxScore = 100;
    export let isGraded = false;
    export let onScoreChange = () => {};

    $: safeMaxScore = normalizeMaxScore(maxScore);
    $: safeScore = clampScore(score, safeMaxScore);

    function normalizeMaxScore(value) {
        const parsed = Number(value);
        return Number.isFinite(parsed) && parsed > 0 ? parsed : 100;
    }

    function clampScore(value, max) {
        const parsed = Number(value);
        if (!Number.isFinite(parsed)) return 0;
        return Math.min(max, Math.max(0, Math.round(parsed)));
    }

    function updateScore(event) {
        if (isGraded) return;
        onScoreChange(clampScore(event.currentTarget.value, safeMaxScore));
    }
</script>

<div class="w-full">
    <div class="flex justify-between items-start mb-3.5">
        <div>
            <h4 class="text-[1.1rem] font-medium text-[#191F00] mb-0.5">{item.name}</h4>
            <p class="text-[0.875rem] font-regular text-[#696969]">{item.desc}</p>
        </div>
        <input
            type="number"
            min="0"
            max={safeMaxScore}
            value={safeScore}
            disabled={isGraded}
            aria-label="Score for {item.name}"
            on:input={updateScore}
            class="w-[3ch] bg-transparent p-0 text-right text-[1.25rem] font-black leading-none text-[#191F00] outline-none [appearance:textfield] disabled:opacity-100 [&::-webkit-inner-spin-button]:appearance-none [&::-webkit-outer-spin-button]:appearance-none"
        />
    </div>

    <input 
        type="range" 
        min="0"
        max={safeMaxScore}
        value={safeScore}
        disabled={isGraded}
        aria-label="Score slider for {item.name}"
        on:input={updateScore}
        class="w-full h-1.5 rounded-full appearance-none outline-none transition-all bg-[#E5E7EB]
                [&::-webkit-slider-thumb]:appearance-none [&::-webkit-slider-thumb]:w-4 [&::-webkit-slider-thumb]:h-4 [&::-webkit-slider-thumb]:rounded-full [&::-webkit-slider-thumb]:bg-[#9BC200] [&::-webkit-slider-thumb]:cursor-pointer hover:[&::-webkit-slider-thumb]:scale-110
                disabled:[&::-webkit-slider-thumb]:bg-[#9BC200] disabled:[&::-webkit-slider-thumb]:cursor-default disabled:hover:[&::-webkit-slider-thumb]:scale-100
                [&::-moz-range-thumb]:w-4 [&::-moz-range-thumb]:h-4 [&::-moz-range-thumb]:border-none [&::-moz-range-thumb]:rounded-full [&::-moz-range-thumb]:bg-[#9BC200] [&::-moz-range-thumb]:cursor-pointer hover:[&::-moz-range-thumb]:scale-110
                disabled:[&::-moz-range-thumb]:bg-[#9BC200] disabled:[&::-moz-range-thumb]:cursor-default disabled:hover:[&::-moz-range-thumb]:scale-100"
    />
    
    <div class="flex justify-between items-center mt-3 text-[0.75rem] text-[#A3A3A3] font-medium">
        <span>0</span>
        <span>{safeMaxScore}</span>
    </div>
</div>
