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
        tournamentId: 1,
        currentRoundId: 1,
        schoolName: "PL KPI lyceum",
        round: "Раунд 1: MVP Development",
        teamMembers: [
            { name: "Michael de Santa", email: "michael@example.com", isCaptain: true },
            { name: "Jijy Maria Olegivna", email: "maria@example.com", isCaptain: false },
            { name: "Fanta Anton Antonovych ", email: "anton@example.com", isCaptain: false }
        ],
        createdTournaments: [
            {
                id: 1,
                title: "Hackathon Ukraine 2026",
                description: "Національний хакатон для студентських команд. Створюйте інноваційні рішення за 48 годин.",
                start_date: "2026-03-15T08:00:00.000Z",
                current_state: "registration", // Відповідає StateTag
                rounds: 5,
                registered_teams: 24,
                max_teams: 50
            },
            {
                id: 2,
                title: "Hackathon Ukraine 2026",
                description: "Національний хакатон для студентських команд. Створюйте інноваційні рішення за 48 годин.",
                start_date: "2026-03-15T08:00:00.000Z",
                current_state: "active",
                rounds: 5,
                registered_teams: 24,
                max_teams: 50
            },
            {
                id: 3,
                title: "Hackathon Ukraine 2026",
                description: "Національний хакатон для студентських команд. Створюйте інноваційні рішення за 48 годин.",
                start_date: "2026-03-15T08:00:00.000Z",
                current_state: "registration",
                rounds: 5,
                registered_teams: 24,
                max_teams: 50
            },
            {
                id: 4,
                title: "Hackathon Ukraine 2026",
                description: "Національний хакатон для студентських команд. Створюйте інноваційні рішення за 48 годин.",
                start_date: "2026-03-15T08:00:00.000Z",
                current_state: "draft",
                rounds: 5,
                registered_teams: 24,
                max_teams: 50
            }
        ],

        assignedTasks: [
            {
                team: "Code Warriors",
                tournament: "Hackathon Ukraine 2026",
                status: "pending",
                id: "task-1", // Для переходу на сторінку оцінювання
                links: { github: "https://github.com/...", video: "https://youtube.com/...", live: "https://demo.com" }
            },
            {
                team: "Binary Bandits",
                tournament: "Hackathon Ukraine 2026",
                status: "pending",
                id: "task-2",
                links: { github: "https://github.com/...", video: "https://youtube.com/..." }
            },
            {
                team: "Hard Coders",
                tournament: "Hackathon Ukraine 2026",
                status: "graded",
                id: "task-3",
                links: { github: "https://github.com/...", video: "https://youtube.com/...", live: "https://demo.com" }
            }
        ],
    }

</script>


<main class="w-full pl-28 py-30 font-sans text-[#191F00] pb-20 pr-30">
    <section class="flex gap-15 items-start mb-20">
        <img
                src={currentAvatar}
                alt="Profile Avatar"
                class="w-35 h-35 pt-4 transition-all duration-500 cursor-pointer"
                on:mouseenter={() => currentAvatar = hoverAvatar}
                on:mouseleave={() => currentAvatar = defaultAvatar}
        >

        <div class="w-full pt-4 flex justify-between items-start">
            <div>
                <div class="flex items-center gap-14 mb-10">
                    <h1 class="text-[3rem] font-bold leading-tight">{user.name}</h1>
                    {#if role != 'none'}
                        <StateTag variant={role} />
                    {/if}
                </div>
                <div class="flex items-center gap-6">
                    <span class="text-[1.4rem] font-semibold">Пошта:</span>
                    <span class="text-[1.4rem] font-regular">{user.email}</span>
                </div>
                {#if role === 'none'}
                    <p class="text-[1.4rem] leading-relaxed font-regular text-[#696969] max-w-5xl mt-13.5">
                        {user.emptyRole}
                    </p>
                {/if}
            </div>

            {#if role === 'admin'}
                <a href="/tournaments/new" class="flex items-center gap-6 border border-[#191F00] rounded-2xl px-7.5 py-4 hover:ring-1">
                    <div class="bg-[#CCFF00] w-10 h-10 rounded-full flex items-center justify-center text-[1.65rem] font-semibold leading-none">
                        +
                    </div>
                    <span class="text-[1.5rem] font-semibold">Створити новий турнір</span>
                </a>
            {/if}
        </div>
    </section>


    {#if role === 'admin'}
        <h2 class="text-[1.5rem] font-semibold mb-13 ml-46">Створені вами турніри:</h2>

        <div class="grid grid-cols-2 gap-12 max-w-7xl ml-46">
            {#each user.createdTournaments as tournament}
                <a href="/tournaments/{tournament.id}" class="block transition-transform duration-200 hover:scale-[1.01] active:scale-[0.99]">
                    <CardShell>
                        <TournamentCard
                                id={tournament.id}
                                title={tournament.title}
                                description={tournament.description}
                                start_date={tournament.start_date}
                                current_state={tournament.current_state}
                                rounds={tournament.rounds}
                                registered_teams={tournament.registered_teams}
                                max_teams={tournament.max_teams}
                        />
                    </CardShell>
                </a>
            {/each}
        </div>
    {/if}


    {#if role === 'jury'}
        <h2 class="text-[1.5rem] font-semibold mb-9.5 ml-46">Призначені для вас</h2>

        <div class="space-y-6 ml-46 max-w-5xl">
            {#each user.assignedTasks as task}
                <a href="/tournaments/results/{task.id}?status={task.status}" class="block rounded-2xl py-6 px-10 border transition-all hover:scale-[1.01]
                    {task.status === 'pending' ? 'bg-[#CCFF00] border-transparent' : 'bg-white border-[#B4B4B4]'}">

                    <div class="flex justify-between items-start mb-6">
                        <div class="flex items-center gap-12.5">
                            <span class="text-[1.13rem] font-regular">{task.team}</span>
                            <span class="text-[1.13rem] italic font-regular
                                {task.status === 'pending' ? 'text-[#516600]' : 'text-[#696969]'}">
                                {task.tournament}
                            </span>
                        </div>

                        <span class="px-8 py-1.5 rounded-full text-[1rem] font-light
                            {task.status === 'pending' ? 'bg-[#191F00] text-[#CCFF00]' : 'bg-[#191F00] text-white'}">
                            {task.status === 'pending' ? 'Очікує' : 'Оцінено'}
                        </span>
                    </div>

                    <div class="flex gap-20 {task.status === 'pending' ? 'text-[#516600]' : 'text-[#696969]'}">

                        {#if task.links.github}
                            <div class="flex items-center gap-3.5 text-[1.13rem]">
                                <img src="/icons/github_green.svg" alt="GitHub" class="w-6 h-6"
                                     style={task.status === 'graded' ? 'filter: grayscale(1);' : ''}>
                                GitHub
                            </div>
                        {/if}

                        {#if task.links.video}
                            <div class="flex items-center gap-3.5 text-[1.13rem]">
                                <img src="/icons/camera_green.svg" alt="Video" class="w-6 h-6"
                                     style={task.status === 'graded' ? 'filter: grayscale(1);' : ''}>
                                Video Demo
                            </div>
                        {/if}

                        {#if task.links.live}
                            <div class="flex items-center gap-3.5 text-[1.13rem]">
                                <img src="/icons/globe_green.svg" alt="Live" class="w-6 h-6"
                                     style={task.status === 'graded' ? 'filter: grayscale(1);' : ''}>
                                Live Demo URL
                            </div>
                        {/if}
                    </div>
                </a>
            {/each}
        </div>
    {/if}


    {#if role === 'participant'}
        <h2 class="text-[2.5rem] font-bold mb-13 ml-46">Моя команда</h2>

        <div class="border border-[#B4B4B4] rounded-2xl px-8.5 py-6 mb-12 ml-46 max-w-5xl">
            <div class="flex items-start gap-10">
                <img src="/icons/team_profile.svg" alt="Team" class="w-22.5 h-22.5">

                <div class="pt-2">
                    <h3 class="text-[1.9rem] font-bold leading-tight mb-2">{user.teamName}</h3>
                    <p class="text-[#696969] text-[1.13rem] underline decoration-solid [#696969]">{user.tournamentName}</p>
                </div>
            </div>

            <div class="flex items-center gap-3 mt-9 text-[#696969]">
                <div class="flex justify-start">
                    <img src="/icons/school.svg" alt="School" class="w-6 h-6">
                </div>
                <span class="text-[1.2rem]">{user.schoolName}</span>
            </div>
        </div>

        <div class="border border-[#B4B4B4] rounded-2xl p-9 mb-12 ml-46 max-w-5xl">
            <h4 class="text-[1.3rem] font-semibold mb-9">Учасники ({user.teamMembers.length})</h4>
            <div class="space-y-4">
                {#each user.teamMembers as member}
                    <div class="bg-[#CCFF00] rounded-xl py-4.5 px-7.5 flex justify-between items-center">
                        <div>
                            <p class="font-regular text-[1.13rem] mb-4">{member.name}</p>
                            <div class="flex items-center gap-3.5 text-[#191F00]">
                                <img src="/icons/mail_dark.svg" alt="Email" class="w-6 h-auto">
                                <span class="text-[1.13rem] text-[#516600]">{member.email}</span>
                            </div>
                        </div>
                        {#if member.isCaptain}
                            <span class="bg-[#191F00] text-[#CCFF00] text-[1rem] px-7 py-1.5 rounded-full font-light">Капітан</span>
                        {/if}
                    </div>
                {/each}
            </div>
        </div>

        <div class="bg-[#191F00] text-white rounded-2xl py-15 px-13 ml-46 flex justify-between items-center max-w-5xl">
            <div>
                <h3 class="text-[1.5rem] font-bold mb-12">{user.round}</h3>
                <div class="flex gap-9">
                    <a href="/tournaments/tourtask" class="bg-white text-[#191F00] text-[1.13rem] px-7.5 py-4 rounded-xl font-bold hover:bg-gray-200 transition">Подивитися завдання</a>
                    <a href={`/tournaments/${user.tournamentId}/rounds/${user.currentRoundId}/submission`} class="bg-linear-to-r from-[#BCEB01] to-[#EEFF00] text-[#191F00] text-[1.13rem] px-8 py-4 rounded-xl font-bold hover:brightness-90 transition-all">Здати роботу</a>
                </div>
            </div>
            <div class="text-right">
                <p class="text-[1.13rem] mb-5">Часу залишилося</p>
                <p class="text-[2.5rem] font-bold">{timeLeft}</p>
            </div>
        </div>
    {/if}


</main>