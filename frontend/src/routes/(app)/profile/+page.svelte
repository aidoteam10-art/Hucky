<script>
	import StateTag from '../../../components/StateTag.svelte';

	export let data;
	export let form;

	let defaultAvatar = '/icons/avatar.svg';
	let hoverAvatar = '/icons/avatar_change.svg';
	let currentAvatar = defaultAvatar;

	$: hasTeams = data.teams.length > 0;
	$: hasInvitations = data.invitations.length > 0;
	$: hasJuryAssignments = data.juryAssignments.length > 0;

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
</script>

<svelte:head>
	<title>Hucky - Профіль</title>
</svelte:head>

<main class="w-full px-6 py-10 font-sans text-[#191F00] md:px-16 lg:px-28 lg:py-20">
	<section class="mb-12 flex flex-col items-center gap-8 text-center md:flex-row md:items-start md:text-left">
		<img
			src={currentAvatar}
			alt="Profile Avatar"
			class="h-24 w-24 cursor-pointer pt-0 transition-all duration-500 lg:h-35 lg:w-35 md:pt-4"
			on:mouseenter={() => (currentAvatar = hoverAvatar)}
			on:mouseleave={() => (currentAvatar = defaultAvatar)}
		/>
            {#if role === 'admin'}
                <div class="flex flex-col gap-4 w-full md:w-auto">
                    <a href="/tournaments/new" class="flex items-center gap-4 lg:gap-6 border border-[#191F00] rounded-2xl px-5 lg:px-7.5 py-3 lg:py-4 hover:ring-1 w-full md:w-auto justify-center">
                        <div class="bg-[#CCFF00] w-8 h-8 lg:w-10 lg:h-10 rounded-full flex items-center justify-center text-xl lg:text-[1.65rem] font-semibold leading-none">
                            +
                        </div>
                        <span class="text-lg lg:text-[1.5rem] font-semibold">Створити новий турнір</span>
                    </a>

                    <a href="/choose_jury" class="flex items-center gap-4 lg:gap-6 border border-[#191F00] rounded-2xl px-5 lg:px-7.5 py-3 lg:py-4 hover:ring-1 w-full md:w-auto justify-center">
                        <div class=" w-9 h-9 lg:w-10 lg:h-10 rounded-full flex items-center justify-center">
                            <img src="/icons/hummer.svg" alt="" class="w-9 h-9 lg:w-9 lg:h-9">
                        </div>
                        <span class="text-lg lg:text-[1.5rem] font-semibold">Обрати журі</span>
                    </a>
                </div>
            {/if}
        </div>
    </section>

    {#if role === 'admin'}
        <h2 class="text-xl lg:text-[1.5rem] font-semibold mb-8 lg:mb-13 md:ml-32 lg:ml-46 text-center md:text-left">
            Створені вами турніри:
        </h2>
        <div class="grid grid-cols-1 sm:grid-cols-2 gap-8 lg:gap-12 max-w-7xl md:ml-32 lg:ml-46">
            {#each user.createdTournaments as tournament}
                <TournamentCard
                        variant="grey"
                        current_state={tournament.current_state}
                        title={tournament.title}
                        description={tournament.description}
                        start_date={tournament.start_date}
                        rounds={tournament.rounds}
                        max_teams={tournament.max_teams}
                        registered_teams={tournament.registered_teams}
                        id={tournament.id}
                />
            {/each}
        </div>
    {/if}

		<div class="w-full pt-0 md:pt-4">
			<div class="mb-6 flex flex-col items-center gap-4 md:flex-row lg:gap-10">
				<h1 class="text-3xl leading-tight font-bold lg:text-[3rem]">{data.profile.full_name}</h1>
				{#if hasTeams}
					<StateTag variant="participant" />
				{/if}
			</div>
			<div class="flex flex-col items-center gap-2 md:flex-row md:gap-6">
				<span class="text-lg font-semibold lg:text-[1.4rem]">Пошта:</span>
				<span class="text-lg lg:text-[1.4rem]">{data.profile.email}</span>
			</div>
		</div>
	</section>

	{#if form?.message}
		<div class="mb-8 rounded-xl border border-red-200 bg-red-50 px-5 py-4 text-sm font-semibold text-red-700">
			{form.message}
		</div>
	{/if}

	<section class="grid grid-cols-1 gap-8 xl:grid-cols-[minmax(0,1fr)_24rem]">
		<div class="space-y-8">
			<div class="rounded-2xl border border-[#E5E7EB] bg-white p-6 shadow-sm lg:p-8">
				<div class="mb-6 flex flex-col gap-2 sm:flex-row sm:items-center sm:justify-between">
					<h2 class="text-2xl font-bold">Мої команди</h2>
					<span class="text-sm font-semibold text-gray-500">{data.teams.length} у списку</span>
				</div>

				{#if hasTeams}
					<div class="grid gap-4">
						{#each data.teams as item (item.team_id)}
							<a
								href={`/teams/${item.team_id}`}
								class="block rounded-xl border border-[#E5E7EB] bg-[#FAFAFA] p-5 transition hover:border-[#CCFF00] hover:bg-[#FBFFE9]"
							>
								<div class="mb-4 flex flex-col gap-3 sm:flex-row sm:items-start sm:justify-between">
									<div>
										<h3 class="text-xl font-bold">{item.team_name}</h3>
										<p class="mt-1 text-sm text-[#696969]">{item.tournament.title}</p>
									</div>
									<div class="flex flex-wrap gap-2">
										<span class="rounded-full bg-[#191F00] px-4 py-1.5 text-xs font-semibold text-[#CCFF00]">
											{item.role === 'captain' ? 'Капітан' : 'Учасник'}
										</span>
										<StateTag variant={item.tournament.status} />
									</div>
								</div>
								<div class="grid grid-cols-1 gap-3 text-sm text-[#696969] sm:grid-cols-2">
									<p><span class="font-semibold text-[#191F00]">Статус:</span> {item.status}</p>
									<p><span class="font-semibold text-[#191F00]">Учасників:</span> {item.members_count}</p>
								</div>
							</a>
						{/each}
					</div>
				{:else}
					<div class="rounded-xl bg-[#F4F4F5] px-6 py-8 text-center">
						<p class="mb-5 font-semibold text-[#696969]">
							У вас ще немає команд. Оберіть турнір з відкритою реєстрацією та створіть команду.
						</p>
						<a
							href="/tournaments"
							class="inline-flex rounded-xl bg-[#CCFF00] px-7 py-3 font-bold text-[#191F00] hover:bg-[#A9D207]"
						>
							Перейти до турнірів
						</a>
					</div>
				{/if}
			</div>

			<div class="rounded-2xl border border-[#E5E7EB] bg-white p-6 shadow-sm lg:p-8">
				<div class="mb-6 flex flex-col gap-2 sm:flex-row sm:items-center sm:justify-between">
					<h2 class="text-2xl font-bold">Мої оцінювання</h2>
					<span class="text-sm font-semibold text-gray-500">{data.juryAssignments.length} у списку</span>
				</div>

				{#if hasJuryAssignments}
					<div class="grid gap-4">
						{#each data.juryAssignments as assignment (assignment.id)}
							<a
								href={`/tournaments/results/${assignment.id}`}
								class="block rounded-xl border border-[#E5E7EB] p-5 transition {assignment.status === 'pending'
									? 'bg-[#FBFFE9] hover:border-[#CCFF00]'
									: 'bg-[#FAFAFA] hover:border-[#B4B4B4]'}"
							>
								<div class="mb-4 flex flex-col gap-3 sm:flex-row sm:items-start sm:justify-between">
									<div>
										<h3 class="text-xl font-bold">{assignment.team_name}</h3>
										<p class="mt-1 text-sm text-[#696969]">{assignment.tournament_title}</p>
										<p class="mt-1 text-sm text-[#696969]">{assignment.round_title}</p>
									</div>
									<span class="rounded-full bg-[#191F00] px-4 py-1.5 text-xs font-semibold text-[#CCFF00]">
										{assignment.status === 'pending' ? 'Очікує' : 'Оцінено'}
									</span>
								</div>
								<p class="text-sm font-semibold text-[#696969]">
									Submitted: {formatDate(assignment.submitted_at)}
								</p>
							</a>
						{/each}
					</div>
				{:else}
					<p class="rounded-xl bg-[#F4F4F5] px-5 py-6 text-center text-sm font-semibold text-[#696969]">
						У вас немає призначених робіт для оцінювання.
					</p>
				{/if}
			</div>
		</div>

		<aside class="space-y-8">
			<div class="rounded-2xl border border-[#E5E7EB] bg-white p-6 shadow-sm">
				<div class="mb-5 flex items-center justify-between gap-3">
					<h2 class="text-xl font-bold">Запрошення</h2>
					<span class="rounded-full bg-[#F4F4F5] px-3 py-1 text-xs font-bold text-[#696969]">
						{data.invitations.length}
					</span>
				</div>

				{#if hasInvitations}
					<div class="space-y-4">
						{#each data.invitations as invitation (invitation.id)}
							<div class="rounded-xl border border-[#E5E7EB] bg-[#FAFAFA] p-4">
								<h3 class="font-bold">{invitation.team_name}</h3>
								<p class="mt-1 text-sm text-[#696969]">{invitation.tournament_title}</p>
								<p class="mt-3 text-xs font-semibold text-[#696969]">
									Від: {invitation.invited_by.full_name}
								</p>
								<p class="mt-1 text-xs font-semibold text-[#696969]">
									Дійсне до: {formatDate(invitation.expires_at)}
								</p>
								<div class="mt-4 grid grid-cols-2 gap-2">
									<form method="POST" action="?/acceptInvitation">
										<input type="hidden" name="invitation_id" value={invitation.id} />
										<button
											type="submit"
											class="w-full rounded-lg bg-[#CCFF00] px-4 py-2 text-sm font-bold text-[#191F00] hover:bg-[#A9D207]"
										>
											Прийняти
										</button>
									</form>
									<form method="POST" action="?/declineInvitation">
										<input type="hidden" name="invitation_id" value={invitation.id} />
										<button
											type="submit"
											class="w-full rounded-lg border border-[#B4B4B4] bg-white px-4 py-2 text-sm font-bold text-[#191F00] hover:bg-[#F4F4F5]"
										>
											Відхилити
										</button>
									</form>
								</div>
							</div>
						{/each}
					</div>
				{:else}
					<p class="rounded-xl bg-[#F4F4F5] px-5 py-6 text-center text-sm font-semibold text-[#696969]">
						Немає pending-запрошень.
					</p>
				{/if}
			</div>
		</aside>
	</section>
</main>
