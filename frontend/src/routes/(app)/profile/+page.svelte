<script>
    import StateTag from "../../../components/StateTag.svelte";
    import TournamentCard from "../../../components/TournamentCard.svelte";
    import CardShell from "../../../components/CardShell.svelte";
    // Ролі: "none" (без ролі), "participant" (учасник), "jury" (журі), "admin" (адмін)
    let role = "jury";
    let timeLeft = "30:23:50:23";
    let defaultAvatar = "/icons/avatar.svg";
    let hoverAvatar = "/icons/avatar_change.svg";
    let currentAvatar = defaultAvatar;
    let user = {
        name: "Michael de Santa",
        email: "michael@example.com",
        emptyRole: "Створіть та зареєструйте команду на турнір, щоб отримати більше можливостей. Зареєструвати команду може лише один учасник, який буде її капітаном.",
        teamName: "Code Warriors",
        tournamentName: "Hackathon Ukraine 2026",
        schoolName: "PL KPI lyceum",
        round: "Раунд 1: MVP Development",
        teamMembers: [
            { name: "Michael de Santa", email: "michael@example.com", isCaptain: true },
            { name: "Jijy Maria Olegivna", email: "maria@example.com", isCaptain: false },
            { name: "Fanta Anton Antonovych ", email: "anton@example.com", isCaptain: false }
        ],
        createdTournaments: [
            { id: 1, title: "Hackathon Ukraine 2026", description: "Національний хакатон для студентських команд.", start_date: "2026-03-15", current_state: "registration", rounds: 5, registered_teams: 24, max_teams: 50 },
            { id: 2, title: "Hackathon Ukraine 2026", description: "Національний хакатон для студентських команд.", start_date: "2026-03-15", current_state: "active", rounds: 5, registered_teams: 24, max_teams: 50 },
            { id: 3, title: "Hackathon Ukraine 2026", description: "Національний хакатон для студентських команд.", start_date: "2026-03-15", current_state: "registration", rounds: 5, registered_teams: 24, max_teams: 50 },
            { id: 4, title: "Hackathon Ukraine 2026", description: "Національний хакатон для студентських команд.", start_date: "2026-03-15", current_state: "draft", rounds: 5, registered_teams: 24, max_teams: 50 }
        ],
        assignedTasks: [
            { team: "Code Warriors", tournament: "Hackathon Ukraine 2026", status: "pending", id: "task-1", links: { github: "#", video: "#", live: "#" } },
            { team: "Binary Bandits", tournament: "Hackathon Ukraine 2026", status: "pending", id: "task-2", links: { github: "#", video: "#" } },
            { team: "Hard Coders", tournament: "Hackathon Ukraine 2026", status: "graded", id: "task-3", links: { github: "#", video: "#", live: "#" } }
        ],
    }
</script>

<main class="w-full px-6 md:px-16 lg:pl-28 lg:pr-30 py-10 lg:py-30 font-sans text-[#191F00] pb-20">

    <section class="flex flex-col md:flex-row gap-8 lg:gap-15 items-center md:items-start mb-12 lg:mb-20 text-center md:text-left">
        <img
                src={currentAvatar}
                alt="Profile Avatar"
                class="w-24 h-24 lg:w-35 lg:h-35 pt-0 md:pt-4 transition-all duration-500 cursor-pointer"
                on:mouseenter={() => currentAvatar = hoverAvatar}
                on:mouseleave={() => currentAvatar = defaultAvatar}
        >

        <div class="w-full pt-0 md:pt-4 flex flex-col lg:flex-row justify-between items-center md:items-start gap-6">
            <div>
                <div class="flex flex-col md:flex-row items-center gap-4 lg:gap-14 mb-6 lg:mb-10">
                    <h1 class="text-3xl lg:text-[3rem] font-bold leading-tight">{user.name}</h1>
                    {#if role != 'none'}
                        <StateTag variant={role} />
                    {/if}
                </div>
                <div class="flex flex-col md:flex-row items-center gap-2 md:gap-6">
                    <span class="text-lg lg:text-[1.4rem] font-semibold">Пошта:</span>
                    <span class="text-lg lg:text-[1.4rem] font-regular">{user.email}</span>
                </div>
                {#if role === 'none'}
                    <p class="text-base lg:text-[1.4rem] leading-relaxed font-regular text-[#696969] max-w-5xl mt-6 lg:mt-13.5">
                        {user.emptyRole}
                    </p>
                {/if}
            </div>

            {#if role === 'admin'}
                <a href="/tournament_creation" class="flex items-center gap-4 lg:gap-6 border border-[#191F00] rounded-2xl px-5 lg:px-7.5 py-3 lg:py-4 hover:ring-1 w-full md:w-auto justify-center">
                    <div class="bg-[#CCFF00] w-8 h-8 lg:w-10 lg:h-10 rounded-full flex items-center justify-center text-xl lg:text-[1.65rem] font-semibold leading-none">
                        +
                    </div>
                    <span class="text-lg lg:text-[1.5rem] font-semibold">Створити новий турнір</span>
                </a>
            {/if}
        </div>
    </section>

    {#if role === 'admin'}
        <h2 class="text-xl lg:text-[1.5rem] font-semibold mb-8 lg:mb-13 md:ml-32 lg:ml-46 text-center md:text-left">Створені вами турніри:</h2>
        <div class="grid grid-cols-1 sm:grid-cols-2 gap-8 lg:gap-12 max-w-7xl md:ml-32 lg:ml-46">
            {#each user.createdTournaments as tournament}
                <a href="/tournaments/{tournament.id}" class="block transition-transform duration-200 hover:scale-[1.01] active:scale-[0.99]">
                    <CardShell>
                        <TournamentCard {...tournament} />
                    </CardShell>
                </a>
            {/each}
        </div>
    {/if}

    {#if role === 'jury'}
        <h2 class="text-xl lg:text-[1.5rem] font-semibold mb-6 lg:mb-9.5 md:ml-32 lg:ml-46">Призначені для вас</h2>
        <div class="space-y-6 md:ml-32 lg:ml-46 max-w-5xl">
            {#each user.assignedTasks as task}
                <a href="/tournaments/results/{task.id}?status={task.status}" class="block rounded-2xl py-6 px-6 lg:px-10 border transition-all hover:scale-[1.01]
                    {task.status === 'pending' ? 'bg-[#CCFF00] border-transparent' : 'bg-white border-[#B4B4B4]'}">

                    <div class="flex flex-col sm:flex-row justify-between items-start gap-4 mb-6">
                        <div class="flex flex-col md:flex-row md:items-center gap-2 lg:gap-12.5">
                            <span class="text-lg lg:text-[1.13rem] font-bold sm:font-regular">{task.team}</span>
                            <span class="text-sm lg:text-[1.13rem] italic font-regular {task.status === 'pending' ? 'text-[#516600]' : 'text-[#696969]'}">
                                {task.tournament}
                            </span>
                        </div>
                        <span class="px-6 lg:px-8 py-1.5 rounded-full text-sm lg:text-[1rem] font-light bg-[#191F00] {task.status === 'pending' ? 'text-[#CCFF00]' : 'text-white'}">
                            {task.status === 'pending' ? 'Очікує' : 'Оцінено'}
                        </span>
                    </div>

                    <div class="flex flex-wrap gap-6 lg:gap-20 {task.status === 'pending' ? 'text-[#516600]' : 'text-[#696969]'}">
                        {#each Object.entries(task.links) as [type, url]}
                            <div class="flex items-center gap-2 lg:gap-3.5 text-sm lg:text-[1.13rem]">
                                <img src="/icons/{type === 'live' ? 'globe' : type === 'video' ? 'camera' : 'github'}_green.svg" alt={type} class="w-5 h-5 lg:w-6 lg:h-6" style={task.status === 'graded' ? 'filter: grayscale(1);' : ''}>
                                {type.charAt(0).toUpperCase() + type.slice(1)}
                            </div>
                        {/each}
                    </div>
                </a>
            {/each}
        </div>
    {/if}

    {#if role === 'participant'}
        <h2 class="text-3xl lg:text-[2.5rem] font-bold mb-8 lg:mb-13 md:ml-32 lg:ml-46">Моя команда</h2>

        <div class="border border-[#B4B4B4] rounded-2xl px-6 lg:px-8.5 py-6 mb-8 lg:mb-12 md:ml-32 lg:ml-46 max-w-5xl">
            <div class="flex flex-col sm:flex-row items-center sm:items-start gap-6 lg:gap-10 text-center sm:text-left">
                <img src="/icons/team_profile.svg" alt="Team" class="w-16 h-16 lg:w-22.5 lg:h-22.5">
                <div class="pt-0 lg:pt-2">
                    <h3 class="text-2xl lg:text-[1.9rem] font-bold leading-tight mb-2">{user.teamName}</h3>
                    <p class="text-[#696969] text-base lg:text-[1.13rem] underline decoration-solid">{user.tournamentName}</p>
                </div>
            </div>
            <div class="flex items-center justify-center sm:justify-start gap-3 mt-6 lg:mt-9 text-[#696969]">
                <img src="/icons/school.svg" alt="School" class="w-5 h-5 lg:w-6 lg:h-6">
                <span class="text-base lg:text-[1.2rem]">{user.schoolName}</span>
            </div>
        </div>

        <div class="border border-[#B4B4B4] rounded-2xl p-6 lg:p-9 mb-8 lg:mb-12 md:ml-32 lg:ml-46 max-w-5xl">
            <h4 class="text-lg lg:text-[1.3rem] font-semibold mb-6 lg:mb-9">Учасники ({user.teamMembers.length})</h4>
            <div class="space-y-4">
                {#each user.teamMembers as member}
                    <div class="bg-[#CCFF00] rounded-xl py-4 px-5 lg:py-4.5 lg:px-7.5 flex flex-col sm:flex-row justify-between items-start sm:items-center gap-4">
                        <div>
                            <p class="font-regular text-base lg:text-[1.13rem] mb-2 sm:mb-4">{member.name}</p>
                            <div class="flex items-center gap-2 lg:gap-3.5 text-[#191F00]">
                                <img src="/icons/mail_dark.svg" alt="Email" class="w-5 h-auto">
                                <span class="text-sm lg:text-[1.13rem] text-[#516600] break-all">{member.email}</span>
                            </div>
                        </div>
                        {#if member.isCaptain}
                            <span class="bg-[#191F00] text-[#CCFF00] text-xs lg:text-[1rem] px-5 lg:px-7 py-1.5 rounded-full font-light">Капітан</span>
                        {/if}
                    </div>
                {/each}
            </div>
        </div>

        <div class="bg-[#191F00] text-white rounded-2xl py-8 px-6 lg:py-15 lg:px-13 md:ml-32 lg:ml-46 flex flex-col lg:flex-row justify-between items-center max-w-5xl gap-8">
            <div class="w-full text-center lg:text-left">
                <h3 class="text-xl lg:text-[1.5rem] font-bold mb-6 lg:mb-12">{user.round}</h3>
                <div class="flex flex-col sm:flex-row gap-4 lg:gap-9 justify-center lg:justify-start">
                    <a href="/tournaments/tourtask" class="bg-white text-[#191F00] text-base lg:text-[1.13rem] px-6 lg:px-7.5 py-3 lg:py-4 rounded-xl font-bold hover:bg-gray-200 transition">Подивитися завдання</a>
                    <a href="/task_submission" class="bg-gradient-to-r from-[#BCEB01] to-[#EEFF00] text-[#191F00] text-base lg:text-[1.13rem] px-6 lg:px-8 py-3 lg:py-4 rounded-xl font-bold hover:brightness-90 transition-all">Здати роботу</a>
                </div>
            </div>
            <div class="text-center lg:text-right">
                <p class="text-sm lg:text-[1.13rem] opacity-70 mb-2 lg:mb-5">Часу залишилося</p>
                <p class="text-3xl lg:text-[2.5rem] font-bold tracking-widest">{timeLeft}</p>
            </div>
        </div>
    {/if}

</main>