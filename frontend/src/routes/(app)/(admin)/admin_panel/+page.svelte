<script>
	import { avatarSrc } from '$lib/avatar';

	export let data;
	export let form;

	const roles = {
		participant: 'Учасник',
		organiser: 'Організатор',
		jury: 'Журі',
		admin: 'Адмін'
	};

	const roleOptions = [
		{ id: 'all', label: 'Усі' },
		{ id: 'participant', label: roles.participant },
		{ id: 'organiser', label: roles.organiser },
		{ id: 'jury', label: roles.jury },
		{ id: 'admin', label: roles.admin }
	];

	$: users = data.users?.items || [];
	$: counts = {
		all: data.users?.total || users.length,
		participant: users.filter((user) => user.role === 'participant').length,
		organiser: users.filter((user) => user.role === 'organiser').length,
		jury: users.filter((user) => user.role === 'jury').length,
		admin: users.filter((user) => user.role === 'admin').length
	};
	$: currentRole = data.filters?.role || 'all';
	$: search = data.filters?.search || '';

	function queryFor(role) {
		const params = [];
		if (role && role !== 'all') params.push(`role=${encodeURIComponent(role)}`);
		if (search) params.push(`search=${encodeURIComponent(search)}`);
		const query = params.join('&');
		return query ? `/admin_panel?${query}` : '/admin_panel';
	}

	function roleClass(role) {
		if (role === 'participant') return 'text-[#CCFF00]';
		if (role === 'organiser') return 'text-sky-300 ring-1 ring-sky-300';
		if (role === 'jury') return 'text-[#F97316] ring-1 ring-[#F97316]';
		return 'text-gray-300 ring-1 ring-gray-500';
	}

	function isCurrentUser(user) {
		return user.id === data.profile.id;
	}
</script>

<main class="min-h-[calc(100vh-5rem)] bg-white px-4 py-10 font-sans text-[#191F00] sm:px-8 lg:px-28 lg:py-24">
	<section class="mx-auto max-w-6xl">
		<div class="mb-8">
			<div class="mb-4 flex items-center gap-4">
				<img src="/icons/top-admin.svg" alt="" class="h-10 w-10 shrink-0" />
				<h1 class="text-3xl font-bold leading-tight sm:text-4xl">Найвищий адміністратор</h1>
			</div>
			<p class="max-w-3xl text-sm leading-6 text-[#696969] sm:text-base">
				Керування глобальними ролями користувачів.
			</p>
		</div>

		<section class="mb-6 rounded-xl border border-[#B4B4B4] bg-white p-5 sm:p-6">
			<div class="flex flex-col gap-4 sm:flex-row sm:items-center">
				<img src="/icons/admin_logo.svg" alt="admin" class="h-18 w-18" />
				<div class="min-w-0">
					<h2 class="break-words text-xl font-bold">{data.profile.full_name}</h2>
					<div class="mt-2 flex min-w-0 items-center gap-2 text-sm text-[#696969]">
						<img src="/icons/mail.svg" alt="" class="h-4 w-4 shrink-0 opacity-70" />
						<span class="break-all">{data.profile.email}</span>
					</div>
					<span class="mt-3 inline-flex rounded-full border border-[#FF4545] bg-[#FDECEC] px-3 py-1 text-xs font-medium text-[#FF4545]">
						Superadmin
					</span>
				</div>
			</div>
		</section>

		{#if form?.message}
			<div class="mb-6 rounded-xl border border-red-200 bg-red-50 px-5 py-4 text-sm font-semibold text-red-700">
				{form.message}
			</div>
		{/if}

		<section class="rounded-xl border border-[#B4B4B4] bg-white p-4 sm:p-5">
			<div class="mb-5 flex flex-col gap-4 lg:flex-row lg:items-center lg:justify-between">
				<h2 class="text-base font-bold">Усі користувачі ({counts.all})</h2>

				<div class="grid gap-3 sm:grid-cols-[minmax(0,1fr)_12rem] lg:w-[34rem]">
					<form method="GET" action="/admin_panel" class="relative block">
						{#if currentRole !== 'all'}
							<input type="hidden" name="role" value={currentRole} />
						{/if}
						<img
							src="/icons/search_icon.svg"
							alt=""
							class="pointer-events-none absolute left-3 top-1/2 h-4 w-4 -translate-y-1/2 opacity-70"
						/>
						<input
							name="search"
							value={search}
							type="search"
							placeholder="Пошук за іменем або поштою"
							class="h-11 w-full rounded-lg border border-[#B4B4B4] bg-white pl-10 pr-3 text-sm outline-none transition focus:border-[#191F00] focus:ring-1 focus:ring-[#191F00]"
						/>
					</form>

					<select
						value={currentRole}
						on:change={(event) => location.assign(queryFor(event.currentTarget.value))}
						class="h-11 w-full rounded-lg border border-[#B4B4B4] bg-white px-3 text-sm outline-none transition focus:border-[#191F00] focus:ring-1 focus:ring-[#191F00]"
					>
						{#each roleOptions as option}
							<option value={option.id}>
								{option.label} ({counts[option.id] ?? counts.all})
							</option>
						{/each}
					</select>
				</div>
			</div>

			<div class="overflow-hidden rounded-lg border border-[#B4B4B4]">
				<div class="hidden grid-cols-[minmax(18rem,1fr)_10rem_13rem] bg-[#CCFF00] px-5 py-3 text-xs font-bold uppercase md:grid">
					<span>Користувач</span>
					<span>Поточна роль</span>
					<span>Змінити роль</span>
				</div>

				<div class="divide-y divide-[#D8D8D8]">
					{#each users as user}
						<article class="grid gap-4 bg-white p-4 md:grid-cols-[minmax(18rem,1fr)_10rem_13rem] md:items-center md:px-5">
							<div class="flex min-w-0 items-start gap-4">
								<img
									src={isCurrentUser(user) ? $avatarSrc : user.avatar_url || '/icons/avatar.svg'}
									alt=""
									class="h-12 w-12 shrink-0 rounded-full bg-white object-cover"
								/>
								<div class="min-w-0">
									<h3 class="break-words text-sm font-bold">{user.full_name}</h3>
									<div class="mt-2 flex min-w-0 items-center gap-2 text-xs text-[#696969]">
										<img src="/icons/mail.svg" alt="" class="h-3.5 w-3.5 shrink-0 opacity-70" />
										<span class="break-all">{user.email}</span>
									</div>
								</div>
							</div>

							<div class="flex items-center justify-between gap-3 md:block">
								<span class="text-xs font-semibold uppercase text-[#696969] md:hidden">Поточна роль</span>
								<span class="inline-flex min-w-24 justify-center rounded-full bg-[#191F00] px-3 py-1 text-xs font-medium {roleClass(user.role)}">
									{roles[user.role]}
								</span>
							</div>

							{#if isCurrentUser(user)}
								<div class="rounded-lg border border-[#E5E7EB] bg-[#F4F4F5] px-3 py-2 text-sm font-semibold text-[#696969]">
									Власна роль захищена
								</div>
							{:else}
								<form method="POST" action="?/updateRole" class="grid gap-2 sm:grid-cols-[8rem_minmax(0,1fr)] sm:items-center md:block">
									<input type="hidden" name="user_id" value={user.id} />
									<span class="text-xs font-semibold uppercase text-[#696969] md:hidden">Змінити роль</span>
									<select
										name="role"
										value={user.role}
										on:change={(event) => event.currentTarget.form.requestSubmit()}
										class="h-10 w-full rounded-lg border border-[#B4B4B4] bg-white px-3 text-sm outline-none transition focus:border-[#191F00] focus:ring-1 focus:ring-[#191F00]"
									>
										<option value="participant">Учасник</option>
										<option value="organiser">Організатор</option>
										<option value="jury">Журі</option>
										<option value="admin">Адмін</option>
									</select>
								</form>
							{/if}
						</article>
					{:else}
						<div class="px-5 py-10 text-center text-sm text-[#696969]">
							Користувачів за цими параметрами не знайдено.
						</div>
					{/each}
				</div>
			</div>
		</section>
	</section>
</main>
