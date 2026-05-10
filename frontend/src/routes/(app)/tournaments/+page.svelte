<script>
	import { resolve } from '$app/paths';
	import TournamentCard from '../../../components/TournamentCard.svelte';

	export let data;

	const filterOptions = [
		{ id: 'all', label: 'Всі' },
		{ id: 'draft', label: 'Чернетки' },
		{ id: 'registration', label: 'Реєстрація' },
		{ id: 'running', label: 'Тривають' },
		{ id: 'finished', label: 'Завершені' }
	];

	$: tournaments = data.tournaments;
	$: currentStatus = data.filters.status;
	$: searchQuery = data.filters.search;
	$: pageNumbers = Array.from({ length: tournaments.total_pages }, (_, index) => index + 1);

	function queryFor(params) {
		const query = [];
		const status = params.status ?? currentStatus;
		const search = params.search ?? searchQuery;
		const page = params.page ?? tournaments.page;

		if (status && status !== 'all') query.push(`status=${encodeURIComponent(status)}`);
		if (search) query.push(`search=${encodeURIComponent(search)}`);
		if (page && page > 1) query.push(`page=${encodeURIComponent(page)}`);

		const suffix = query.join('&');
		return suffix ? `?${suffix}` : '';
	}
</script>

<svelte:head>
	<title>Hucky - Турніри</title>
</svelte:head>

<main class="px-4 pt-10 pb-8 md:px-10 md:pt-18 xl:px-17">
	<div class="mx-auto max-w-[1440px]">
		<div class="mb-9 flex flex-col gap-6 xl:flex-row xl:items-center xl:justify-between">
			<div class="flex flex-wrap items-center gap-3 md:gap-6">
				<div class="hidden h-8 w-8 items-center justify-center sm:flex">
					<img src="/icons/filter_icon.svg" alt="" />
				</div>

				<div class="flex flex-wrap gap-2 md:gap-4">
					{#each filterOptions as option (option.id)}
						<a
							href={resolve('/tournaments' + queryFor({ status: option.id, page: 1 }))}
							class="rounded-lg px-4 py-3 text-sm font-bold leading-none transition md:px-9 md:py-5 md:text-lg {currentStatus ===
							option.id
								? 'bg-[#CCFF00] text-[#111111]'
								: 'bg-[#F4F4F5] text-[#202020] hover:bg-[#D9D9D9]'}"
						>
							{option.label}
						</a>
					{/each}
				</div>
			</div>

			<div class="flex w-full flex-col gap-3 xl:w-[32rem]">
				<form method="GET" action="/tournaments" class="relative w-full">
					{#if currentStatus !== 'all'}
						<input type="hidden" name="status" value={currentStatus} />
					{/if}
					<input
						type="text"
						name="search"
						value={searchQuery}
						placeholder="Пошук..."
						class="w-full rounded-full border-none bg-[#F4F4F5] px-6 py-4 text-base font-semibold text-[#202020] outline-none md:px-12 md:py-5 md:text-lg"
					/>
					<button
						type="submit"
						class="absolute top-1/2 right-4 flex h-10 w-10 -translate-y-1/2 items-center justify-center rounded-full hover:bg-[#e5e5e5]"
						aria-label="Пошук"
					>
						<img src="/icons/search_icon.svg" alt="" class="h-6 w-6" />
					</button>
				</form>

					{#if data.user}
					<a
						href={resolve('/tournaments/new')}
						class="w-full rounded-xl bg-[#191F00] px-5 py-3 text-center font-bold text-white transition hover:bg-[#2b3500]"
					>
						Створити турнір
					</a>
				{/if}
			</div>
		</div>

		{#if data.error}
			<div class="mb-6 rounded-xl border border-red-200 bg-red-50 px-5 py-4 text-sm font-semibold text-red-700">
				{data.error}
			</div>
		{/if}

		<section class="tournaments-grid mx-auto grid grid-cols-1 gap-x-7 gap-y-7 md:grid-cols-2 xl:grid-cols-3">
			{#each tournaments.items as tournament (tournament.id)}
				<TournamentCard
					variant="grey"
					current_state={tournament.status}
					title={tournament.title}
					description={tournament.description}
					start_date={tournament.starts_at}
					rounds={tournament.rounds_count}
					max_teams={tournament.max_teams}
					registered_teams={tournament.registered_teams}
					id={tournament.id}
				/>
			{:else}
				<div class="col-span-full rounded-2xl border border-[#e5e5e5] bg-white px-8 py-12 text-center">
					<p class="text-lg font-semibold text-[#191F00]">Турнірів не знайдено.</p>
					<p class="mt-2 text-sm text-gray-500">Змініть фільтр або створіть перший турнір.</p>
				</div>
			{/each}
		</section>

		{#if tournaments.total_pages > 1}
			<div
				class="sticky bottom-5 z-10 mx-auto mt-10 flex w-fit items-center justify-center gap-2 rounded-2xl border border-[#e5e5e5] bg-white/80 px-6 py-3 shadow-[0_8px_30px_rgb(0,0,0,0.12)] backdrop-blur-md"
			>
				<a
					href={resolve('/tournaments' + queryFor({ page: Math.max(tournaments.page - 1, 1) }))}
					class="flex h-10 w-10 items-center justify-center rounded-full transition hover:bg-[#e7e7e7] {tournaments.page ===
					1
						? 'pointer-events-none opacity-50'
						: ''}"
					aria-label="Попередня сторінка"
				>
					‹
				</a>
				{#each pageNumbers as pageNumber (pageNumber)}
					<a
						href={resolve('/tournaments' + queryFor({ page: pageNumber }))}
						class="flex h-10 w-10 items-center justify-center rounded-full text-lg font-bold text-[#202020] transition {tournaments.page ===
						pageNumber
							? 'bg-[#CCFF00]'
							: 'bg-[#F4F4F5] hover:bg-[#dddddd]'}"
					>
						{pageNumber}
					</a>
				{/each}
				<a
					href={resolve(
						'/tournaments' + queryFor({ page: Math.min(tournaments.page + 1, tournaments.total_pages) })
					)}
					class="flex h-10 w-10 items-center justify-center rounded-full transition hover:bg-[#e7e7e7] {tournaments.page ===
					tournaments.total_pages
						? 'pointer-events-none opacity-50'
						: ''}"
					aria-label="Наступна сторінка"
				>
					›
				</a>
			</div>
		{/if}
	</div>
</main>
