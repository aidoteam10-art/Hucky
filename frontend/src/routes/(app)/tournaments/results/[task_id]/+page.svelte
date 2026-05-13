<script>
    import GradingLink from '/src/components/GradingLink.svelte';
    import GradingRange from '/src/components/GradingRange.svelte';

    export let data;
    export let form;

    $: assignment = data.assignment;
    $: isGraded = assignment.status === 'completed';
    $: criteria = assignment.criteria.map((item) => ({
        ...item,
        desc: item.description,
        score: item.score ?? 0
    }));
    $: comment = assignment.comment ?? '';
    $: totalScore = criteria.reduce((sum, item) => sum + Number(item.score || 0), 0);
</script>

<main class="w-full min-h-screen bg-white px-5 md:px-10 xl:pl-16 xl:pr-38 py-8 xl:py-16 font-sans text-[#191F00]">
    
    <header class="mb-10 xl:mb-20">
        <h1 class="text-3xl md:text-[2.5rem] font-bold leading-tight mb-2">Оцінювання</h1>
        <p class="text-base md:text-[1.13rem] text-[#696969] font-normal">Перевірте та оцініть роботу команди.</p>
    </header>

    <div class="grid grid-cols-1 xl:grid-cols-12 gap-y-10 xl:gap-y-0 gap-x-12 items-start">
        
        <section class="xl:col-span-7 flex flex-col pt-0 xl:pt-11">
            
            <div class="flex flex-col sm:flex-row justify-start sm:justify-between items-start sm:items-center gap-4 sm:gap-0 mb-6 sm:mb-8">
                <h2 class="text-anywhere text-2xl sm:text-[1.5rem] font-bold leading-none">{assignment.team.name}</h2>
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
                    <GradingLink href={assignment.submission.github_url} icon="/icons/github_blue.svg" text="GitHub" />
                    <GradingLink href={assignment.submission.video_demo_url} icon="/icons/camera_blue.svg" text="Video Demo" />
                    {#if assignment.submission.live_demo_url}
                        <GradingLink href={assignment.submission.live_demo_url} icon="/icons/globe_blue.svg" text="Live Demo URL" />
                    {/if}
                </div>
            </article>

            <article class="border border-[#E5E7EB] rounded-[1.5rem] p-6 sm:p-8 bg-white shadow-sm">
                <h3 class="text-lg sm:text-[1.13rem] font-bold mb-4">Опис від команди</h3>
                <p class="text-anywhere whitespace-pre-line text-sm sm:text-[1rem] text-[#696969]">
                    {assignment.submission.description || 'Команда не додала опис.'}
                </p>
            </article>

        </section>

        <section class="xl:col-span-5 flex flex-col">
            
            {#if form?.message}
                <div class="mb-5 rounded-xl border border-red-200 bg-red-50 px-5 py-4 text-sm font-semibold text-red-700">
                    <span class="text-anywhere block">{form.message}</span>
                </div>
            {/if}

            <form method="POST">
                <article class="border border-[#E5E7EB] rounded-[1.5rem] p-6 sm:p-10 bg-white shadow-sm mb-5">
                    <h3 class="text-xl sm:text-[1.25rem] font-bold mb-7">Критерії оцінювання</h3>

                    <div class="flex flex-col gap-6">
                        {#each criteria as item}
                            <input type="hidden" name="criterion_id" value={item.id} />
                            <input type="hidden" name="score" value={item.score} />
                            <GradingRange bind:item {isGraded} />
                        {/each}
                    </div>
                </article>

                <article class="border border-[#E5E7EB] rounded-[1.5rem] py-5 px-6 sm:py-6 sm:px-8 bg-white shadow-sm flex justify-between items-center mb-5">
                    <div class="min-w-0">
                        <h3 class="text-xl sm:text-[1.25rem] font-bold mb-1">Загальний підсумок</h3>
                        <p class="text-xs sm:text-sm font-normal text-[#696969]">Із {criteria.length * 100}</p>
                    </div>
                    <span class="text-4xl sm:text-[2.25rem] font-black leading-none">{totalScore}</span>
                </article>

                <article class="border border-[#E5E7EB] rounded-[1.5rem] p-6 sm:p-8 bg-white shadow-sm mb-5">
                    <h3 class="text-xl sm:text-[1.25rem] font-bold mb-4">Коментар</h3>
                    <textarea 
                        name="comment"
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
                        type="submit"
                        class="w-full bg-[#CCFF00] hover:bg-[#A9D207] shadow-sm text-[#191F00] font-bold py-4 rounded-2xl text-lg sm:text-[1.25rem] transition-colors"
                    >
                        Підтвердити оцінювання
                    </button>
                {/if}
            </form>

        </section>
    </div>
</main>
