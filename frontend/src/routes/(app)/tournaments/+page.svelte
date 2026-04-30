<script>
  import TournamentCard from '../../../components/TournamentCard.svelte';
  import Pagination from '../../../components/Pagination.svelte';
  import { tournaments } from '$lib/data/tournaments.js';

  let currentFilter = 'all';
  let searchQuery = '';
  let currentPage = 1;
  let itemsPerPage = 6;

  const filterOptions = [
    { id: 'all', label: 'Всі' },
    { id: 'registration', label: 'Реєстрація відкрита' },
    { id: 'active', label: 'Активні' },
    { id: 'completed', label: 'Завершені' }
  ];

  $: filteredTournaments = tournaments.filter((t) => {
    const matchesFilter = currentFilter === 'all' || t.current_state === currentFilter;
    const matchesSearch =
            t.title.toLowerCase().includes(searchQuery.toLowerCase()) ||
            t.description.toLowerCase().includes(searchQuery.toLowerCase());

    return matchesFilter && matchesSearch;
  });

  $: totalPages = Math.ceil(filteredTournaments.length / itemsPerPage) || 1;
  
  // Скидаємо на 1-шу сторінку при будь-якій зміні пошуку або фільтру
  $: searchQuery, currentFilter, (currentPage = 1);

  $: paginatedTournaments = filteredTournaments.slice((currentPage -1)* itemsPerPage, currentPage * itemsPerPage);

</script>

<svelte:head>
  <title>Hucky - Турніри</title>
</svelte:head>

<main class="px-4 pt-10 pb-2 md:px-10 md:pt-18 md:pb-3.6 xl:px-17">
  <div class="mx-auto max-w-[1440px]">
    
    <div class="mb-9 flex flex-col gap-6 xl:flex-row xl:items-center xl:justify-between">
      <div class="flex flex-wrap items-center gap-5 md:gap-6">
        <div class="flex h-8 w-8 items-center justify-center">
              <img src = "icons/filter_icon.svg" alt = "Filter Icon">
        </div>

        <div class="flex flex-wrap gap-2 md:gap-4">
          {#each filterOptions as option}
            <button
                    class="rounded-lg px-4 py-3 text-sm font-bold leading-none transition md:px-9.5 md:py-5 md:text-lg {
        currentFilter === option.id
          ? 'bg-[#CCFF00] text-[#111111]'
          : 'bg-[#F4F4F5] text-[#202020] hover:bg-[#D9D9D9]'
      }"
                    on:click={() => (currentFilter = option.id)}
            >
              {option.label}
            </button>
          {/each}
        </div>
      </div>

      <div class="relative w-full xl:w-115">
        <input
                type="text"
                bind:value={searchQuery}
                placeholder="Пошук . . ."
                class="w-full rounded-full border-none bg-[#F4F4F5] px-6 py-4 font-semibold text-base text-[#8e8e8e] outline-none md:px-12.5 md:py-5 md:text-lg"
        />

        <img 
                src="/icons/search_icon.svg" 
                alt="Search Icon" 
                class="absolute right-5 top-1/2 h-6 w-6 -translate-y-1/2" 
        />
      </div>
    </div>

    <section class="tournaments-grid mx-auto grid grid-cols-1 gap-x-7 gap-y-7 md:grid-cols-2 xl:grid-cols-3">
      {#each paginatedTournaments as t (t.id)}
            <TournamentCard
                    variant="grey"
                    current_state={t.current_state}
                    title={t.title}
                    description={t.description}
                    start_date={t.start_date}
                    rounds={t.rounds}
                    max_teams={t.max_teams}
                    registered_teams={t.registered_teams}
                    id={t.id}
            />
      {:else}
        <p class="col-span-full py-10 text-center text-lg text-gray-500">
          Не знайдено жодного турніру.
        </p>
      {/each}
    </section>

    <!-- Приклеюємо пагінацію до низу вікна -->
    <div class="sticky bottom-5 z-10 w-fit mx-auto px-6 py-3 rounded-2xl bg-white/80 backdrop-blur-md shadow-[0_8px_30px_rgb(0,0,0,0.12)] border border-[#e5e5e5]">
      <Pagination bind:currentPage={currentPage} totalPages={totalPages} />
    </div>
  </div>
</main>