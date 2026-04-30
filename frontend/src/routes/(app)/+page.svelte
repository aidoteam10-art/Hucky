<script>
    import TournamentCard from "../../components/TournamentCard.svelte";
    import Link from "../../components/Link.svelte";
    let features = [
        { title: 'Турніри', desc: 'Створюйте та керуйте багатосекундними змаганнями з гнучкими налаштуваннями', icon: '/icons/trophie.svg' },
        { title: 'Командна робота', desc: 'Реєстрація команд, взаємодія учасників та капітанів', icon: '/icons/team.svg' },
        { title: 'Оцінювання журі', desc: 'Оцінювання з автоматичним розподілом робіт між суддями', icon: '/icons/shield.svg' },
        { title: 'Лідерборд', desc: 'Автоматичний рейтинг з детальною розбивкою за категоріями', icon: '/icons/graph.svg' }
  ];
  let tournaments = [
    {title : "Hackathon Ukraine 2026", desc : "Національний хакатон для студентських команд. Створюйте інноваційні рішення за 48 годин.", max_teams : 50, registered_teams: 24, date : new Date(2026, 3, 15, 12, 0, 0, 0)},
    {title : "Hackathon Ukraine 2026", desc : "Національний хакатон для студентських команд. Створюйте інноваційні рішення за 48 годин.", max_teams : 50, registered_teams: 24, date : new Date(2026, 3, 15, 12, 0, 0, 0)},
    {title : "Hackathon Ukraine 2026", desc : "Національний хакатон для студентських команд. Створюйте інноваційні рішення за 48 годин.", max_teams : 50, registered_teams: 24, date : new Date(2026, 3, 15, 12, 0, 0, 0)},
  ]
</script>

<section class="relative flex min-h-screen lg:min-h-[90%] bg-[#191F00] overflow-hidden py-16 lg:py-0">

    <div class="absolute top-0 right-0 w-1/4 h-full bg-[#CCFF00] z-10 hidden lg:block"></div>
    <div class="relative z-20 flex flex-col items-center lg:items-start justify-center px-6 sm:px-10 lg:pl-26 text-white w-full lg:w-[60%] text-center lg:text-left">
        <h1 class="text-4xl sm:text-5xl lg:text-[5rem] leading-tight font-black mb-8 lg:mb-15 mt-7">
            Побувайте у <span class="text-[#CCFF00]">змаганнях</span><br> нового рівня
        </h1>
        <p class="mb-12 lg:mb-20 text-white tracking-wide font-light text-lg lg:text-[1.5rem]">
            Hucky - платформа для проведення командних програмістських <br class="hidden lg:block"> турнірів    
        </p>
        <div class="flex flex-col sm:flex-row gap-4 sm:gap-6 lg:gap-12 w-full sm:w-auto justify-stretch lg:justify-start">
            <div class="w-full sm:w-auto flex *:w-full">
                <Link href="/registration" variant="green" size="lg">Розпочати</Link>
            </div>
            <div class="w-full sm:w-auto flex *:w-full whitespace-nowrap">
                <Link href="/tournaments" variant="outline" size="lg">Переглянути турніри</Link>
            </div>
        </div>
  </div>

    <img src="girl_laptop.png" alt="Girl with laptop" class="absolute bottom-0 right-0 max-h-[80%] lg:max-h-full max-w-[40%] z-30 object-contain h-auto w-auto transform hidden lg:block">

</section>


<div class="flex flex-col items-center py-16 xl:py-20 px-10 gap-8 xl:gap-12 mb-10 text-center xl:text-left">
    <div class="flex flex-col items-center gap-2 text-center">
        <h1 class="font-bold text-3xl sm:text-4xl xl:text-[2.75rem] text-[#191F00]">Все для проведення турнірів</h1>
        <p class="text-lg xl:text-[1.5rem] text-[#191F00]">Повний набір інструментів для організаторів, команд та журі</p>
    </div>

    <div class="grid grid-cols-1 sm:grid-cols-2 xl:grid-cols-4 gap-4 md:gap-6 xl:gap-12 w-full px-2 sm:px-8 xl:px-24">
        {#each features as feature}
        <div class="p-4 md:p-6 xl:p-10 flex flex-col ring-2 rounded-xl items-start gap-4 h-full text-left">
           <img src={feature.icon} alt={feature.title} class="bg-[#191F00] p-1.5 md:p-2.5 rounded-[0.625rem] w-8 h-8 md:w-12 md:h-12">
           <h3 class="font-semibold text-base md:text-xl xl:text-[1.25rem]">{feature.title}</h3>
           <p class="text-sm md:text-base xl:text-[1.2rem] leading-tight">{feature.desc}</p>
        </div>
        {/each}
    </div>

</div>



<div class="flex flex-col items-center gap-10 lg:gap-16 mb-16 lg:mb-24 px-4 lg:px-0">
    <h1 class="font-bold text-3xl sm:text-4xl lg:text-[2.75rem] text-center">Майбутні турніри</h1>
    
    <div class="flex flex-row justify-between w-full px-0 sm:px-6 lg:px-10 items-center gap-2 sm:gap-6 lg:gap-12">
        
        <img src="arrow_left.svg" alt="Previous tournament" class="h-6 sm:h-8 md:h-10 shrink-0 cursor-pointer">
        
        <div class="grid grid-cols-1 sm:grid-cols-2 xl:grid-cols-3 gap-4 mx-4 sm:gap-6 w-full [&>div:nth-child(2)]:hidden sm:[&>div:nth-child(2)]:block [&>div:nth-child(3)]:hidden xl:[&>div:nth-child(3)]:block">
            {#each tournaments as tournament}
            <div class="w-full min-w-0">
                <TournamentCard 
                    title={tournament.title}
                    description={tournament.desc}
                    max_teams={tournament.max_teams}
                    registered_teams={tournament.registered_teams}
                    start_date={tournament.date.toISOString()}
                />
            </div>
            {/each}
        </div>
        
        <img src="arrow_right.svg" alt="Next tournament" class="h-6 sm:h-8 md:h-10 shrink-0 cursor-pointer">
    </div>
</div>

<hr>

<section class="pb-16 lg:pb-50 px-5 sm:px-10 lg:px-25 pt-16 lg:pt-25">
    <div class="flex flex-col items-center justify-center bg-[#191F00] py-10 lg:py-15 px-6 gap-6 rounded-3xl text-center w-full">
        <h2 class="text-white font-bold text-3xl sm:text-4xl lg:text-[2.75rem]">Готові до <span class="bg-clip-text text-transparent bg-linear-to-r from-[#BCEB01] to-[#EEFF00]">змагань</span>?</h2>
        <p class="text-white text-base sm:text-lg lg:text-[1.25rem] font-light text-center">Зареєструйте свою команду або створіть власний турнір <br class="hidden sm:block"> вже зараз.</p>
        <div class="w-full sm:w-auto flex justify-center *:w-full">
            <Link href="/registration" variant="green" size="lg">Створити акаунт</Link>
        </div>
    </div>
</section>