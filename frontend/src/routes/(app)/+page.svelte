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


<section class = "relative flex min-h-[90%] bg-[#191F00]">

    <div class="absolute top-0 right-0 w-1/4 h-full bg-[#CCFF00] z-10 hidden xl:block"></div>
    <div class="relative z-20 flex flex-col justify-center pl-13 xl:pl-26 text-white">
        <h1 class="text-[5rem] leading-tight font-black mb-15 mt-7">Побувайте у <span class = "text-[#CCFF00]">змаганнях</span><br> нового рівня</h1>
        <p class="mb-20 text-white tracking-wide font-light text-[1.5rem]">Hucky - платформа для проведення командних програмістських <br> турнірів    </p>
        <div class="flex gap-12">
            <Link href="/registration" variant="green" size="lg">Розпочати</Link>
            <Link href="/tournaments" variant="outline" size="lg">Переглянути турніри</Link>
        </div>
  </div>

    <img src="girl_laptop.png" alt="Girl with laptop" class="absolute bottom-0 right-0 max-h-full z-30 object-contain h-auto w-auto transform hidden xl:block">

</section>


<div class = "flex flex-col items-center py-20 px-5 gap-12 mb-10">
    <div class = "flex flex-col items-center gap-2">
        <h1 class = "font-bold text-[2.75rem] text-[#191F00]">Все для проведення турнірів</h1>
        <p class = "text-[1.5rem] text-[#191F00]">Повний набір інструментів для організаторів, команд та журі</p>
    </div>



    <div class="grid grid-cols-1 sm:grid-cols-2 lg:grid-cols-4 gap-4 md:gap-6 xl:gap-12 w-full px-8 md:px-12 xl:px-24">
        {#each features as feature}
        <div class="p-10 flex flex-col ring-2 rounded-xl items-start gap-4 h-full">
           <img src={feature.icon} alt={feature.title} class="bg-[#191F00] p-2.5 rounded-[0.625rem] w-12 h-12">
           <h3 class="font-semibold text-[1.25rem]">{feature.title}</h3>
           <p class="text-[1.2rem] leading-tight">{feature.desc}</p>
        </div>
        {/each}
    </div>


</div>



<div class = "flex flex-col items-center gap-16 mb-24">
    <h1 class = "font-bold text-[2.75rem]">Майбутні турніри</h1>
    <div class = "flex justify-between px-10 items-center">
        <img src="arrow_left.svg" alt="Previous tournament" class="mr-8 h-10">
        <div class = "grid grid-cols-1 lg:grid-cols-3 gap-5">
            {#each tournaments as tournament}
            <div>
            <TournamentCard></TournamentCard>
            </div>
            {/each}
        </div>
        <img src="arrow_right.svg" alt="Next tournament" class="ml-8 h-10">
    </div>
</div>

<hr>

<section class = "pb-50 px-25 pt-25">
    <div class = "flex flex-col items-center justify-center bg-[#191F00] py-15 gap-6 rounded-3xl">
        <h2 class = "text-white font-bold text-[2.75rem]">Готові до <span class = "bg-clip-text text-transparent bg-linear-to-r from-[#BCEB01] to-[#EEFF00]">змагань</span>?</h2>
        <p class ="text-white text-[1.25rem] font-light text-center">Зареєструйте свою команду або створіть власний турнір <br> вже зараз.</p>
        <Link href = "/registration" variant = "green" size = "lg" >Створити акаунт</Link>
    </div>
</section>

