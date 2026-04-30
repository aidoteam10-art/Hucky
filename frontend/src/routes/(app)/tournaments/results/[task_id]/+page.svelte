<script>
    import { goto } from '$app/navigation';
    import { page } from '$app/stores';
    
    import GradingLink from '/src/components/GradingLink.svelte';
    import GradingRange from '/src/components/GradingRange.svelte';

    $: isGraded = $page.url.searchParams.get('status') === 'graded';

    const team = {
        name: "Code Warriors"
    };
    
    let criteria = [
        { id: 'backend', name: 'Backend Quality', desc: 'Clean code, patterns, OOP, tests', score: 0 },
        { id: 'database', name: 'Database', desc: 'Structure, setup, optimization', score: 0 },
        { id: 'frontend', name: 'Frontend', desc: 'Clean code, UX/UI, responsiveness, tests', score: 0 },
        { id: 'requirements', name: 'Requirements', desc: 'Must-have features completion', score: 0 },
        { id: 'functionality', name: 'Functionality', desc: 'Working features, no bugs', score: 0 },
        { id: 'usability', name: 'Usability', desc: 'User experience, ease of use', score: 0 }
    ];

    let comment = "";

    $: if (isGraded) {
        const gradedScores = [95, 86, 98, 88, 95, 87];
        criteria = criteria.map((item, index) => ({ ...item, score: gradedScores[index] }));
        comment = "Це дуууже крута робота. Молодці!";
    }

    $: totalScore = criteria.reduce((sum, item) => sum + item.score, 0);

    function handleSubmit() {
        if (isGraded) return;
        goto('/profile'); 
    }
</script>

<main class="w-full min-h-screen bg-white px-5 md:px-10 xl:pl-16 xl:pr-38 py-8 xl:py-16 font-sans text-[#191F00]">
    
    <header class="mb-10 xl:mb-20">
        <h1 class="text-3xl md:text-[2.5rem] font-bold leading-tight mb-2">Оцінювання</h1>
        <p class="text-base md:text-[1.13rem] text-[#696969] font-normal">Перевірте та оцініть роботу команди.</p>
    </header>

    <div class="grid grid-cols-1 xl:grid-cols-12 gap-y-10 xl:gap-y-0 gap-x-12 items-start">
        
        <section class="xl:col-span-7 flex flex-col pt-0 xl:pt-11">
            
            <div class="flex flex-col sm:flex-row justify-start sm:justify-between items-start sm:items-center gap-4 sm:gap-0 mb-6 sm:mb-8">
                <h2 class="text-2xl sm:text-[1.5rem] font-bold leading-none">{team.name}</h2>
                <button 
                    on:click={() => history.back()} 
                    class="flex items-center gap-2 text-sm sm:text-[1rem] text-[#191F00] hover:text-[#696969] transition-colors font-medium bg-transparent"
                >
                    <svg width="18" height="18" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
                        <path d="M19 12H5M12 19l-7-7 7-7"/>
                    </svg>
                    Назад до списку
                </button>
            </div>

            <article class="border border-[#E5E7EB] rounded-[1.5rem] p-6 sm:p-8 bg-white shadow-sm mb-6">
                <h3 class="text-lg sm:text-[1.13rem] font-bold mb-6">Матеріали для перевірки</h3>
                <div class="flex flex-col gap-5">
                    <GradingLink icon="/icons/github_blue.svg" text="GitHub" />
                    <GradingLink icon="/icons/camera_blue.svg" text="Video Demo" />
                    <GradingLink icon="/icons/globe_blue.svg" text="Live Demo URL" />
                </div>
            </article>

            <div class="w-full aspect-video border border-[#E5E7EB] rounded-[1.5rem] bg-white shadow-sm flex items-center justify-center mb-6 px-4 text-center">
                <span class="text-[#696969] text-sm sm:text-[1rem] font-normal">Область попереднього перегляду відео</span>
            </div>
            
            <div class="w-full aspect-video border border-[#E5E7EB] rounded-[1.5rem] bg-white shadow-sm flex items-center justify-center px-4 text-center">
                <span class="text-[#696969] text-sm sm:text-[1rem] font-normal">Область попереднього перегляду відео</span>
            </div>

        </section>

        <section class="xl:col-span-5 flex flex-col">
            
            <article class="border border-[#E5E7EB] rounded-[1.5rem] p-6 sm:p-10 bg-white shadow-sm mb-5">
                <h3 class="text-xl sm:text-[1.25rem] font-bold mb-7">Критерії оцінювання</h3>

                <div class="flex flex-col gap-6">
                    {#each criteria as item}
                        <GradingRange bind:item {isGraded} />
                    {/each}
                </div>
            </article>

            <article class="border border-[#E5E7EB] rounded-[1.5rem] py-5 px-6 sm:py-6 sm:px-8 bg-white shadow-sm flex justify-between items-center mb-5">
                <div>
                    <h3 class="text-xl sm:text-[1.25rem] font-bold mb-1">Загальний підсумок</h3>
                    <p class="text-xs sm:text-sm font-normal text-[#696969]">Із 600</p>
                </div>
                <span class="text-4xl sm:text-[2.25rem] font-black leading-none">{totalScore}</span>
            </article>

            <article class="border border-[#E5E7EB] rounded-[1.5rem] p-6 sm:p-8 bg-white shadow-sm mb-5">
                <h3 class="text-xl sm:text-[1.25rem] font-bold mb-4">Коментар</h3>
                <textarea 
                    bind:value={comment}
                    readonly={isGraded}
                    placeholder="Додатковий відгук..."
                    class="w-full h-24 resize-none rounded-xl p-4 text-sm sm:text-[1rem] outline-none font-normal transition-all placeholder:text-[#A3A3A3] {isGraded ? 'border-transparent px-0 bg-transparent text-[#191F00]' : 'border border-[#E5E7EB] focus:border-[#9BC200]'}"
                ></textarea>
            </article>

            {#if isGraded}
                <div class="w-full bg-white border border-[#E5E7EB] shadow-sm text-[#191F00] font-bold py-4 rounded-2xl text-lg sm:text-[1.13rem] text-center">
                    Оцінено
                </div>
            {:else}
                <button 
                    on:click={handleSubmit}
                    class="w-full bg-[#CCFF00] hover:bg-[#A9D207] shadow-sm text-[#191F00] font-bold py-4 rounded-2xl text-lg sm:text-[1.25rem] transition-colors"
                >
                    Підтвердити оцінювання
                </button>
            {/if}

        </section>
    </div>
</main>