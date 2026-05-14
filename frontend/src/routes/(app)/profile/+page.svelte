<script>
	import StateTag from '../../../components/StateTag.svelte';
	import TournamentCard from '../../../components/TournamentCard.svelte';
	import { tick } from 'svelte';
	import { avatarSrc, DEFAULT_AVATAR, isDefaultAvatar, setAvatar } from '$lib/avatar';

	export let data;
	export let form;

	let hoverAvatar = '/icons/avatar_change.svg';
	let avatarInput;
	let avatarForm;
	let avatarUrl = '';
	let avatarError = '';
	let isAvatarHovering = false;

	$: shownAvatar = isAvatarHovering && isDefaultAvatar($avatarSrc) ? hoverAvatar : $avatarSrc;
	$: setAvatar(data.profile?.avatar_url);

	$: role = data.profile?.role || 'participant';
	$: isParticipant = role === 'participant';
	$: isOrganiser = role === 'organiser';
	$: isJury = role === 'jury';
	$: isAdmin = role === 'admin';
	$: createdTournaments = data.createdTournaments || [];
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

	function openAvatarPicker() {
		avatarError = '';
		avatarInput?.click();
	}

	function handleAvatarChange(event) {
		const file = event.currentTarget.files?.[0];
		event.currentTarget.value = '';

		if (!file) return;
		if (!file.type.startsWith('image/')) {
			avatarError = 'Оберіть файл зображення.';
			return;
		}
		if (file.size > 900 * 1024) {
			avatarError = 'Зображення має бути не більше 900 КБ.';
			return;
		}

		const reader = new FileReader();
		reader.onload = async () => {
			if (typeof reader.result === 'string') {
				avatarUrl = reader.result;
				setAvatar(reader.result);
				avatarError = '';
				await tick();
				avatarForm?.requestSubmit();
			}
		};
		reader.onerror = () => {
			avatarError = 'Не вдалося прочитати зображення.';
		};
		reader.readAsDataURL(file);
	}
</script>

<svelte:head>
	<title>Hucky - Профіль</title>
</svelte:head>

<main class="w-full px-6 py-10 font-sans text-[#191F00] md:px-16 lg:px-28 lg:py-20">
	<section class="mb-12 flex flex-col items-center gap-8 text-center md:flex-row md:flex-wrap md:items-start md:gap-12 md:text-left lg:gap-14 xl:flex-nowrap">
		<div class="flex shrink-0 flex-col items-center gap-3 md:items-start">
			<button
				type="button"
				class="group relative h-24 w-24 overflow-hidden rounded-full border border-[#B4B4B4] bg-white transition hover:ring-2 hover:ring-[#CCFF00] focus:outline-none focus:ring-2 focus:ring-[#191F00] lg:h-35 lg:w-35"
				aria-label="Змінити фото профілю"
				on:click={openAvatarPicker}
				on:mouseenter={() => (isAvatarHovering = true)}
				on:mouseleave={() => (isAvatarHovering = false)}
			>
				<img
					src={shownAvatar || DEFAULT_AVATAR}
					alt="Profile Avatar"
					class="h-full w-full object-cover transition-all duration-300"
				/>
				<span class="absolute inset-x-0 bottom-0 bg-[#191F00]/80 px-2 py-1 text-xs font-semibold text-white opacity-0 transition group-hover:opacity-100">
					Змінити
				</span>
			</button>
			<input
				bind:this={avatarInput}
				type="file"
				accept="image/*"
				class="sr-only"
				on:change={handleAvatarChange}
			/>
			<form bind:this={avatarForm} method="POST" action="?/updateAvatar" class="hidden">
				<input type="hidden" name="avatar_url" value={avatarUrl} />
			</form>
			{#if avatarError}
				<p class="max-w-36 text-center text-xs font-semibold text-red-600 md:text-left">{avatarError}</p>
			{/if}
		</div>
		<div class="min-w-0 w-full pt-0 md:min-w-64 md:flex-1 md:pt-4">
			<div class="mb-6 flex flex-col items-center gap-4 md:flex-row md:flex-wrap md:items-center lg:gap-10">
				<h1 class="text-anywhere text-3xl leading-tight font-bold lg:text-[3rem]">{data.profile.full_name}</h1>
				<StateTag variant={role} />
			</div>
			<div class="flex flex-col items-center gap-2 md:flex-row md:items-start md:gap-4 lg:gap-6">
				<span class="shrink-0 whitespace-nowrap text-lg font-semibold lg:text-[1.4rem]">Пошта:</span>
				<span class="text-anywhere min-w-0 text-lg leading-tight lg:text-[1.4rem]">{data.profile.email}</span>
			</div>
		</div>
		{#if isOrganiser || isAdmin}
			<div class="grid w-full grid-cols-1 gap-3 sm:max-w-md sm:grid-cols-2 md:basis-full md:max-w-2xl xl:w-80 xl:basis-auto xl:grid-cols-1 xl:max-w-80 xl:min-w-72">
				{#if isOrganiser}
					<a href="/tournaments/new" class="flex min-w-0 items-center justify-center gap-3 rounded-2xl border border-[#191F00] px-4 py-3 hover:ring-1 md:justify-start lg:gap-4 lg:px-5 lg:py-4">
						<div class="flex aspect-square h-8 shrink-0 items-center justify-center rounded-full bg-[#CCFF00] text-xl font-semibold leading-none lg:h-10 lg:text-[1.65rem]">
							+
						</div>
						<span class="min-w-0 text-left text-base font-semibold leading-tight sm:text-lg lg:text-[1.25rem] xl:text-[1.5rem]">
							Створити новий турнір
						</span>
					</a>

					<a href="/choose_jury" class="flex min-w-0 items-center justify-center gap-3 rounded-2xl border border-[#191F00] px-4 py-3 hover:ring-1 md:justify-start lg:gap-4 lg:px-5 lg:py-4">
						<div class="flex aspect-square h-9 shrink-0 items-center justify-center rounded-full lg:h-10">
							<img src="/icons/hummer.svg" alt="" class="h-9 w-9 shrink-0 lg:h-9 lg:w-9" />
						</div>
						<span class="min-w-0 text-left text-base font-semibold leading-tight sm:text-lg lg:text-[1.25rem] xl:text-[1.5rem]">
							Обрати журі
						</span>
					</a>
				{/if}
				{#if isAdmin}
					<a href="/admin_panel" class="flex min-w-0 items-center justify-center gap-3 rounded-2xl border border-[#191F00] px-4 py-3 hover:ring-1 md:justify-start lg:gap-4 lg:px-5 lg:py-4">
						<div class="flex aspect-square h-9 shrink-0 items-center justify-center rounded-full lg:h-10">
							<img src="/icons/top-admin.svg" alt="" class="h-8 w-8 shrink-0 lg:h-9 lg:w-9" />
						</div>
						<span class="min-w-0 text-left text-base font-semibold leading-tight sm:text-lg lg:text-[1.25rem] xl:text-[1.5rem]">
							Панель адміністратора
						</span>
					</a>
				{/if}
			</div>
		{/if}
	</section>

	{#if isOrganiser}
		<section class="mb-12">
			<h2 class="mb-8 text-center text-xl font-semibold md:ml-32 md:text-left lg:mb-13 lg:ml-46 lg:text-[1.5rem]">
				Турніри в управлінні:
			</h2>
			<div class="grid max-w-7xl grid-cols-1 gap-8 sm:grid-cols-2 md:ml-32 lg:ml-46 lg:gap-12">
				{#each createdTournaments as tournament}
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
					<p class="rounded-xl bg-[#F4F4F5] px-6 py-8 text-center font-semibold text-[#696969] sm:col-span-2">
						Поки що немає турнірів в управлінні.
					</p>
				{/each}
			</div>
		</section>
	{/if}

	{#if form?.message}
		<div class="mb-8 rounded-xl border border-red-200 bg-red-50 px-5 py-4 text-sm font-semibold text-red-700">
			<span class="text-anywhere block">{form.message}</span>
		</div>
	{/if}

	{#if isParticipant}
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
									<div class="min-w-0">
										<h3 class="text-anywhere text-xl font-bold">{item.team_name}</h3>
										<p class="text-anywhere mt-1 text-sm text-[#696969]">{item.tournament.title}</p>
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
								<h3 class="text-anywhere font-bold">{invitation.team_name}</h3>
								<p class="text-anywhere mt-1 text-sm text-[#696969]">{invitation.tournament_title}</p>
								<p class="text-anywhere mt-3 text-xs font-semibold text-[#696969]">
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
	{/if}

	{#if isJury}
		<section class="rounded-2xl border border-[#E5E7EB] bg-white p-6 shadow-sm lg:p-8">
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
								<div class="min-w-0">
									<h3 class="text-anywhere text-xl font-bold">{assignment.team_name}</h3>
									<p class="text-anywhere mt-1 text-sm text-[#696969]">{assignment.tournament_title}</p>
									<p class="text-anywhere mt-1 text-sm text-[#696969]">{assignment.round_title}</p>
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
		</section>
	{/if}
</main>
