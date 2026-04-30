<script>
    import { onMount } from "svelte";
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
    {title : "Hackathon Ukraine 2026", desc : "Національний хакатон для студентських команд. Створюйте інноваційні рішення за 48 годин.", max_teams : 50, registered_teams: 24, date : new Date(2026, 3, 15, 12, 0, 0, 0)},
    {title : "Hackathon Ukraine 2026", desc : "Національний хакатон для студентських команд. Створюйте інноваційні рішення за 48 годин.", max_teams : 50, registered_teams: 24, date : new Date(2026, 3, 15, 12, 0, 0, 0)},
    {title : "Hackathon Ukraine 2026", desc : "Національний хакатон для студентських команд. Створюйте інноваційні рішення за 48 годин.", max_teams : 50, registered_teams: 24, date : new Date(2026, 3, 15, 12, 0, 0, 0)},
  ]

  let scrollContainer;

  onMount(() => {
    if (scrollContainer) {
      // Спочатку відцентровуємо скрол так, щоб було видно центральні елементи (а не починати з найлівішого)
      const centerPosition = (scrollContainer.scrollWidth - scrollContainer.clientWidth) / 2;
      scrollContainer.scrollLeft = centerPosition;

      // --- 1. Інерційний скролінг коліщатком (Lerp алгоритм) ---
      let targetScroll = centerPosition;
      let isAnimating = false;

      const lerp = (start, end, factor) => start + (end - start) * factor;

      const animateScroll = () => {
        if (!scrollContainer) return;
        const currentScroll = scrollContainer.scrollLeft;
        // Поки різниця між ціллю і поточним станом більше 0.5px - анімуємо
        if (Math.abs(targetScroll - currentScroll) > 0.5) {
          scrollContainer.scrollLeft = lerp(currentScroll, targetScroll, 0.08); // 0.08 = коефіцієнт плавності гальмування
          requestAnimationFrame(animateScroll);
        } else {
          scrollContainer.scrollLeft = targetScroll;
          isAnimating = false;
        }
      };

      const handleWheel = (e) => {
        e.preventDefault();
        // Якщо анімація вже спинилась, синхронізуємо ціль з фактичним станом
        if (!isAnimating) {
          targetScroll = scrollContainer.scrollLeft;
        }
        
        targetScroll += e.deltaY * 1.5;
        // Не даємо залетіти за краї
        targetScroll = Math.max(0, Math.min(targetScroll, scrollContainer.scrollWidth - scrollContainer.clientWidth));
        
        if (!isAnimating) {
          isAnimating = true;
          requestAnimationFrame(animateScroll);
        }
      };

      // --- 2. Скролінг перетягуванням (Drag-to-Scroll) ---
      let isDown = false;
      let startX;
      let scrollLeftPos;
      let isDragging = false;

      const handleMouseDown = (e) => {
        isDown = true;
        isDragging = false;
        scrollContainer.classList.add('cursor-grabbing');
        scrollContainer.classList.remove('cursor-pointer');
        startX = e.pageX - scrollContainer.offsetLeft;
        scrollLeftPos = scrollContainer.scrollLeft;
        isAnimating = false; // Зупиняємо інерцію від коліщатка, якщо схопили мишкою
      };

      const handleMouseLeave = () => {
        isDown = false;
        scrollContainer.classList.remove('cursor-grabbing');
        scrollContainer.classList.add('cursor-pointer');
      };

      const handleMouseUp = () => {
        isDown = false;
        scrollContainer.classList.remove('cursor-grabbing');
        scrollContainer.classList.add('cursor-pointer');
      };

      const handleMouseMove = (e) => {
        if (!isDown) return;
        e.preventDefault();
        const x = e.pageX - scrollContainer.offsetLeft;
        const walk = (x - startX) * 1.5; // Множник швидкості для тяги за курсором
        
        // Якщо рух склав більше ніж 5px, маркуємо це як "перетягування", щоб уникнути випадкових кліків по картках
        if (Math.abs(walk) > 5) isDragging = true; 
        
        scrollContainer.scrollLeft = scrollLeftPos - walk;
        targetScroll = scrollContainer.scrollLeft; // Синхронізуємо щоб коліщатко знало де ми спинились
      };

      // Перехоплювач кліків (забороняє переходи на іншу сторінку, якщо ми щойно тягнули карусель)
      const handleClick = (e) => {
        if (isDragging) {
          e.preventDefault();
          e.stopPropagation();
        }
      };

      // Додаємо всі слухачі
      scrollContainer.addEventListener('wheel', handleWheel, { passive: false });
      scrollContainer.addEventListener('mousedown', handleMouseDown);
      scrollContainer.addEventListener('mouseleave', handleMouseLeave);
      scrollContainer.addEventListener('mouseup', handleMouseUp);
      scrollContainer.addEventListener('mousemove', handleMouseMove);
      scrollContainer.addEventListener('click', handleClick, { capture: true });
      
      return () => {
        if (!scrollContainer) return;
        scrollContainer.removeEventListener('wheel', handleWheel);
        scrollContainer.removeEventListener('mousedown', handleMouseDown);
        scrollContainer.removeEventListener('mouseleave', handleMouseLeave);
        scrollContainer.removeEventListener('mouseup', handleMouseUp);
        scrollContainer.removeEventListener('mousemove', handleMouseMove);
        scrollContainer.removeEventListener('click', handleClick, { capture: true });
      };
    }
  });
</script>

<section class="relative flex min-h-screen lg:min-h-[90%] bg-[#191F00] overflow-hidden py-16 lg:py-0">

    <div class="absolute top-0 right-0 w-1/4 h-full bg-[#CCFF00] z-10 hidden lg:block"></div>
    <div class="relative z-20 flex flex-col items-center lg:items-start justify-center px-6 sm:px-10 lg:pl-26 text-white w-full lg:w-[60%] text-center lg:text-left">
        <div class="md:hidden w-64 mb-16 bg-[#191F00]">
            <video 
                class="w-full h-full object-contain mix-blend-screen" 
                autoplay muted playsinline
            >
                <source src="logo_anim.mp4" type="video/mp4">
            </video>
        </div>
        <h1 class="text-4xl sm:text-5xl lg:text-[5rem] leading-tight font-black mb-8 lg:mb-15 mt-2">
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


<div class="flex flex-col items-center py-16 xl:py-20 px-5 gap-8 xl:gap-12 mb-10 text-center xl:text-left">
    <div class="flex flex-col items-center gap-2 text-center">
        <h1 class="font-bold text-3xl sm:text-4xl xl:text-[2.75rem] text-[#191F00]">Все для проведення турнірів</h1>
        <p class="text-lg xl:text-[1.5rem] text-[#191F00]">Повний набір інструментів для організаторів, команд та журі</p>
    </div>

    <div class="grid grid-cols-1 sm:grid-cols-2 xl:grid-cols-4 gap-4 md:gap-6 xl:gap-12 w-full px-2 sm:px-8 md:px-12">
        {#each features as feature}
        <div class="p-5 md:p-6 xl:p-10 flex flex-col ring-2 rounded-xl items-start gap-4 h-full text-left">
           <img src={feature.icon} alt={feature.title} class="bg-[#191F00] p-1.5 md:p-2.5 rounded-[0.625rem] w-8 h-8 md:w-12 md:h-12">
           <h3 class="font-semibold text-base md:text-xl xl:text-[1.25rem]">{feature.title}</h3>
           <p class="text-sm md:text-base xl:text-[1.2rem] leading-tight">{feature.desc}</p>
        </div>
        {/each}
    </div>

</div>



<div class="flex flex-col items-center gap-6 lg:gap-10 mb-16 lg:mb-24 w-full overflow-hidden">
    
    <h1 class="font-bold text-3xl sm:text-4xl lg:text-[2.75rem] text-center px-4">Майбутні турніри</h1>
    
    <!-- Горизонтальний скрол-контейнер з вільним прокручуванням (без магнетизму) -->
    <div bind:this={scrollContainer} class="flex overflow-x-auto w-full gap-4 sm:gap-6 px-4 sm:px-10 lg:px-26 pb-8 pt-2 [scrollbar-width:none] [&::-webkit-scrollbar]:hidden select-none cursor-pointer">
        {#each tournaments as tournament}
        <!-- Кожна картка має задану ширину, не стискається і вільно скролиться -->
        <div class="w-[85vw] sm:w-[45vw] xl:w-[28vw] shrink-0">
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