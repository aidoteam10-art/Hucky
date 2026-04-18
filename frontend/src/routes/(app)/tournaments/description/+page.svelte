<script>
    import StateTag from "../../../../components/StateTag.svelte";
    let current_state = "completed";

    let tournament = {
        name: "Hackathon Ukraine 2026",
        status: current_state,
        description: "Build innovative solutions using modern web technologies. Teams compete across multiple rounds with expert jury evaluation.",
        startRegDate: "01.04.2026",
        endRegDate: "21.04.2026",
        startDate: "21.03.2026",
        endDate: "30.04.2024",
        teamsMaxNum: 20,
        teamsNum: 12,
        round: "MVP Development",
        time: "30:23:50:23",
        registeredTeams: [
            { name: "Code Warriors", count: 3 },
            { name: "Binary Bandits", count: 2 },
            { name: "Pixel Pioners", count: 4 },
            { name: "Hard Coders", count: 3 }
        ],
        rules: [
            "Lorem Ipsum is simply dummy text of the printing and typesetting industry.",
            "Lorem ipsum dolor sit amet, consectetur adipiscing elit, sed do eiusmod tempor incididunt ut labore et dolore magna aliqua.",
            "Duis aute irure dolor in reprehenderit in voluptate velit esse cillum dolore eu fugiat nulla pariatur.",
            "Excepteur sint occaecat cupidatat non proident, sunt in culpa qui officia deserunt mollit anim id est laborum."
        ]
    };
</script>

<main class="w-full px-8 lg:px-12 xl:pl-16 xl:pr-38 py-10 lg:py-16 font-sans text-[#191F00]">

    <section class="flex flex-col lg:flex-row lg:justify-between lg:items-end mb-12 lg:mb-20 gap-8">
        <div class="max-w-2.2xl">
            <div class="flex flex-wrap items-center gap-4 lg:gap-9 mb-6 lg:mb-7">
                <h2 class="text-2xl md:text-3xl xl:text-[2.5rem] font-bold leading-tight">{tournament.name}</h2>
                <StateTag variant={current_state} />
            </div>
            <p class="text-[1rem] xl:text-[1.25rem] w-full xl:w-3xl leading-relaxed">{tournament.description}</p>
        </div>

        <div class="flex flex-col items-start lg:items-end w-full lg:w-auto">
            {#if tournament.status === "registration"}
                <a href="/team_registration" class="w-full lg:w-auto text-center bg-[#CCFF00] hover:bg-[#A9D207] text-[#191F00] font-bold py-3 px-10 rounded-2xl text-[1.1rem] xl:text-[1.25rem] transition-all shadow-sm">
                    Зареєструвати команду
                </a>

            {:else if tournament.status === "active" || tournament.status === "completed"}
                <div class="{tournament.status === 'active' ? 'bg-[#CCFF00]' : 'bg-white border border-[#E5E7EB] shadow-sm'} px-6 xl:px-10 py-5 xl:py-8 rounded-[1.25rem] flex flex-col sm:flex-row items-start sm:items-center gap-3 xl:gap-16 w-full lg:min-w-[450px]">
                    <span class="text-[0.95rem] xl:text-[1.25rem] font-semibold text-[#191F00] whitespace-nowrap">Проведення турніру:</span>
                    <span class="text-[0.95rem] xl:text-[1.25rem] font-regular text-[#191F00] whitespace-nowrap">{tournament.startDate} - {tournament.endDate}</span>
                </div>
            {/if}
        </div>
    </section>

    <div class="grid grid-cols-1 md:grid-cols-2 xl:grid-cols-3 gap-12 lg:gap-16 xl:gap-20">
        <div class="md:col-span-1 xl:col-span-2 max-w-2xl">
            <h2 class="text-2xl md:text-3xl xl:text-[2.5rem] font-bold leading-tight mb-6 lg:mb-7">Правила</h2>
            <ol class="space-y-4 list-decimal pl-6 text-[0.95rem] xl:text-[1.25rem] leading-relaxed">
                {#each tournament.rules as rule}
                    <li class="pl-2">{rule}</li>
                {/each}
            </ol>
        </div>

        <div class="flex flex-col gap-6 items-start xl:items-end w-full">
            {#if tournament.status === "registration"}
                <div class="border border-[#E5E7EB] rounded-[1.5rem] p-6 xl:p-10 bg-white shadow-sm w-full xl:max-w-[650px]">
                    <div class="flex justify-between items-center mb-5 gap-4">
                        <span class="text-[0.9rem] xl:text-[1.1rem] opacity-50">Реєстрація можлива:</span>
                        <span class="text-[0.9rem] xl:text-[1.1rem] font-regular text-right">{tournament.startRegDate} - {tournament.endRegDate}</span>
                    </div>
                    <div class="flex justify-between items-center gap-4">
                        <span class="text-[0.9rem] xl:text-[1.1rem] opacity-50">Максимальна кількість команд:</span>
                        <span class="text-[0.9rem] xl:text-[1.1rem] font-regular">{tournament.teamsMaxNum}</span>
                    </div>
                </div>

                <div class="border border-[#E5E7EB] rounded-[1.5rem] p-6 xl:p-10 bg-white shadow-sm flex justify-between items-center w-full xl:max-w-[350px]">
                    <span class="text-[0.9rem] xl:text-[1.1rem] opacity-50">Зареєстрованих команд:</span>
                    <span class="text-[0.9rem] xl:text-[1.1rem] font-regular">{tournament.teamsNum}</span>
                </div>
            {:else}
                <div class="border border-[#E5E7EB] rounded-[1.5rem] p-6 lg:p-8 xl:p-10 bg-white shadow-sm w-full xl:max-w-[450px] min-w-fit">
                    <div class="flex justify-between items-center mb-6 gap-4">
                        <span class="text-[1rem] xl:text-[1.15rem] font-semibold whitespace-nowrap">Зареєстрованих команд:</span>
                        <span class="text-[1.1rem] xl:text-[1.25rem] font-semibold">{tournament.teamsNum}</span>
                    </div>

                    <div class="space-y-4">
                        {#each tournament.registeredTeams as team}
                            <div class="flex justify-between items-center gap-6">
                                <span class="text-[0.95rem] xl:text-[1.1rem] opacity-80">{team.name}</span>
                                <span class="text-[0.85rem] xl:text-[1rem] opacity-40 whitespace-nowrap">{team.count} учасники</span>
                            </div>
                        {/each}
                    </div>
                    <div class="mt-6 flex justify-center gap-1 opacity-30 italic font-bold text-lg">...</div>
                </div>
            {/if}
        </div>
    </div>

    {#if tournament.status === "active"}
        <div class="bg-[#191F00] rounded-[1.5rem] xl:rounded-[2rem] p-7 lg:p-12 xl:p-16 mt-14 flex flex-col lg:flex-row justify-between items-start lg:items-center text-white gap-8 lg:gap-10">
            <div class="w-full lg:w-auto">
                <h3 class="text-lg xl:text-[1.6rem] font-bold mb-6 lg:mb-8">Раунд 1: {tournament.round}</h3>
                <div class="flex flex-col sm:flex-row gap-4 xl:gap-12">
                    <a href="/tournaments/tourtask" class="text-center bg-white text-[#191F00] font-bold py-3 px-8 rounded-xl text-[1rem] xl:text-[1.1rem] hover:bg-[#CBCBCB] transition-all">
                        Подивитися завдання
                    </a>
                    <a href="/task_submission" class="text-center bg-[#CCFF00] text-[#191F00] font-bold py-3 px-8 rounded-xl text-[1rem] xl:text-[1.1rem] hover:bg-[#A9D207] transition-all">
                        Здати роботу
                    </a>
                </div>
            </div>

            <div class="w-full lg:text-right">
                <p class="text-[0.9rem] xl:text-[1.1rem] opacity-80 mb-3 lg:mb-5">Часу залишилося</p>
                <p class="text-3xl xl:text-[2.5rem] font-bold tracking-wider leading-none">{tournament.time}</p>
            </div>
        </div>
    {/if}

    {#if tournament.status === "completed"}
        <div class="flex justify-center mt-10">
            <a href="/tournaments/results" class="w-full sm:w-auto text-center bg-[#CCFF00] hover:bg-[#A9D207] text-[#191F00] font-bold py-3.5 px-12 rounded-2xl text-[1.1rem] xl:text-[1.25rem] transition-all">
                Перейти до результатів
            </a>
        </div>
    {/if}

</main>