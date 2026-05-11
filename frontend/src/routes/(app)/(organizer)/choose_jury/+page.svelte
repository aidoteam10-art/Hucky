<script>
	const currentOrganizerId = 1;

	let selectedJuryId = '';
	let selectedTournamentId;

	let tournaments = [
		{
			id: 1,
			organizerId: 1,
			title: 'Spring Hackaton 2024',
			status: 'registration',
			juryIds: [1]
		},
		{
			id: 2,
			organizerId: 1,
			title: 'AI Challenge 2026',
			status: 'registration',
			juryIds: [2]
		},
		{
			id: 3,
			organizerId: 1,
			title: 'Cool Hackaton 2026',
			status: 'registration',
			juryIds: []
		},
		{
			id: 4,
			organizerId: 1,
			title: 'Winter Coding Cup',
			status: 'active',
			juryIds: []
		},
		{
			id: 5,
			organizerId: 2,
			title: 'Other Organizer Cup',
			status: 'registration',
			juryIds: [3]
		}
	];

	const juries = [
		{
			id: 1,
			name: 'Alex Kovalenko',
			email: 'example@example.com',
			school: 'KPI University'
		},
		{
			id: 2,
			name: 'Dr. Nataliya Boyko',
			email: 'nataliya@example.com',
			school: 'Lviv Polytechnic'
		},
		{
			id: 3,
			name: 'Maria Shevchenko',
			email: 'maria@example.com',
			school: 'KNU'
		},
		{
			id: 4,
			name: 'Danylo Melnyk',
			email: 'danylo@example.com',
			school: 'Kharkiv IT Cluster'
		}
	];

	$: registrationTournaments = tournaments.filter(
		(tournament) =>
			tournament.organizerId === currentOrganizerId && tournament.status === 'registration'
	);
	$: selectedTournamentId = selectedTournamentId || registrationTournaments[0]?.id;
	$: selectedTournament = registrationTournaments.find(
		(tournament) => tournament.id === selectedTournamentId
	);
	$: assignedJuries = selectedTournament
		? juries.filter((jury) => selectedTournament.juryIds.includes(jury.id))
		: [];
	$: assignedJuryIds = tournaments.flatMap((tournament) => tournament.juryIds);
	$: availableJuries = juries.filter((jury) => !assignedJuryIds.includes(jury.id));
	$: busyJuries = juries
		.map((jury) => {
			const tournament = tournaments.find(
				(item) => item.id !== selectedTournament?.id && item.juryIds.includes(jury.id)
			);

			return tournament ? { ...jury, tournamentTitle: tournament.title } : null;
		})
		.filter(Boolean);

	const statusLabel = {
		registration: 'Реєстрація',
		active: 'Активний',
		finished: 'Завершений'
	};

	function selectTournament(tournamentId) {
		selectedTournamentId = tournamentId;
		selectedJuryId = '';
	}

	function addJury() {
		if (!selectedTournament || !selectedJuryId) return;

		tournaments = tournaments.map((tournament) =>
			tournament.id === selectedTournament.id
				? { ...tournament, juryIds: [...tournament.juryIds, Number(selectedJuryId)] }
				: tournament
		);
		selectedJuryId = '';
	}

	function removeJury(juryId) {
		if (!selectedTournament) return;

		tournaments = tournaments.map((tournament) =>
			tournament.id === selectedTournament.id
				? { ...tournament, juryIds: tournament.juryIds.filter((id) => id !== juryId) }
				: tournament
		);
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
				Призначайте журі на ваші турніри. Одне журі може бути призначене лише на один
				турнір одночасно.
			</p>
		</div>

		<div class="grid gap-7 lg:grid-cols-[18rem_minmax(0,1fr)] xl:grid-cols-[20rem_minmax(0,1fr)]">
			<aside>
				<h2 class="mb-4 text-[1rem] font-medium uppercase text-[#696969]">
					МОЇ ТУРНІРИ ({registrationTournaments.length})
				</h2>

				<div class="grid gap-3">
					{#each registrationTournaments as tournament}
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
											class="rounded-full bg-[#191F00] px-3 py-1 text-[0.6875rem] font-medium text-[#CCFF00]"
										>
											{statusLabel[tournament.status]}
										</span>
										<span class="text-[0.8125rem] text-[#696969]">
											{tournament.juryIds.length}
											{tournament.juryIds.length === 1 ? 'журі' : 'журі'}
										</span>
									</div>
								</div>
							</div>
						</button>
					{/each}
				</div>
			</aside>

			{#if selectedTournament}
				<section class="grid gap-5">
					<div class="rounded-xl border border-[#B4B4B4] bg-white p-5 sm:p-6">
						<div class="flex flex-col gap-3 sm:flex-row sm:items-start sm:justify-between">
							<div>
								<h2 class="break-words text-[1.375rem] font-semibold text-[#191F00]">
									{selectedTournament.title}
								</h2>
								<p class="mt-2 text-[1rem] text-[#696969]">
									Призначено журі: {assignedJuries.length}
								</p>
							</div>
							<span
								class="w-fit rounded-full bg-[#191F00] px-4 py-1.5 text-[0.8125rem] font-medium text-[#CCFF00]"
							>
								{statusLabel[selectedTournament.status]}
							</span>
						</div>
					</div>

					<div class="rounded-xl border border-[#B4B4B4] bg-white p-5 sm:p-6">
						<h2 class="mb-4 text-[1.125rem] font-semibold text-[#191F00]">Додати журі</h2>
						<div class="grid gap-3 sm:grid-cols-[minmax(0,1fr)_8rem]">
							<select
								bind:value={selectedJuryId}
								class="h-12 w-full rounded-lg border border-[#B4B4B4] bg-white px-4 text-[1rem] text-[#191F00] outline-none transition focus:border-[#89AB00] focus:ring-1 focus:ring-[#89AB00]"
							>
								<option value="">Оберіть журі...</option>
								{#each availableJuries as jury}
									<option value={jury.id}>{jury.name}</option>
								{/each}
							</select>
							<button
								type="button"
								on:click={addJury}
								disabled={!selectedJuryId}
								class="h-12 rounded-lg bg-[#CCFF00] px-4 text-[1rem] font-semibold text-[#191F00] transition disabled:cursor-not-allowed disabled:opacity-70"
							>
								+ Додати
							</button>
						</div>
						<p class="mt-3 text-[0.9375rem] text-[#696969]">
							Доступні лише журі, які ще не призначені на жоден турнір.
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
											{jury.name}
										</h3>
										<div
											class="mt-2 flex flex-col gap-2 text-[0.875rem] text-[#696969] sm:flex-row sm:flex-wrap sm:items-center sm:gap-x-5"
										>
											<span class="flex min-w-0 items-center gap-2">
												<img src="/icons/mail.svg" alt="" class="h-4 w-4 shrink-0" />
												<span class="break-all">{jury.email}</span>
											</span>
											<span class="flex min-w-0 items-center gap-2">
												<img src="/icons/school.svg" alt="" class="h-4 w-4 shrink-0" />
												<span class="break-words">{jury.school}</span>
											</span>
										</div>
									</div>
									<button
										type="button"
										on:click={() => removeJury(jury.id)}
										class="self-start rounded-lg px-4 py-2 text-[0.9375rem] font-semibold text-[#191F00] transition hover:bg-[#CCFF00] sm:self-center"
									>
										Зняти
									</button>
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
							Зайняті журі (на інших турнірах)
						</h2>
						<div class="grid gap-2">
							{#each busyJuries as jury}
								<article
									class="flex flex-col gap-2 rounded-lg bg-[#F4F4F4] px-4 py-3 text-[0.9375rem] sm:flex-row sm:items-center sm:justify-between"
								>
									<span class="font-semibold text-[#191F00]">{jury.name}</span>
									<span class="flex items-center gap-2 text-[#696969]">
										<span aria-hidden="true">→</span>
										<span>{jury.tournamentTitle}</span>
									</span>
								</article>
							{:else}
								<p class="rounded-lg bg-[#F4F4F4] px-4 py-3 text-[0.9375rem] text-[#696969]">
									Немає журі, зайнятих на інших турнірах.
								</p>
							{/each}
						</div>
					</div>
				</section>
			{/if}
		</div>
	</section>
</main>
