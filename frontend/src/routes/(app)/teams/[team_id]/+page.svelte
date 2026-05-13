<script>
	import StateTag from '../../../../components/StateTag.svelte';
	import InputField from '/src/components/form/InputField.svelte';

	export let data;
	export let form;

	$: team = data.team;
	$: tournament = data.tournament;
	$: currentMember = team.members.find((member) => member.user_id === data.user.id);
	$: canManage = currentMember?.role === 'captain' || !currentMember;
	$: updateValues = {
		name: form?.values?.name ?? team.name,
		organization: form?.values?.organization ?? team.organization ?? '',
		contact: form?.values?.contact ?? team.contact ?? ''
	};

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
	<title>Hucky - {team.name}</title>
</svelte:head>

<main class="w-full px-6 py-10 font-sans text-[#191F00] lg:px-12 lg:py-16 xl:px-16">
	<section class="mb-10 flex flex-col gap-6 lg:flex-row lg:items-end lg:justify-between">
		<div class="min-w-0">
			<div class="mb-4 flex flex-wrap items-center gap-4">
				<h1 class="text-anywhere text-3xl font-bold lg:text-[2.75rem]">{team.name}</h1>
				{#if currentMember}
					<span class="rounded-full bg-[#191F00] px-4 py-1.5 text-sm font-semibold text-[#CCFF00]">
						{currentMember.role === 'captain' ? 'Капітан' : 'Учасник'}
					</span>
				{/if}
			</div>
			{#if tournament}
				<a href={`/tournaments/${tournament.id}`} class="text-anywhere text-base font-semibold text-[#516600] underline">
					{tournament.title}
				</a>
			{:else}
				<p class="text-sm font-semibold text-[#696969]">Tournament ID: {team.tournament_id}</p>
			{/if}
		</div>
		<a
			href="/profile"
			class="rounded-xl border border-[#E5E7EB] px-6 py-3 text-center text-sm font-bold text-[#191F00] hover:bg-[#F4F4F5]"
		>
			До профілю
		</a>
	</section>

	{#if form?.message}
		<div class="mb-8 rounded-xl border border-red-200 bg-red-50 px-5 py-4 text-sm font-semibold text-red-700">
			<span class="text-anywhere block">{form.message}</span>
		</div>
	{/if}

	<section class="grid grid-cols-1 gap-8 xl:grid-cols-[minmax(0,1fr)_25rem]">
		<div class="space-y-8">
			<div class="rounded-2xl border border-[#E5E7EB] bg-white p-6 shadow-sm lg:p-8">
				<h2 class="mb-6 text-2xl font-bold">Учасники ({team.members.length})</h2>
				<div class="space-y-4">
					{#each team.members as member (member.user_id)}
						<div class="rounded-xl bg-[#CCFF00] p-5">
							<div class="flex flex-col gap-4 sm:flex-row sm:items-start sm:justify-between">
								<div class="min-w-0">
									<div class="mb-2 flex flex-wrap items-center gap-3">
										<h3 class="text-anywhere text-lg font-bold">{member.full_name}</h3>
										{#if member.role === 'captain'}
											<span class="rounded-full bg-[#191F00] px-4 py-1 text-xs font-semibold text-[#CCFF00]">
												Капітан
											</span>
										{/if}
									</div>
									<p class="text-anywhere text-sm font-semibold text-[#516600]">{member.email}</p>
									<p class="mt-2 text-xs font-semibold text-[#516600]">Статус: {member.status}</p>
								</div>
								{#if canManage && member.role !== 'captain' && member.status === 'accepted'}
									<form method="POST" action="?/removeMember">
										<input type="hidden" name="user_id" value={member.user_id} />
										<button
											type="submit"
											class="rounded-lg bg-white px-4 py-2 text-sm font-bold text-red-600 hover:bg-red-50"
										>
											Видалити
										</button>
									</form>
								{/if}
							</div>
						</div>
					{/each}
				</div>
			</div>

			<div class="rounded-2xl border border-[#E5E7EB] bg-white p-6 shadow-sm lg:p-8">
				<div class="mb-6 flex flex-col gap-2 sm:flex-row sm:items-center sm:justify-between">
					<h2 class="text-2xl font-bold">Pending-запрошення</h2>
					<span class="text-sm font-semibold text-gray-500">{team.pending_invitations.length} у списку</span>
				</div>

				{#if team.pending_invitations.length > 0}
					<div class="space-y-4">
						{#each team.pending_invitations as invitation (invitation.id)}
							<div class="rounded-xl border border-[#E5E7EB] bg-[#FAFAFA] p-5">
								<div class="flex flex-col gap-4 sm:flex-row sm:items-center sm:justify-between">
									<div class="min-w-0">
										<p class="text-anywhere font-bold">{invitation.email}</p>
										<p class="text-anywhere mt-1 text-xs font-semibold text-[#696969]">
											Дійсне до: {formatDate(invitation.expires_at)}
										</p>
									</div>
									{#if canManage}
										<form method="POST" action="?/cancelInvitation">
											<input type="hidden" name="invitation_id" value={invitation.id} />
											<button
												type="submit"
												class="rounded-lg border border-[#B4B4B4] bg-white px-4 py-2 text-sm font-bold text-[#191F00] hover:bg-[#F4F4F5]"
											>
												Скасувати
											</button>
										</form>
									{/if}
								</div>
							</div>
						{/each}
					</div>
				{:else}
					<p class="rounded-xl bg-[#F4F4F5] px-5 py-6 text-center font-semibold text-gray-600">
						Немає pending-запрошень.
					</p>
				{/if}
			</div>
		</div>

		<aside class="space-y-6">
			<div class="rounded-2xl border border-[#E5E7EB] bg-white p-6 shadow-sm">
				<h2 class="mb-5 text-xl font-bold">Дані команди</h2>
				<div class="space-y-4 text-sm">
					<div>
						<p class="font-semibold text-gray-500">Організація</p>
						<p class="text-anywhere">{team.organization || 'Не вказано'}</p>
					</div>
					<div>
						<p class="font-semibold text-gray-500">Контакт</p>
						<p class="text-anywhere">{team.contact || 'Не вказано'}</p>
					</div>
					{#if tournament}
						<div>
							<p class="font-semibold text-gray-500">Статус турніру</p>
							<div class="mt-2"><StateTag variant={tournament.status} /></div>
						</div>
					{/if}
				</div>
			</div>

			{#if canManage}
				<form method="POST" action="?/updateTeam" class="rounded-2xl border border-[#E5E7EB] bg-white p-6 shadow-sm">
					<h2 class="mb-5 text-xl font-bold">Редагувати</h2>
					<div class="space-y-4">
						<InputField name="name" required header="Назва*" value={updateValues.name} />
						<InputField name="organization" header="Організація" value={updateValues.organization} />
						<InputField name="contact" header="Контакт" value={updateValues.contact} />
						<button
							type="submit"
							class="w-full rounded-xl bg-[#191F00] px-5 py-3 text-sm font-bold text-white hover:bg-[#2b3500]"
						>
							Зберегти
						</button>
					</div>
				</form>

				<form
					method="POST"
					action="?/createInvitation"
					class="rounded-2xl border border-[#E5E7EB] bg-white p-6 shadow-sm"
				>
					<h2 class="mb-5 text-xl font-bold">Запросити учасника</h2>
					<div class="space-y-4">
						<InputField name="email" required type="email" header="Email*" placeholder="member@example.com" />
						<button
							type="submit"
							class="w-full rounded-xl bg-[#CCFF00] px-5 py-3 text-sm font-bold text-[#191F00] hover:bg-[#A9D207]"
						>
							Надіслати інвайт
						</button>
					</div>
				</form>
			{/if}
		</aside>
	</section>
</main>
