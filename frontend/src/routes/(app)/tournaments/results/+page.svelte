<script>
    const tournament = { name: "Hackathon Ukraine 2026" };

    const teams = Array.from({ length: 6 }, (_, i) => ({
        id: i,
        name: "Code Warriors",
        scores: {
            backend: 88, database: 92, frontend: 89, requirements: 95, functionality: 90, usability: 87
        },
        total: 537
    }));
</script>

<main class="w-full max-w-[1440px] mx-auto py-8 px-6 font-sans text-[#191F00]">
    
    <h1 class="text-4xl font-bold mb-8 md:mb-10">
        {tournament.name}
    </h1>

    <section class="flex flex-col md:flex-row justify-center items-center md:items-end gap-6 md:gap-0 mb-10 md:mb-16">

        <article class="w-full md:w-60 h-auto md:h-[260px] flex-shrink-0 bg-white border border-[#111] rounded-2xl md:rounded-none md:rounded-l-2xl flex flex-col overflow-hidden relative z-0 md:-mr-px">
            <header class="md:h-32 border-b border-[#111] p-6 flex justify-between items-start">
                <h3 class="text-sm font-bold leading-tight uppercase">Silver<br/>Medalist</h3>
                <span class="text-5xl font-extrabold leading-none">02</span>
            </header>
            <div class="flex-grow bg-[#F8F9FA] p-6 flex flex-col justify-between gap-6 md:gap-0">
                <div>
                    <p class="text-base font-bold">Code Warriors</p>
                    <p class="text-xs font-medium text-gray-500 mt-1">MIT Research Labs</p>
                </div>
                <span class="text-4xl font-extrabold">500</span>
            </div>
        </article>

        <article class="w-full md:w-[280px] h-auto md:h-80 flex-shrink-0 bg-white border border-[#111] rounded-2xl md:rounded-none md:rounded-t-2xl flex flex-col overflow-hidden relative z-10 shadow-lg">
            <header class="md:h-[140px] border-b border-[#111] p-6 flex justify-between items-start">
                <h3 class="text-base font-bold leading-tight uppercase">Grand<br/>Champion</h3>
                <span class="text-6xl font-extrabold leading-[0.9]">01</span>
            </header>
            <div class="flex-grow bg-[#FCF6D6] p-6 flex flex-col justify-between gap-6 md:gap-0">
                <div>
                    <p class="text-xl font-bold">Code Warriors</p>
                    <p class="text-xs font-medium text-gray-500 mt-1">MIT Research Labs</p>
                </div>
                <span class="text-5xl font-extrabold">537</span>
            </div>
        </article>

        <article class="w-full md:w-60 h-auto md:h-[200px] flex-shrink-0 bg-white border border-[#111] rounded-2xl md:rounded-none md:rounded-r-2xl flex flex-col overflow-hidden relative z-0 md:-ml-px">
            <header class="md:h-[85px] border-b border-[#111] p-4 px-6 flex justify-between items-start">
                <h3 class="text-sm font-bold leading-tight uppercase">Bronze<br/>Medalist</h3>
                <span class="text-5xl font-extrabold leading-none">03</span>
            </header>
            <div class="flex-grow bg-[#FBE4D6] px-6 py-5 flex flex-col justify-center gap-2">
                <div>
                    <p class="text-base font-bold">Code Warriors</p>
                    <p class="text-xs font-medium text-gray-500 mt-1">MIT Research Labs</p>
                </div>
                <span class="text-4xl font-extrabold">490</span>
            </div>
        </article>

    </section>

    <section class="border border-[#111] rounded-2xl overflow-hidden bg-white shadow-sm">
        <div class="overflow-x-auto">
            <table class="w-full text-left border-collapse text-sm whitespace-nowrap">
                <thead class="bg-[#CCFF00] border-b border-[#111]">
                    <tr>
                        <th class="py-5 px-6 font-bold w-16">#</th>
                        <th class="py-5 px-6 font-bold">Team</th>
                        <th class="py-5 px-4 font-bold text-center">Backend</th>
                        <th class="py-5 px-4 font-bold text-center">Database</th>
                        <th class="py-5 px-4 font-bold text-center">Frontend</th>
                        <th class="py-5 px-4 font-bold text-center">Requirements</th>
                        <th class="py-5 px-4 font-bold text-center">Functionality</th>
                        <th class="py-5 px-4 font-bold text-center">Usability</th>
                        <th class="py-5 px-6 font-bold text-center">Total</th>
                    </tr>
                </thead>
                <tbody>
                    {#each teams as team, i (team.id)}
                        <tr class="border-b border-[#E5E7EB] last:border-b-0 hover:bg-gray-50 transition-colors">
                            <td class="py-5 px-6 font-bold">{i + 1}</td>
                            <td class="py-5 px-6 font-bold">{team.name}</td>
                            <td class="py-5 px-4 font-medium text-gray-500 text-center">{team.scores.backend}</td>
                            <td class="py-5 px-4 font-medium text-gray-500 text-center">{team.scores.database}</td>
                            <td class="py-5 px-4 font-medium text-gray-500 text-center">{team.scores.frontend}</td>
                            <td class="py-5 px-4 font-medium text-gray-500 text-center">{team.scores.requirements}</td>
                            <td class="py-5 px-4 font-medium text-gray-500 text-center">{team.scores.functionality}</td>
                            <td class="py-5 px-4 font-medium text-gray-500 text-center">{team.scores.usability}</td>
                            <td class="py-5 px-6 font-bold text-center">{team.total}</td>
                        </tr>
                    {/each}
                </tbody>
            </table>
        </div>
    </section>

</main>