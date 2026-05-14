<script>
	export let data;
	export let form;

	let selectedJuryId = '';
	let selectedTournamentId = data.selectedTournamentId || '';

	$: manageableTournaments = data.tournaments || [];
	$: juries = data.juries || [];
	$: if (!selectedTournamentId && manageableTournaments.length > 0) {
		selectedTournamentId = manageableTournaments[0].id;
	}
	$: selectedTournament = manageableTournaments.find(
		(tournament) => tournament.id === selectedTournamentId
	);
	$: assignedJuries = selectedTournament
		? juries.filter((jury) => selectedTournament.jury_ids.includes(jury.id))
		: [];
	$: availableJuries = selectedTournament
		? juries.filter((jury) => !selectedTournament.jury_ids.includes(jury.id))
		: [];
	$: assignedElsewhere = juries
		.map((jury) => {
			const assignedTournaments = manageableTournaments.filter(
				(item) => item.id !== selectedTournament?.id && item.jury_ids.includes(jury.id)
			);

			return assignedTournaments.length
				? { ...jury, tournamentTitles: assignedTournaments.map((item) => item.title) }
				: null;
		})
		.filter(Boolean);

	const statusLabel = {
		draft: 'Чернетка',
		registration: 'Реєстрація',
		running: 'Активний',
		finished: 'Завершений'
	};

	function selectTournament(tournamentId) {
		selectedTournamentId = tournamentId;
		selectedJuryId = '';
	}
</script>

<main
	class="min-h-[calc(100vh-5rem)] bg-white px-4 py-10 font-sans text-[#191F00] sm:px-8 lg:py-20 xl:pl-[7.625rem] xl:pr-[8.3125rem]"
>
	<section class="mx-auto w-full">
		<div class="mb-9">
			<div class="mb-5 flex items-center gap-4">
				<img src="/icons/juries_for_tournament.svg" alt="" class="h-12 w-12 shrink-0" />
				<h1 class="text-[2.125rem] font-semibold leading-tight text-[#191F00] sm:text-[2.75rem]">
					Журі моїх турнірів
				</h1>
			</div>
			<p class="max-w-5xl text-[1.0625rem] leading-8 text-[#696969] sm:text-[1.25rem]">
				Призначайте користувачів з роллю журі на турніри, які ще не завершені.
				Якщо журі вже є в іншому вашому турнірі, це буде показано окремо.
			</p>
		</div>

		{#if form?.message}
			<div class="mb-7 rounded-xl border border-red-200 bg-red-50 px-5 py-4 text-sm font-semibold text-red-700">
				{form.message}
			</div>
		{/if}

		<div class="grid gap-7 lg:grid-cols-[18rem_minmax(0,1fr)] xl:grid-cols-[20rem_minmax(0,1fr)]">
			<aside>
				<h2 class="mb-4 text-[1rem] font-medium uppercase text-[#696969]">
					МОЇ ТУРНІРИ ({manageableTournaments.length})
				</h2>

				<div class="grid gap-3">
					{#each manageableTournaments as tournament}
						<button
							type="button"
							on:click={() => selectTournament(tournament.id)}
							class="rounded-lg border p-4 text-left transition {selectedTournamentId === tournament.id
								? 'border-[#89AB00] bg-[#CCFF00]'
								: 'border-[#B4B4B4] bg-white hover:border-[#89AB00]'}"
						>
							<div class="flex items-start gap-3">
								<img
									src={selectedTournamentId === tournament.id
										? '/icons/cup_green.svg'
										: '/icons/cup_gray.svg'}
									alt=""
									class="mt-0.5 h-6 w-6 shrink-0"
								/>
								<div class="min-w-0">
									<h3 class="break-words text-[1rem] font-medium text-[#191F00]">
										{tournament.title}
									</h3>
									<div class="mt-2 flex flex-wrap items-center gap-3">
										<span
											class="shrink-0 whitespace-nowrap rounded-full bg-[#191F00] px-3 py-1 text-[0.6875rem] font-medium text-[#CCFF00]"
										>
											{statusLabel[tournament.status]}
										</span>
										<span class="text-[0.8125rem] text-[#696969]">
											{tournament.jury_ids.length}
											{tournament.jury_ids.length === 1 ? 'журі' : 'журі'}
										</span>
									</div>
								</div>
							</div>
						</button>
					{:else}
						<p class="rounded-lg border border-[#B4B4B4] p-4 text-[0.9375rem] text-[#696969]">
							У вас немає незавершених турнірів.
						</p>
					{/each}
				</div>
			</aside>

			{#if selectedTournament}
				<section class="grid gap-5">
					<div class="rounded-xl border border-[#B4B4B4] bg-white p-5 sm:p-6">
						<div class="flex flex-col gap-3 sm:flex-row sm:items-start sm:justify-between">
							<div class="min-w-0">
								<h2 class="break-words text-[1.375rem] font-semibold text-[#191F00]">
									{selectedTournament.title}
								</h2>
								<p class="mt-2 text-[1rem] text-[#696969]">
									Призначено журі: {assignedJuries.length}
								</p>
							</div>
							<span
								class="w-fit shrink-0 whitespace-nowrap rounded-full bg-[#191F00] px-4 py-1.5 text-[0.8125rem] font-medium text-[#CCFF00]"
							>
								{statusLabel[selectedTournament.status]}
							</span>
						</div>
					</div>

					<div class="rounded-xl border border-[#B4B4B4] bg-white p-5 sm:p-6">
						<h2 class="mb-4 text-[1.125rem] font-semibold text-[#191F00]">Додати журі до турніру</h2>
						<form method="POST" action="?/addJury" class="grid gap-3 sm:grid-cols-[minmax(0,1fr)_8rem]">
							<input type="hidden" name="tournament_id" value={selectedTournament.id} />
							<select
								name="user_id"
								bind:value={selectedJuryId}
								disabled={availableJuries.length === 0}
								class="h-12 w-full rounded-lg border border-[#B4B4B4] bg-white px-4 text-[1rem] text-[#191F00] outline-none transition focus:border-[#89AB00] focus:ring-1 focus:ring-[#89AB00]"
							>
								<option value="">Оберіть журі...</option>
								{#each availableJuries as jury}
									<option value={jury.id}>{jury.full_name}</option>
								{/each}
							</select>
							<button
								type="submit"
								disabled={!selectedJuryId}
								class="h-12 rounded-lg bg-[#CCFF00] px-4 text-[1rem] font-semibold text-[#191F00] transition disabled:cursor-not-allowed disabled:opacity-70"
							>
								+ Додати
							</button>
						</form>
						<p class="mt-3 text-[0.9375rem] text-[#696969]">
							{juries.length === 0
								? 'У системі ще немає користувачів з роллю журі.'
								: 'У списку лише користувачі з роллю журі, які ще не додані до вибраного турніру.'}
						</p>
					</div>

					<div class="rounded-xl border border-[#B4B4B4] bg-white p-5 sm:p-6">
						<h2 class="mb-4 text-[1.125rem] font-semibold text-[#191F00]">
							Призначені журі ({assignedJuries.length})
						</h2>
						<div class="grid gap-3">
							{#each assignedJuries as jury}
								<article
									class="flex flex-col gap-3 rounded-lg border border-[#B4B4B4] bg-white p-4 sm:flex-row sm:items-center sm:justify-between"
								>
									<div class="min-w-0">
										<h3 class="break-words text-[1rem] font-semibold text-[#191F00]">
											{jury.full_name}
										</h3>
										<div
											class="mt-2 flex flex-col gap-2 text-[0.875rem] text-[#696969] sm:flex-row sm:flex-wrap sm:items-center sm:gap-x-5"
										>
											<span class="flex min-w-0 items-center gap-2">
												<img src="/icons/mail.svg" alt="" class="h-4 w-4 shrink-0" />
												<span class="break-all">{jury.email}</span>
											</span>
										</div>
									</div>
									<form method="POST" action="?/removeJury" class="self-start sm:self-center">
										<input type="hidden" name="tournament_id" value={selectedTournament.id} />
										<input type="hidden" name="user_id" value={jury.id} />
										<button
											type="submit"
											class="rounded-lg px-4 py-2 text-[0.9375rem] font-semibold text-[#191F00] transition hover:bg-[#CCFF00]"
										>
											Зняти
										</button>
									</form>
								</article>
							{:else}
								<p class="rounded-lg border border-[#B4B4B4] p-4 text-[0.9375rem] text-[#696969]">
									На цей турнір ще не призначено журі.
								</p>
							{/each}
						</div>
					</div>

					<div class="rounded-xl border border-[#B4B4B4] bg-white p-5 sm:p-6">
						<h2 class="mb-4 text-[1.125rem] font-semibold text-[#191F00]">
							Також призначені на інші мої турніри
						</h2>
						<div class="grid gap-2">
							{#each assignedElsewhere as jury}
								<article
									class="flex flex-col gap-2 rounded-lg bg-[#F4F4F4] px-4 py-3 text-[0.9375rem] sm:flex-row sm:items-center sm:justify-between"
								>
									<span class="font-semibold text-[#191F00]">{jury.full_name}</span>
									<span class="flex items-center gap-2 text-[#696969]">
										<span aria-hidden="true">→</span>
										<span>{jury.tournamentTitles.join(', ')}</span>
									</span>
								</article>
							{:else}
								<p class="rounded-lg bg-[#F4F4F4] px-4 py-3 text-[0.9375rem] text-[#696969]">
									Немає журі, призначених на інші незавершені турніри.
								</p>
							{/each}
						</div>
					</div>
				</section>
			{:else}
				<section class="rounded-xl border border-[#B4B4B4] bg-white p-6 text-[#696969]">
					<h2 class="mb-2 text-[1.25rem] font-semibold text-[#191F00]">Немає доступних турнірів</h2>
					<p>Журі можна призначати тільки на турніри, які ще не завершені.</p>
				</section>
			{/if}
		</div>
	</section>
</main>
