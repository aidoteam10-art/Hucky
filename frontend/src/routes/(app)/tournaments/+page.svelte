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

<main class="bg-[#f3f3f3] pb-[90px] pt-[70px] md:pt-[95px]">
  <div class="mx-auto max-w-[1680px] px-6 md:px-7 xl:px-8">
    <div class="mb-[36px] flex flex-col gap-6 xl:flex-row xl:items-center xl:justify-between">
      <div class="flex flex-wrap items-center gap-5 md:gap-6">
        <div class="flex h-[32px] w-[32px] items-center justify-center">
          <svg
            class="h-[24px] w-[24px] fill-none stroke-[1.8] stroke-[#202020]"
            viewBox="0 0 24 24"
          >
            <path d="M4 5H20L14 12V18L10 20V12L4 5Z" />
          </svg>
        </div>

        {#each filterOptions as option}
          <button
            class={`rounded-[8px] px-8 py-[16px] text-[19px] font-[700] leading-none transition ${
              currentFilter === option.id
                ? 'bg-[#CCFF00] text-[#111111]'
                : 'bg-[#e9e9e9] text-[#202020]'
            }`}
            on:click={() => (currentFilter = option.id)}
          >
            {option.label}
          </button>
        {/each}
      </div>

      <div class="relative w-full xl:w-[410px]">
        <input
          type="text"
          bind:value={searchQuery}
          placeholder="Пошук . . ."
          class="h-[56px] w-full rounded-full border-none bg-[#e9e9e9] pl-7 pr-14 text-[18px] text-[#8e8e8e] outline-none"
        />

        <svg
          class="absolute right-5 top-1/2 h-[24px] w-[24px] -translate-y-1/2 fill-none stroke-[1.8] stroke-[#9a9a9a]"
          viewBox="0 0 24 24"
        >
          <circle cx="11" cy="11" r="7"></circle>
          <path d="M20 20L16.5 16.5"></path>
        </svg>
      </div>
    </div>

    <section class="tournaments-grid mx-auto grid max-w-[1600px] grid-cols-1 gap-x-[28px] gap-y-[28px] md:grid-cols-2 xl:grid-cols-3">
      {#each filteredTournaments as t (t.id)}
        <div class="scale-y-[0.97]">
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
      {:else}
        <p class="col-span-full py-10 text-center text-lg text-gray-500">
          Не знайдено жодного турніру.
        </p>
      {/each}
    </section>

    <Pagination bind:currentPage={currentPage} totalPages={3} />
  </div>
</main>