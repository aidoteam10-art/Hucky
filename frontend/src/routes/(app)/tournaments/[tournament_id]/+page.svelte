<script>
	import { resolve } from '$app/paths';
	import StateTag from '../../../../components/StateTag.svelte';
	import DateField from '/src/components/form/DateField.svelte';
	import InputField from '/src/components/form/InputField.svelte';
	import TextArea from '/src/components/form/TextArea.svelte';

	export let data;
	export let form;

	let newRoundRequirements = [''];

	$: tournament = data.tournament;
	$: rounds = data.rounds || [];
	$: registeredTeams = tournament.registered_teams || [];
	$: userTeam = data.userTeam;
	$: canSetupRounds = data.canManage && ['draft', 'registration'].includes(tournament.status);
	$: canManageJury = data.canManage && tournament.status === 'registration';
	$: canOperateRounds = data.canManage && tournament.status === 'running';

	function formatDate(value) {
		if (!value) return 'Не вказано';
		return new Intl.DateTimeFormat('uk-UA', {
			day: '2-digit',
			month: 'long',
			year: 'numeric',
			hour: '2-digit',
			minute: '2-digit'
		}).format(new Date(value));
	}

	function tournamentActions(status) {
		if (status === 'draft') return [{ status: 'registration', label: 'Відкрити реєстрацію' }];
		if (status === 'registration')
			return [
				{ status: 'running', label: 'Запустити турнір' },
				{ status: 'draft', label: 'Повернути в чернетку', secondary: true }
			];
		if (status === 'running') return [{ status: 'finished', label: 'Завершити турнір' }];
		return [];
	}

	function roundActions(round, tournamentStatus) {
		if (tournamentStatus === 'running' && round.status === 'draft') {
			return [{ status: 'active', label: 'Опублікувати задачу' }];
		}
		return [];
	}

	function addRoundRequirement() {
		newRoundRequirements = [...newRoundRequirements, ''];
	}

	function removeRoundRequirement(index) {
		newRoundRequirements = newRoundRequirements.filter((_, itemIndex) => itemIndex !== index);
		if (newRoundRequirements.length === 0) newRoundRequirements = [''];
	}
</script>

<svelte:head>
	<title>Hucky - {tournament.title}</title>
</svelte:head>

<main class="w-full px-6 py-10 font-sans text-[#191F00] lg:px-12 lg:py-16 xl:px-16">
	<section class="mb-12 grid grid-cols-1 gap-8 lg:grid-cols-[minmax(0,1fr)_auto] lg:items-end">
		<div class="min-w-0">
			<div class="mb-6 flex flex-wrap items-center gap-4 lg:gap-8">
				<h1 class="text-anywhere text-2xl leading-tight font-bold md:text-3xl xl:text-[2.5rem]">
					{tournament.title}
				</h1>
				<StateTag variant={tournament.status} />
			</div>
			<p class="text-anywhere w-full text-[1rem] leading-relaxed xl:text-[1.25rem]">
				{tournament.description}
			</p>
		</div>

		<div class="flex w-full flex-col gap-3 lg:w-auto lg:items-end">
			{#if userTeam}
				<a
					href={`/teams/${userTeam.team_id}`}
					class="w-full rounded-2xl bg-[#191F00] px-10 py-3 text-center text-[1.1rem] font-bold text-[#CCFF00] shadow-sm transition-all hover:bg-[#2b3500] lg:w-auto"
				>
					Моя команда
				</a>
			{:else if tournament.status === 'registration' && data.canRegisterTeam}
				<a
					href={resolve('/tournaments/[tournament_id]/team-registration', {
						tournament_id: String(tournament.id)
					})}
					class="w-full rounded-2xl bg-[#CCFF00] px-10 py-3 text-center text-[1.1rem] font-bold text-[#191F00] shadow-sm transition-all hover:bg-[#A9D207] lg:w-auto"
				>
					Зареєструвати команду
				</a>
			{:else if tournament.status === 'registration' && !data.isAuthenticated}
				<a
					href="/login"
					class="w-full rounded-2xl bg-[#CCFF00] px-10 py-3 text-center text-[1.1rem] font-bold text-[#191F00] shadow-sm transition-all hover:bg-[#A9D207] lg:w-auto"
				>
					Увійти для реєстрації
				</a>
			{/if}
			<a
				href={resolve('/tournaments')}
				class="w-full rounded-2xl border border-[#E5E7EB] px-8 py-3 text-center text-sm font-bold text-[#191F00] transition hover:bg-[#F4F4F5] lg:w-auto"
			>
				До списку турнірів
			</a>
			{#if tournament.status === 'finished'}
				<a
					href={`/tournaments/results?tournament_id=${tournament.id}`}
					class="w-full rounded-2xl border border-[#191F00] px-8 py-3 text-center text-sm font-bold text-[#191F00] transition hover:bg-[#191F00] hover:text-white lg:w-auto"
				>
					Таблиця лідерів
				</a>
			{/if}
		</div>
	</section>

	{#if form?.message}
		<div class="mb-8 rounded-xl border border-red-200 bg-red-50 px-5 py-4 text-sm font-semibold text-red-700">
			<span class="text-anywhere block">{form.message}</span>
		</div>
	{/if}

	<section class="grid grid-cols-1 gap-8 xl:grid-cols-[minmax(0,1fr)_25rem]">
		<div class="space-y-8">
			<div class="rounded-2xl border border-[#E5E7EB] bg-white p-6 shadow-sm lg:p-8">
				<h2 class="mb-5 text-2xl font-bold">Правила</h2>
				<p class="text-anywhere whitespace-pre-line text-[1rem] leading-relaxed">{tournament.rules}</p>
			</div>

			{#if tournament.active_round}
				<div class="rounded-2xl bg-[#191F00] p-7 text-white lg:p-10">
					<div class="flex flex-col gap-6 lg:flex-row lg:items-center lg:justify-between">
						<div class="min-w-0">
							<p class="mb-2 text-sm font-semibold text-[#CCFF00]">Активний раунд</p>
							<h2 class="text-anywhere text-2xl font-bold">{tournament.active_round.title}</h2>
							<p class="text-anywhere mt-3 text-sm opacity-80">
								Дедлайн: {formatDate(tournament.active_round.deadline_at)}
							</p>
						</div>
						<a
							href={resolve('/rounds/[round_id]', {
								round_id: String(tournament.active_round.id)
							})}
							class="rounded-xl bg-white px-8 py-3 text-center font-bold text-[#191F00] transition hover:bg-[#CBCBCB]"
						>
							Подивитися завдання
						</a>
					</div>
				</div>
			{/if}

			<div class="rounded-2xl border border-[#E5E7EB] bg-white p-6 shadow-sm lg:p-8">
				<div class="mb-6 flex flex-col gap-2 sm:flex-row sm:items-center sm:justify-between">
					<h2 class="text-2xl font-bold">Раунди</h2>
					<span class="text-sm font-semibold text-gray-500">{rounds.length} у списку</span>
				</div>

				<div class="space-y-4">
					{#each rounds as round (round.id)}
						<div class="rounded-xl border border-[#E5E7EB] bg-[#FAFAFA] p-5">
							<div class="flow-root">
								<div class="mb-4 flex flex-col gap-2 md:float-right md:mb-3 md:ml-5 md:w-40 md:items-stretch">
									<a
										href={resolve('/rounds/[round_id]', { round_id: String(round.id) })}
										class="whitespace-normal break-normal rounded-lg border border-[#191F00] px-4 py-2 text-center text-sm font-bold text-[#191F00] hover:bg-[#191F00] hover:text-white"
									>
										Відкрити
									</a>
									{#if data.canManage}
										{#each roundActions(round, tournament.status) as action (action.status)}
											<form method="POST" action="?/changeRoundStatus">
												<input type="hidden" name="round_id" value={round.id} />
												<input type="hidden" name="status" value={action.status} />
												<button
													type="submit"
													class="w-full whitespace-normal break-normal rounded-lg bg-[#CCFF00] px-4 py-2 text-sm font-bold text-[#191F00] hover:bg-[#A9D207]"
												>
													{action.label}
												</button>
											</form>
										{/each}
										{#if canOperateRounds && round.status === 'active'}
											<form method="POST" action="?/lockSubmissions">
												<input type="hidden" name="round_id" value={round.id} />
												<button
													type="submit"
													class="w-full rounded-lg bg-[#191F00] px-4 py-2 text-sm font-bold text-white hover:bg-[#2b3500]"
												>
													Lock submissions
												</button>
											</form>
										{/if}
										{#if canOperateRounds && round.status === 'submission_closed'}
											<form method="POST" action="?/generateAssignments" class="grid gap-2 rounded-lg border border-[#E5E7EB] bg-white p-3">
												<input type="hidden" name="round_id" value={round.id} />
												<label class="text-xs font-bold text-[#696969]">
													Reviews
													<input
														name="reviews_per_submission"
														type="number"
														min="1"
														step="1"
														value="3"
														class="mt-1 w-full rounded border border-[#B4B4B4] px-2 py-1 text-sm"
													/>
												</label>
												<label class="text-xs font-bold text-[#696969]">
													Max per jury
													<input
														name="max_assignments_per_jury"
														type="number"
														min="1"
														step="1"
														value="5"
														class="mt-1 w-full rounded border border-[#B4B4B4] px-2 py-1 text-sm"
													/>
												</label>
												<button
													type="submit"
													class="rounded-lg bg-[#CCFF00] px-4 py-2 text-sm font-bold text-[#191F00] hover:bg-[#A9D207]"
												>
													Generate assignments
												</button>
											</form>
										{/if}
									{/if}
								</div>

								<div class="mb-2 flex flex-wrap items-center gap-3">
									<h3 class="text-anywhere text-lg font-bold">{round.position}. {round.title}</h3>
									<StateTag variant={round.status} />
								</div>
								<p class="text-anywhere line-clamp-2 overflow-hidden text-sm text-gray-700">{round.task_description}</p>
								<p class="text-anywhere mt-2 text-xs font-semibold text-gray-500">
									{formatDate(round.starts_at)} - {formatDate(round.deadline_at)}
								</p>
							</div>
						</div>
					{:else}
						<p class="rounded-xl bg-[#F4F4F5] px-5 py-6 text-center font-semibold text-gray-600">
							Раундів ще немає.
						</p>
					{/each}
				</div>
			</div>

			<div class="rounded-2xl border border-[#E5E7EB] bg-white p-6 shadow-sm lg:p-8">
				<div class="mb-6 flex flex-col gap-2 sm:flex-row sm:items-center sm:justify-between">
					<h2 class="text-2xl font-bold">Зареєстровані команди</h2>
					<span class="text-sm font-semibold text-gray-500">{registeredTeams.length} у списку</span>
				</div>

				{#if registeredTeams.length > 0}
					<div class="grid gap-4">
						{#each registeredTeams as team (team.id)}
							<div class="rounded-xl border border-[#E5E7EB] bg-[#FAFAFA] p-5">
								<div class="flex flex-col gap-2 sm:flex-row sm:items-center sm:justify-between">
									<h3 class="text-anywhere text-lg font-bold">{team.name}</h3>
									<span class="rounded-full bg-[#191F00] px-4 py-1.5 text-xs font-semibold text-[#CCFF00]">
										{team.members_count} учасників
									</span>
								</div>
							</div>
						{/each}
					</div>
				{:else}
					<p class="rounded-xl bg-[#F4F4F5] px-5 py-6 text-center font-semibold text-gray-600">
						Команди ще не зареєстровані.
					</p>
				{/if}
			</div>
		</div>

		<aside class="space-y-6">
			<div class="rounded-2xl border border-[#E5E7EB] bg-white p-6 shadow-sm">
				<h2 class="mb-5 text-xl font-bold">Розклад</h2>
				<div class="space-y-4 text-sm">
					<div>
						<p class="font-semibold text-gray-500">Реєстрація</p>
						<p class="text-anywhere">{formatDate(tournament.registration_starts_at)}</p>
						<p class="text-anywhere">{formatDate(tournament.registration_ends_at)}</p>
					</div>
					<div>
						<p class="font-semibold text-gray-500">Початок турніру</p>
						<p class="text-anywhere">{formatDate(tournament.starts_at)}</p>
					</div>
					<div>
						<p class="font-semibold text-gray-500">Максимум команд</p>
						<p>{tournament.max_teams ?? 'Без ліміту'}</p>
					</div>
					<div>
						<p class="font-semibold text-gray-500">Зареєстровано команд</p>
						<p>{tournament.registered_teams_count}</p>
					</div>
				</div>
			</div>

			{#if data.canManage}
				{#if canManageJury}
					<div class="rounded-2xl border border-[#E5E7EB] bg-white p-6 shadow-sm">
						<a href={`/choose_jury?tournament_id=${tournament.id}`} class="flex w-full items-center justify-center gap-4 rounded-2xl border border-[#191F00] px-5 py-3 hover:ring-1">
							<div class="flex h-9 w-9 items-center justify-center rounded-full">
								<img src="/icons/hummer.svg" alt="" class="h-9 w-9" />
							</div>
							<span class="text-lg font-semibold">Обрати журі</span>
						</a>
					</div>
				{/if}

				<div class="rounded-2xl border border-[#E5E7EB] bg-white p-6 shadow-sm">
					<h2 class="mb-4 text-xl font-bold">Organizer controls</h2>
					<div class="flex flex-col gap-3">
						{#each tournamentActions(tournament.status) as action (action.status)}
							<form method="POST" action="?/changeTournamentStatus">
								<input type="hidden" name="status" value={action.status} />
								<button
									type="submit"
									class="w-full rounded-xl px-5 py-3 text-sm font-bold transition {action.secondary
										? 'border border-[#191F00] bg-white text-[#191F00] hover:bg-[#F4F4F5]'
										: 'bg-[#CCFF00] text-[#191F00] hover:bg-[#A9D207]'}"
								>
									{action.label}
								</button>
							</form>
						{:else}
							<p class="text-sm font-semibold text-gray-500">Для цього статусу немає доступних переходів.</p>
						{/each}
					</div>
				</div>

				{#if canSetupRounds}
					<form method="POST" action="?/createRound" class="rounded-2xl border border-[#E5E7EB] bg-white p-6 shadow-sm">
						<h2 class="mb-4 text-xl font-bold">Додати раунд</h2>
						<div class="space-y-4">
							<InputField name="title" required header="Назва*" placeholder="Round 2: Final" />
							<TextArea
								name="task_description"
								required
								header="Опис завдання*"
								placeholder="Опишіть задачу раунду"
								rows={4}
							/>
							<TextArea
								name="technology_requirements"
								header="Технології"
								placeholder="Будь-які вимоги до стеку"
								rows={2}
							/>
							<div class="grid grid-cols-1 gap-4">
								<DateField name="starts_at" required header="Початок*" />
								<DateField name="deadline_at" required header="Дедлайн*" />
								<InputField name="position" type="number" header="Позиція" placeholder="Автоматично" />
							</div>
							<div class="space-y-3">
								<div class="flex items-center justify-between">
									<p class="text-sm font-bold text-[#32221B]">Must-have</p>
									<button type="button" on:click={addRoundRequirement} class="text-sm font-bold hover:underline">
										Додати
									</button>
								</div>
								{#each newRoundRequirements as requirement, index (index)}
									<div class="flex items-end gap-2" data-filled={Boolean(requirement)}>
										<InputField
											bind:value={newRoundRequirements[index]}
											name="must_have"
											header=""
											placeholder="Вимога {index + 1}"
											class="w-full"
										/>
										<button
											type="button"
											on:click={() => removeRoundRequirement(index)}
											class="mb-2 rounded-lg border border-[#B4B4B4] px-3 py-2 text-sm font-bold hover:bg-[#F4F4F5]"
											aria-label="Видалити вимогу"
										>
											×
										</button>
									</div>
								{/each}
							</div>
							<button
								type="submit"
								class="w-full rounded-xl bg-[#191F00] px-5 py-3 text-sm font-bold text-white hover:bg-[#2b3500]"
							>
								Створити раунд
							</button>
						</div>
					</form>
				{/if}
			{/if}
		</aside>
	</section>
</main>
