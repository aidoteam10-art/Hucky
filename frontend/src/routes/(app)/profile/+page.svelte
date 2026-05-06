<script>
    import StateTag from "../../../components/StateTag.svelte";
    import TournamentCard from "../../../components/TournamentCard.svelte";
    import { tournaments } from '$lib/data/tournaments.js';


    // Ролі: "none" (без ролі), "participant" (учасник), "jury" (журі), "admin" (адмін)
    let role = "admin";

    // Стани турніру: "before" (до початку), "running" (йде), "finished" (завершено)
    // Ці дані будуть приходити з бекенду (tournaments.status)
    let tournamentState = "running";

    // Логіка капітана буде визначатися з membership користувача
    let isCaptain = false;

    let timeLeft = "30:23:50:23";
    let defaultAvatar = "/icons/avatar.svg";
    let hoverAvatar = "/icons/avatar_change.svg";
    let currentAvatar = defaultAvatar;

    let rounds = [
        {
            id: 1,
            title: "Раунд 1: MVP Development",
            timeLeft: "30:23:50:23"
        },
        {
            id: 2,
            title: "Раунд 2: Final Pitch & Demo",
            timeLeft: "12:00:00:00"
        },
        {
            id: 3,
            title: "Раунд 3: Jury Evaluation",
            timeLeft: "4:00:00:00"
        }
    ];
    $: currentRoundIndex = tournamentState === 'finished' ? 1 : 0;
    $: currentRound = rounds[currentRoundIndex];

    // Основні дані користувача (users)
    let user = {
        name: "Erika Leaf",
        email: "erika37@example.com",
        emptyRole: "Зареєструйте команду на турнір або отримайте запрошення від іншого учасника, щоб отримати більше можливостей. Зареєструвати команду може лише один учасник, який буде її капітаном.",

        // Повернуто всі 4 турніри для адміна
        createdTournaments: [
            { id: 1, title: "Hackathon Ukraine 2026", description: "Національний хакатон для студентських команд.", start_date: "2026-03-15", current_state: "registration", rounds: 5, registered_teams: 24, max_teams: 50 },
            { id: 2, title: "Hackathon Ukraine 2026", description: "Національний хакатон для студентських команд.", start_date: "2026-03-15", current_state: "active", rounds: 5, registered_teams: 24, max_teams: 50 },
            { id: 3, title: "Hackathon Ukraine 2026", description: "Національний хакатон для студентських команд.", start_date: "2026-03-15", current_state: "registration", rounds: 5, registered_teams: 24, max_teams: 50 },
            { id: 4, title: "Hackathon Ukraine 2026", description: "Національний хакатон для студентських команд.", start_date: "2026-03-15", current_state: "draft", rounds: 5, registered_teams: 24, max_teams: 50 }
        ],

        // Повернуто всі 3 завдання для журі
        assignedTasks: [
            { team: "Code Warriors", tournament: "Hackathon Ukraine 2026", status: "pending", id: "task-1", links: { github: "#", video: "#", live: "#" } },
            { team: "Binary Bandits", tournament: "Hackathon Ukraine 2026", status: "pending", id: "task-2", links: { github: "#", video: "#" } },
            { team: "Hard Coders", tournament: "Hackathon Ukraine 2026", status: "graded", id: "task-3", links: { github: "#", video: "#", live: "#" } }
        ]
    };

    // Запрошення (team_invitations)
    let invitations = [
        { id: "inv-1", teamName: "Code Warriors", tournamentName: "Hackathon Ukraine 2026", organization: "PL KPI lyceum", inviterName: "Annie White", inviterEmail: "annie@example.com" },
        { id: "inv-2", teamName: "Cool Broccoli", tournamentName: "Hackathon Ukraine 2026", organization: "PL KPI lyceum", inviterName: "Max Blossom", inviterEmail: "max@example.com" }
    ];

    // Дані команди (teams)
    let team = {
        id: "team-1",
        name: "Code Warriors",
        tournamentId: "1",
        tournamentName: "Hackathon Ukraine 2026",
        organization: "PL KPI lyceum",
    };

    // Учасники команди (team_memberships + users)
    let teamMembers = [
        { id: "u-1", name: "Michael de Santa", email: "michael@example.com", role: "captain", status: "accepted" },
        { id: "u-2", name: "Erika Leaf", email: "erika37@example.com", role: "member", status: "accepted" },
        { id: "u-3", name: "Fanta Anton Antonovych", email: "anton@example.com", role: "member", status: "pending" }
    ];

    // Інформація про турнір/раунди
    let currentRound = {
        id: 1,
        title: "Раунд 1: MVP Development"
    };

    let pastSubmissions = [
        // Якщо масив не пустий, користувач побачить блок з останніми сабмітами
        { title: "Round 1: MVP Development", timeAgo: "Здано 2 години тому" }
    ];

    let pastTournaments = [
        { title: "Spring Hakathon 2024", teamDetails: "Code Warriors - Місце #1", status: "Переможець" },
        { title: "Winter Code 2023", teamDetails: "Code Warriors - Місце #5", status: "Завершено" }
    ];

    // Функції для роботи з API у майбутньому
    async function acceptInvite(inviteId) { /* Виклик POST /invitations/:token/accept */ }
    async function declineInvite(inviteId) { /* Виклик POST /invitations/:token/decline */ }
    async function kickMember(memberId) { /* Виклик DELETE /teams/:id/members/:user_id */ }
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
                    {#if role !== 'none'}
                        <StateTag variant={role} />
                    {/if}
                </div>
                <div class="flex flex-col md:flex-row items-center gap-2 md:gap-6">
                    <span class="text-lg lg:text-[1.4rem] font-semibold">Пошта:</span>
                    <span class="text-lg lg:text-[1.4rem] font-regular">{user.email}</span>
                </div>
                {#if role === 'none'}
                    <p class="text-base lg:text-[1.2rem] leading-relaxed font-regular text-[#696969] max-w-5xl mt-6 lg:mt-13.5">
                        {user.emptyRole}
                    </p>
                {/if}
            </div>

            {#if role === 'admin'}
                <a href="/tournaments/new" class="flex items-center gap-4 lg:gap-6 border border-[#191F00] rounded-2xl px-5 lg:px-7.5 py-3 lg:py-4 hover:ring-1 w-full md:w-auto justify-center">
                    <div class="bg-[#CCFF00] w-8 h-8 lg:w-10 lg:h-10 rounded-full flex items-center justify-center text-xl lg:text-[1.65rem] font-semibold leading-none">
                        +
                    </div>
                    <span class="text-lg lg:text-[1.5rem] font-semibold">Створити новий турнір</span>
                </a>
            {/if}
        </div>
    </section>

    <!-- БЛОК ЗАПРОШЕНЬ -->
    {#if role === 'none' && invitations.length > 0}
        <div class="md:ml-32 lg:ml-46 max-w-5xl mb-12">
            <h2 class="text-xl lg:text-[1.3rem] font-semibold mb-6 flex items-center gap-3">
                <img src="/icons/invitations.svg" alt="Запрошення" class="w-5 h-5">
                Запрошення до команд
                <span class="bg-[#CCFF00] text-[#191F00] text-sm font-bold px-2.5 py-0.5 rounded-md">{invitations.length}</span>
            </h2>

            <div class="space-y-6">
                {#each invitations as invite}
                    <div class="border border-[#B4B4B4] rounded-[1.2rem] p-4 lg:p-8 flex flex-col gap-8 transition-all hover:shadow-md bg-white">

                        <!-- Інформація про команду та турнір -->
                        <div class="space-y-3">
                            <div class="flex items-center gap-3">
                                <img src="/icons/team_gray.svg" alt="Team" class="w-6 h-6">
                                <h3 class="text-xl lg:text-[1.5rem] font-bold text-[#191F00]">{invite.teamName}</h3>
                            </div>

                            <div class="flex items-center gap-3">
                                <img src="/icons/trophy_gray.svg" alt="Tournament" class="w-5 h-5">
                                <p class="text-[#696969] text-base lg:text-[1.1rem] underline decoration-solid">{invite.tournamentName}</p>
                            </div>

                            <div class="flex items-center gap-3">
                                <img src="/icons/school_gray.svg" alt="Organization" class="w-5 h-5">
                                <span class="text-[#696969] text-base lg:text-[1.1rem]">{invite.organization}</span>
                            </div>
                        </div>

                        <!-- Сіра картка запрошувача -->
                        <div class="bg-[#F4F4F4]/50 border border-[#D9D9D9] rounded-[1.2rem] p-4 lg:p-5">
                            <div class="flex flex-col gap-4">
                                <div class="flex items-center gap-2 text-lg lg:text-[1.2rem]">
                                    <span class="text-[#696969]">Запрошує:</span>
                                    <span class="font-semibold text-[#191F00]">{invite.inviterName}</span>
                                </div>

                                <div class="space-y-3">
                                    <div class="flex items-center gap-3 text-[#696969]">
                                        <img src="/icons/mail_green.svg" alt="Email" class="w-5 h-5">
                                        <span class="text-base">{invite.inviterEmail}</span>
                                    </div>
                                    <div class="flex items-center gap-3 text-[#696969]">
                                        <img src="/icons/telegram_green.svg" alt="Telegram" class="w-5 h-5">
                                        <span class="text-base">@{invite.inviterName.toLowerCase().replace(' ', '_')}</span>
                                    </div>
                                </div>
                            </div>
                        </div>

                        <!-- Кнопки дій -->
                        <div class="flex flex-col sm:flex-row gap-5">
                            <button
                                    on:click={() => acceptInvite(invite.id)}
                                    class="flex-1 bg-[#CCFF00] text-[#191F00] text-xl font-bold py-4 rounded-2xl hover:brightness-95 transition flex justify-center items-center gap-3 shadow-xs"
                            >
                                <span class="text-2xl">✓</span> Прийняти
                            </button>
                            <button
                                    on:click={() => declineInvite(invite.id)}
                                    class="flex-1 bg-white border border-[#D9D9D9] text-[#191F00] text-xl font-bold py-4 rounded-2xl hover:bg-gray-50 transition flex justify-center items-center gap-3"
                            >
                                <span class="text-2xl">✕</span> Відхилити
                            </button>
                        </div>
                    </div>
                {/each}
            </div>
        </div>
    {/if}

    <!-- БЛОК АДМІНА -->
    <!-- БЛОК АДМІНА -->
    {#if role === 'admin'}
        <h2 class="text-xl lg:text-[1.5rem] font-semibold mb-8 lg:mb-13 md:ml-32 lg:ml-46 text-center md:text-left">
            Створені вами турніри:
        </h2>
        <div class="grid grid-cols-1 sm:grid-cols-2 gap-8 lg:gap-12 max-w-7xl md:ml-32 lg:ml-46">
            {#each user.createdTournaments as tournament}
                <TournamentCard
                        variant="grey"
                        current_state={tournament.current_state}
                        title={tournament.title}
                        description={tournament.description}
                        start_date={tournament.start_date}
                        rounds={tournament.rounds}
                        max_teams={tournament.max_teams}
                        registered_teams={tournament.registered_teams}
                        id={tournament.id}
                />
            {/each}
        </div>
    {/if}

    <!-- БЛОК ЖУРІ -->
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

    <!-- БЛОК УЧАСНИКА ТА КАПІТАНА -->
    {#if role === 'participant'}
        <h2 class="text-3xl lg:text-[2.5rem] font-bold mb-8 lg:mb-13 md:ml-32 lg:ml-46">Моя команда</h2>

        <div class="border border-[#B4B4B4] rounded-2xl px-6 lg:px-8.5 py-6 mb-8 lg:mb-12 md:ml-32 lg:ml-46 max-w-5xl">
            <div class="flex flex-col sm:flex-row items-center sm:items-start gap-6 lg:gap-10 text-center sm:text-left">
                <img src="/icons/team_profile.svg" alt="Team" class="w-16 h-16 lg:w-22.5 lg:h-22.5">
                <div class="pt-0 lg:pt-2">
                    <h3 class="text-2xl lg:text-[1.9rem] font-bold leading-tight mb-2">{team.name}</h3>
                    <p class="text-[#696969] text-base lg:text-[1.13rem] underline decoration-solid">{team.tournamentName}</p>
                </div>
            </div>
            <div class="flex items-center justify-center sm:justify-start gap-3 mt-6 lg:mt-9 text-[#696969]">
                <img src="/icons/school.svg" alt="School" class="w-5 h-5 lg:w-6 lg:h-6">
                <span class="text-base lg:text-[1.2rem]">{team.organization}</span>
            </div>
        </div>

        <div class="border border-[#B4B4B4] rounded-2xl p-6 lg:p-9 mb-8 lg:mb-12 md:ml-32 lg:ml-46 max-w-5xl">
            <h4 class="text-lg lg:text-[1.3rem] font-semibold mb-6 lg:mb-9">Учасники ({teamMembers.length})</h4>
            <div class="space-y-4">
                {#each teamMembers as member}
                    <div class="rounded-xl py-4 px-5 lg:py-4.5 lg:px-7.5 flex flex-col sm:flex-row justify-between items-start sm:items-center gap-4 border transition-all
                    {member.status === 'pending' && tournamentState === 'before'
                        ? 'bg-white border-[#B4B4B4] text-[#696969]'
                        : 'bg-[#CCFF00] border-transparent text-[#191F00]'}">
                        <div>
                            <p class="font-regular text-base lg:text-[1.13rem] mb-2 sm:mb-4">{member.name}</p>
                            <div class="flex items-center gap-2 lg:gap-3.5">
                                <img src="/icons/mail_dark.svg" alt="Email" class="w-5 h-auto"
                                     style={member.status === 'pending' && tournamentState === 'before' ? 'filter: invert(45%) sepia(0%) saturate(0%) hue-rotate(145deg) brightness(90%) contrast(85%);' : ''}>
                                <span class="text-sm lg:text-[1.13rem] break-all
                                {member.status === 'pending' && tournamentState === 'before' ? 'text-[#696969]' : 'text-[#516600]'}">
                                    {member.email}
                                </span>
                            </div>
                        </div>
                        <div class="flex flex-col sm:flex-row items-center gap-4 w-full sm:w-auto">
                            {#if member.role === 'captain'}
                                <span class="bg-[#191F00] text-[#CCFF00] text-xs lg:text-[1rem] px-5 lg:px-7 py-1.5 rounded-full font-light w-full sm:w-auto text-center">Капітан</span>
                            {/if}
                            {#if member.status === 'pending' && tournamentState === 'before'}
                                <div class="flex items-center gap-2 bg-[#FFEDDD] px-4 py-1.5 rounded-full border border-[#FF7700]">
                                    <img src="/icons/waiting.svg" alt="Waiting" class="w-4 h-4">
                                    <span class="text-sm font-semibold text-[#FF7700]">Очікується</span>
                                </div>
                            {/if}
                        </div>
                    </div>
                {/each}
            </div>
        </div>

        {#if tournamentState === 'running' || tournamentState === 'finished'}
            <div class="bg-[#191F00] text-white rounded-2xl py-8 px-6 lg:py-15 lg:px-13 mb-8 lg:mb-12 md:ml-32 lg:ml-46 flex flex-col lg:flex-row justify-between items-center max-w-5xl gap-8">
                <div class="w-full text-center lg:text-left">
                    <h3 class="text-xl lg:text-[1.5rem] font-bold mb-6 lg:mb-12">
                        {currentRound.title}
                    </h3>

                    <div class="flex flex-col sm:flex-row gap-4 lg:gap-6 justify-center lg:justify-start">
                        <a href="/tournaments/tourtask" class="bg-white text-[#191F00] text-base lg:text-[1.13rem] px-6 lg:px-7.5 py-3 lg:py-4 rounded-xl font-bold hover:bg-gray-200 transition text-center">Подивитися завдання</a>

                        {#if isCaptain}
                            <a href={`/tournaments/${team.tournamentId}/rounds/${currentRound.id}/submission`} class="bg-linear-to-r from-[#BCEB01] to-[#EEFF00] text-[#191F00] text-base lg:text-[1.13rem] px-6 lg:px-8 py-3 lg:py-4 rounded-xl font-bold hover:brightness-90 transition-all text-center">
                                Здати роботу
                            </a>
                        {/if}
                    </div>
                </div>

                <div class="text-center lg:text-right">
                    <p class="text-sm lg:text-[1.13rem] opacity-70 mb-2 lg:mb-5">Часу залишилося</p>
                    <p class="text-3xl lg:text-[2.5rem] font-bold tracking-widest">
                        {currentRound.timeLeft}
                    </p>
                </div>
            </div>
        {/if}

        {#if tournamentState === 'finished' && pastSubmissions.length > 0}
            <div class="border border-[#B4B4B4] rounded-2xl p-6 lg:p-9 mb-8 lg:mb-12 md:ml-32 lg:ml-46 max-w-5xl">
                <h4 class="text-lg lg:text-[1.3rem] font-semibold mb-6 flex items-center gap-3">
                    <img src="/icons/last_submits.svg" alt="Doc" class="w-5 h-5"> Останні сабміти
                </h4>
                <div class="space-y-4">
                    {#each pastSubmissions as sub}
                        <div class="bg-gray-50 border border-gray-200 rounded-xl py-4 px-5 flex justify-between items-center">
                            <div>
                                <p class="font-bold text-base lg:text-[1.13rem]">{sub.title}</p>
                                <p class="text-sm text-[#696969] mt-1">{sub.timeAgo}</p>
                            </div>
                            <span class="px-5 py-1.5 border border-[#B4B4B4] text-[#696969] rounded-full text-sm font-medium bg-white">Здано</span>
                        </div>
                    {/each}
                </div>
            </div>
        {/if}

        <!-- Історія турнірів -->
        {#if pastTournaments.length > 0}
            <div class="border border-[#B4B4B4] rounded-2xl p-6 lg:p-9 md:ml-32 lg:ml-46 max-w-5xl">
                <h4 class="text-lg lg:text-[1.3rem] font-semibold mb-6 flex items-center gap-3">
                    <img src="/icons/history.svg" alt="Trophy" class="w-5 h-5"> Історія турнірів
                </h4>
                <div class="space-y-4">
                    {#each pastTournaments as history}
                        <div class="bg-gray-50 border border-[#B4B4B4] rounded-xl py-4 px-5 flex flex-col sm:flex-row justify-between items-start sm:items-center gap-4">
                            <div>
                                <p class="font-semibold text-base lg:text-[1.13rem]">{history.title}</p>
                                <p class="text-sm text-[#696969] mt-1">{history.teamDetails}</p>
                            </div>
                            <span class="px-5 py-1.5 rounded-full text-sm font-semibold {history.status === 'Переможець' ? 'bg-[#CCFF00] text-[#191F00]' : 'bg-white border border-[#B4B4B4] text-[#696969]'}">
                                {history.status}
                            </span>
                        </div>
                    {/each}
                </div>
            </div>
        {/if}
    {/if}

</main>