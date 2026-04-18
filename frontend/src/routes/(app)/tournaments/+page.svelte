<script>
  import TournamentCard from '../../../components/TournamentCard.svelte';
  import Pagination from '../../../components/Pagination.svelte';
  import CardShell from '../../../components/CardShell.svelte';
  import { tournaments } from '$lib/data/tournaments.js';

  let currentFilter = 'all';
  let searchQuery = '';
  let currentPage = 1;

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
</script>

<svelte:head>
  <title>Hucky - Турніри</title>
</svelte:head>

<main class="px-4 py-10 md:px-10 md:pt-19 md:pb-14.5 xl:px-17">
  <div class="mx-auto max-w-[1440px]">
    
    <div class="mb-9 flex flex-col gap-6 xl:flex-row xl:items-center xl:justify-between">
      
      <div class="flex flex-wrap items-center gap-3 md:gap-6">
        <div class="hidden sm:flex h-8 w-8 items-center justify-center">
          <svg
                  class="h-6 w-6 fill-none stroke-[1.8] stroke-[#202020]"
                  viewBox="0 0 24 24"
          >
            <path d="M4 5H20L14 12V18L10 20V12L4 5Z" />
          </svg>
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

        <svg
                class="absolute right-5 top-1/2 h-5 w-5 -translate-y-1/2 fill-none stroke-[1.8] stroke-[#9a9a9a] md:h-6 md:w-6"
                viewBox="0 0 24 24"
        >
          <circle cx="11" cy="11" r="7"></circle>
          <path d="M20 20L16.5 16.5"></path>
        </svg>
      </div>
    </div>

    <section class="tournaments-grid mx-auto grid grid-cols-1 gap-x-7 gap-y-7 sm:grid-cols-2 lg:grid-cols-3">
      {#each filteredTournaments as t (t.id)}
        <a href="/tournaments/{t.id}" class="block transition-transform duration-200 hover:scale-[1.02] active:scale-[0.98]">
          <div class="h-full scale-y-[0.97]">
            <CardShell>
              <TournamentCard
                      current_state={t.current_state}
                      title={t.title}
                      description={t.description}
                      start_date={t.start_date}
                      rounds={t.rounds}
                      max_teams={t.max_teams}
                      registered_teams={t.registered_teams}
                      id={t.id}
              />
            </CardShell>
          </div>
        </a>
      {:else}
        <p class="col-span-full py-10 text-center text-lg text-gray-500">
          Не знайдено жодного турніру.
        </p>
      {/each}
    </section>

    <div class="mt-10">
      <Pagination bind:currentPage={currentPage} totalPages={3} />
    </div>
  </div>
</main>