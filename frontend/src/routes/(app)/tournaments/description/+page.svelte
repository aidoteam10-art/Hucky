<script>
    import StateTag from "../../../../components/StateTag.svelte";
    let current_state = "registration";

    let tournament = {
        name: "Hackathon Ukraine 2026",
        status: current_state, // "registration" | "active" | "completed"
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

<main class = "w-full pl-16 pr-38 py-16 font-sans text-[#191F00]">
    <section class="flex justify-between items-end mb-20">
        <div class="max-w-2.2xl">
            <div class="flex items-center gap-9 mb-7">
                <h2 class="text-[2.5rem] font-bold leading-tight">{tournament.name}</h2>
                <StateTag variant={current_state} />
            </div>
            <p class = "text-[1.25rem] w-3xl">{tournament.description}</p>
        </div>
        <div class="flex flex-col items-end">
            {#if tournament.status === "registration"}
                <button class="bg-[#CCFF00] hover:bg-[#A9D207] text-[#191F00] font-bold py-3.5 px-10 rounded-2xl text-[1.25rem] transition-all shadow-sm">
                    Зареєструвати команду
                </button>

            {:else if tournament.status === "active"}
                <div class="bg-[#CCFF00] px-10 py-8 rounded-[1.25rem] flex items-center gap-16 min-w-[450px]">
                    <span class="text-[1.25rem] font-semibold text-[#191F00]">Проведення турніру:</span>
                    <span class="text-[1.25rem] font-regular text-[#191F00]">{tournament.startDate} - {tournament.endDate}</span>
                </div>

            {:else if tournament.status === "completed"}
                <div class="bg-white border border-[#E5E7EB] px-10 py-8 rounded-[1.25rem] flex items-center gap-16 min-w-[450px] shadow-sm">
                    <span class="text-[1.25rem] font-semibold text-[#191F00]">Проведення турніру:</span>
                    <span class="text-[1.25rem] font-regular text-[#191F00]">{tournament.startDate} - {tournament.endDate}</span>
                </div>
            {/if}
        </div>
    </section>

    <div class="grid grid-cols-1 lg:grid-cols-3 gap-20">
        <div class="lg:col-span-2 max-w-2xl">
            <h2 class="text-[2.5rem] font-bold leading-tight mb-7">Правила</h2>
            <ol class="space-y-3 list-decimal pl-6 text-[1.25rem]">
                {#each tournament.rules as rule}
                    <li class="pl-2">{rule}</li>
                {/each}
            </ol>
        </div>

        <div class="flex flex-col gap-6 items-end">
            {#if tournament.status === "registration"}
                <div class="border border-[#E5E7EB] rounded-[1.5rem] p-10 bg-white shadow-sm w-full max-w-[650px]">
                    <div class="flex justify-between items-center mb-6">
                        <span class="text-[1.1rem] opacity-50">Реєстрація можлива:</span>
                        <span class="text-[1.1rem] font-regular">{tournament.startRegDate} - {tournament.endRegDate}</span>
                    </div>
                    <div class="flex justify-between items-center">
                        <span class="text-[1.1rem] opacity-50">Максимальна кількість команд:</span>
                        <span class="text-[1.1rem] font-regular">{tournament.teamsMaxNum}</span>
                    </div>
                </div>

                <div class="border border-[#E5E7EB] rounded-[1.5rem] p-10 bg-white shadow-sm flex justify-between items-center w-full max-w-[350px]">
                    <span class="text-[1.1rem] opacity-50">Зареєстрованих команд:</span>
                    <span class="text-[1.1rem] font-regular">{tournament.teamsNum}</span>
                </div>
            {:else}
                <div class="border border-[#E5E7EB] rounded-[1.5rem] p-10 bg-white shadow-sm w-full max-w-[450px]">
                    <div class="flex justify-between items-center mb-8">
                        <span class="text-[1.15rem] font-semibold">Зареєстрованих команд:</span>
                        <span class="text-[1.25rem] font-semibold">{tournament.teamsNum}</span>
                    </div>

                    <div class="space-y-5">
                        {#each tournament.registeredTeams as team}
                            <div class="flex justify-between items-center">
                                <span class="text-[1.1rem] opacity-80">{team.name}</span>
                                <span class="text-[1rem] opacity-40">{team.count} учасники</span>
                            </div>
                        {/each}
                    </div>

                    <div class="mt-8 flex justify-center gap-1 opacity-30 italic font-bold text-xl">...</div>
                </div>
            {/if}
        </div>
    </div>

    {#if tournament.status === "active"}
        <div class="bg-[#191F00] rounded-[2rem] p-16 mt-14 flex justify-between items-center text-white">
            <div>
                <h3 class="text-[1.6rem] font-bold mb-8">Раунд 1: {tournament.round}</h3>
                <div class="flex gap-12">
                    <button class="bg-white text-[#191F00] font-bold py-3.5 px-8 rounded-xl text-[1.1rem] hover:bg-[#CBCBCB] transition-all">
                        Подивитися завдання
                    </button>
                    <button class="bg-[#CCFF00] text-[#191F00] font-bold py-3.5 px-8 rounded-xl text-[1.1rem] hover:bg-[#A9D207] transition-all">
                        Здати роботу
                    </button>
                </div>
            </div>

            <div class="text-right">
                <p class="text-[1.1rem] opacity-80 mb-5">Часу залишилося</p>
                <p class="text-[2.5rem] font-bold tracking-wider leading-none">{tournament.time}</p>
            </div>
        </div>
    {/if}

    {#if tournament.status === "completed"}
        <div class="flex justify-center mt-10">
            <a href="/tournaments/results" class="bg-[#CCFF00] hover:bg-[#A9D207] text-[#191F00] font-bold py-4 px-12 rounded-2xl text-[1.25rem] transition-all">
                Перейти до результатів
            </a>
        </div>
    {/if}


</main>