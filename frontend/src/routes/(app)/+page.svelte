<script>
	import TournamentCard from "../../components/TournamentCard.svelte";

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
    {title : "Hackathon Ukraine 2026", desc : "Національний хакатон для студентських команд. Створюйте інноваційні рішення за 48 годин.", max_teams : 50, registered_teams: 24, date : new Date(2026, 3, 15, 12, 0, 0, 0)},
    {title : "Hackathon Ukraine 2026", desc : "Національний хакатон для студентських команд. Створюйте інноваційні рішення за 48 годин.", max_teams : 50, registered_teams: 24, date : new Date(2026, 3, 15, 12, 0, 0, 0)},
  ]
</script>


<section class = "relative flex min-h-[90%] bg-[#191F00]">

    <div class="absolute top-0 right-0 w-1/4 h-full bg-[#CCFF00] z-10 hidden lg:block"></div>
    <div class="relative z-20 flex flex-col justify-center pl-10.5 lg:pl-21 text-white">
        <h1 class="text-[3.25rem] leading-tight font-black mb-12 mt-7">Побувайте у <span class = "text-[#CCFF00]">змаганнях</span><br> нового рівня</h1>
        <p class="mb-16 text-gray-300 text-[1rem]">Hucky - платформа для проведення командних програмістських <br> турнірів    </p>
        <div class="flex gap-12">
            <a href = "/registration" class = "text-[#191F00] text-[0.9rem] font-bold bg-linear-to-br from-[#BCEB01] to-[#EEFF00] rounded py-3.5 px-12 hover:bg-linear-to-tl">Розпочати</a>


            <a href = "/tournaments" class = "text-white text-[0.9rem]  bg-[#191F00] font-bold py-3.5 px-12 rounded ring-1 hover:ring-2 transition-colors duration-300 ease-in">Переглянути турніри</a>
        </div>
  </div>

    <img src = "girl_laptop.png" class = "absolute bottom-0 right-0 max-h-[100%] z-30 object-contain h-auto w-auto transform  hidden lg:block">

</section>


<div class = "flex flex-col items-center py-20 px-5 gap-12">
    <div class = "flex flex-col items-center gap-4">
        <h1 class = "font-bold text-3xl">Все для проведення турнірів</h1>
        <p class = "">Повний набір інструментів для організаторів, команд та журі</p>
    </div>



    <div class = "grid grid-cols-1 lg:grid-cols-4 gap-6">
        {#each features as feature}
        <div class = "p-8 flex flex-col ring-[1.5px] rounded-xl items-start gap-4 min-w-70">
           <img src = {feature.icon} class = "bg-[#191F00] p-[0.625rem] rounded-[0.625rem] w-9 h-9">
           <h3 class ="font-semibold text-[0.85rem]">{feature.title} </h3>
           <p class = "text-[0.85rem] leading-tight"> {feature.desc}</p>
        </div>
        {/each}
    </div>


</div>



<div class = "flex flex-col items-center gap-16">
    <h1 class = "font-bold text-3xl">Майбутні турніри</h1>
    <div class = "grid grid-cols-1 lg:grid-cols-3">
        {#each tournaments as tournament}
        <div>
        <TournamentCard></TournamentCard>
        </div>
        {/each}
    </div>

</div>