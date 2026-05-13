<script>
    export let data;

    const criteria = ['backend', 'database', 'frontend', 'requirements', 'functionality', 'usability'];
    const labels = {
        backend: 'Backend',
        database: 'Database',
        frontend: 'Frontend',
        requirements: 'Requirements',
        functionality: 'Functionality',
        usability: 'Usability'
    };

    $: leaderboard = data.leaderboard;
    $: teams = leaderboard?.items || [];
</script>

<main class="w-full max-w-[1440px] mx-auto py-8 px-6 font-sans text-[#191F00]">

    {#if !leaderboard}
        <section class="rounded-2xl border border-[#E5E7EB] bg-white px-6 py-10 text-center shadow-sm">
            <h1 class="mb-4 text-3xl font-bold">Результати турніру</h1>
            <p class="text-anywhere mb-6 text-[#696969]">{data.message}</p>
            <a href="/tournaments" class="inline-flex rounded-xl bg-[#CCFF00] px-7 py-3 font-bold text-[#191F00] hover:bg-[#A9D207]">
                До турнірів
            </a>
        </section>
    {:else}
    <h1 class="text-anywhere text-4xl font-bold mb-8 md:mb-10">
        {leaderboard.tournament.title}
    </h1>

    <section class="flex flex-col md:flex-row justify-center items-center md:items-end gap-6 md:gap-0 mb-10 md:mb-16">
        {#each teams.slice(0, 3) as team, index (team.team_id)}
            <article class="w-full flex-shrink-0 overflow-hidden rounded-2xl border border-[#111] bg-white md:w-64 {index === 0 ? 'md:h-80 shadow-lg' : 'md:h-[240px]'}">
                <header class="border-b border-[#111] p-6 flex justify-between items-start {index === 0 ? 'bg-[#FCF6D6]' : index === 1 ? 'bg-[#F8F9FA]' : 'bg-[#FBE4D6]'}">
                    <h3 class="text-sm font-bold uppercase leading-tight">
                        {index === 0 ? 'Grand Champion' : index === 1 ? 'Silver Medalist' : 'Bronze Medalist'}
                    </h3>
                    <span class="text-5xl font-extrabold leading-none">{String(team.rank).padStart(2, '0')}</span>
                </header>
                <div class="p-6">
                    <p class="text-anywhere text-base font-bold">{team.team_name}</p>
                    <p class="text-anywhere mt-1 text-xs font-medium text-gray-500">{team.organization || 'No organization'}</p>
                    <span class="mt-8 block text-4xl font-extrabold">{team.total}</span>
                </div>
            </article>
        {:else}
            <div class="w-full rounded-2xl border border-[#E5E7EB] bg-white px-6 py-10 text-center text-[#696969]">
                Оцінки ще не додані.
            </div>
        {/each}
    </section>

    <section class="border border-[#111] rounded-2xl overflow-hidden bg-white shadow-sm">
        <div class="overflow-x-auto">
            <table class="w-full text-left border-collapse text-sm whitespace-nowrap">
                <thead class="bg-[#CCFF00] border-b border-[#111]">
                    <tr>
                        <th class="py-5 px-6 font-bold w-16">#</th>
                        <th class="py-5 px-6 font-bold">Team</th>
                        {#each criteria as criterion}
                            <th class="py-5 px-4 font-bold text-center">{labels[criterion]}</th>
                        {/each}
                        <th class="py-5 px-4 font-bold text-center">Reviews</th>
                        <th class="py-5 px-6 font-bold text-center">Total</th>
                    </tr>
                </thead>
                <tbody>
                    {#each teams as team (team.team_id)}
                        <tr class="border-b border-[#E5E7EB] last:border-b-0 hover:bg-gray-50 transition-colors">
                            <td class="py-5 px-6 font-bold">{team.rank}</td>
                            <td class="text-anywhere py-5 px-6 font-bold whitespace-normal">{team.team_name}</td>
                            {#each criteria as criterion}
                                <td class="py-5 px-4 font-medium text-gray-500 text-center">{team.scores[criterion] ?? '-'}</td>
                            {/each}
                            <td class="py-5 px-4 font-medium text-gray-500 text-center">{team.reviews_count}</td>
                            <td class="py-5 px-6 font-bold text-center">{team.total}</td>
                        </tr>
                    {:else}
                        <tr>
                            <td colspan="10" class="py-10 text-center font-semibold text-[#696969]">
                                Немає завершених оцінювань.
                            </td>
                        </tr>
                    {/each}
                </tbody>
            </table>
        </div>
    </section>
    {/if}

</main>
