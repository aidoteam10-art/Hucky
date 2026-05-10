<script>
	const roles = {
		participant: 'Учасник',
		jury: 'Журі',
		admin: 'Адмін'
	};

	let search = '';
	let roleFilter = 'all';

	let topAdmin = {
		name: 'Volodymyr Hrynchyshyn',
		email: 'annie@example.com',
		status: 'Найвищий адмін'
	};

	let users = [
		{
			id: 1,
			name: 'Alex Kovalenko',
			email: 'example@example.com',
			school: 'KPI University',
			role: 'participant'
		},
		{
			id: 2,
			name: 'Maria Shevchenko',
			email: 'maria@example.com',
			school: 'Lviv Polytechnic',
			role: 'participant'
		},
		{
			id: 3,
			name: 'Danylo Melnyk',
			email: 'danylo@example.com',
			school: 'KNU',
			role: 'jury'
		},
		{
			id: 4,
			name: 'Oksana Bondar',
			email: 'oksana@example.com',
			school: 'Kharkiv IT Cluster',
			role: 'admin'
		},
		{
			id: 5,
			name: 'Ihor Savchuk',
			email: 'ihor@example.com',
			school: 'KPI University',
			role: 'jury'
		}
	];

	$: counts = {
		all: users.length,
		participant: users.filter((user) => user.role === 'participant').length,
		jury: users.filter((user) => user.role === 'jury').length,
		admin: users.filter((user) => user.role === 'admin').length
	};

	$: normalizedSearch = search.trim().toLowerCase();
	$: filteredUsers = users.filter((user) => {
		const matchesRole = roleFilter === 'all' || user.role === roleFilter;
		const matchesSearch =
			!normalizedSearch ||
			user.name.toLowerCase().includes(normalizedSearch) ||
			user.email.toLowerCase().includes(normalizedSearch);

		return matchesRole && matchesSearch;
	});

	const changeRole = (userId, role) => {
		users = users.map((user) => (user.id === userId ? { ...user, role } : user));
	};
</script>

<main class="min-h-[calc(100vh-5rem)] bg-white px-4 py-10 font-sans text-[#191F00] sm:px-8 lg:px-28 lg:py-24">
	<section class="mx-auto max-w-6xl">
		<div class="mb-8">
			<div class="mb-4 flex items-center gap-4">
				<div class="flex h-10 w-10 shrink-0 items-center justify-center text-[#FF4545]" aria-hidden="true">
					<svg viewBox="0 0 24 24" class="h-8 w-8" fill="none" stroke="currentColor" stroke-width="2">
						<path d="m4 8 4 3 4-7 4 7 4-3-2 10H6L4 8Z" />
						<path d="M6 21h12" />
					</svg>
				</div>
				<h1 class="text-3xl font-bold leading-tight sm:text-4xl">Найвищий адміністратор</h1>
			</div>
			<p class="max-w-3xl text-sm leading-6 text-[#696969] sm:text-base">
				Керування ролями користувачів. Лише ви можете призначати журі та адміністраторів.
			</p>
		</div>

		<section class="mb-6 rounded-xl border border-[#B4B4B4] bg-white p-5 sm:p-6">
			<div class="flex flex-col gap-4 sm:flex-row sm:items-center">
				<div class="flex h-16 w-16 shrink-0 items-center justify-center rounded-full bg-[#FFECEC] text-[#FF4545]">
					<img src="/icons/shield.svg" alt="" class="h-8 w-8" />
				</div>
				<div class="min-w-0">
					<h2 class="break-words text-xl font-bold">{topAdmin.name}</h2>
					<div class="mt-2 flex min-w-0 items-center gap-2 text-sm text-[#696969]">
						<img src="/icons/mail_dark.svg" alt="" class="h-4 w-4 shrink-0 opacity-70" />
						<span class="break-all">{topAdmin.email}</span>
					</div>
					<span class="mt-3 inline-flex rounded-full border border-[#FF4545] px-3 py-1 text-xs font-medium text-[#FF4545]">
						{topAdmin.status}
					</span>
				</div>
			</div>
		</section>

		<section class="rounded-xl border border-[#B4B4B4] bg-white p-4 sm:p-5">
			<div class="mb-5 flex flex-col gap-4 lg:flex-row lg:items-center lg:justify-between">
				<h2 class="text-base font-bold">Усі користувачі ({counts.all})</h2>

				<div class="grid gap-3 sm:grid-cols-[minmax(0,1fr)_12rem] lg:w-[34rem]">
					<label class="relative block">
						<span class="sr-only">Пошук за іменем або поштою</span>
						<img
							src="/icons/search_icon.svg"
							alt=""
							class="pointer-events-none absolute left-3 top-1/2 h-4 w-4 -translate-y-1/2 opacity-70"
						/>
						<input
							bind:value={search}
							type="search"
							placeholder="Пошук за іменем або поштою"
							class="h-11 w-full rounded-lg border border-[#B4B4B4] bg-white pl-10 pr-3 text-sm outline-none transition focus:border-[#191F00] focus:ring-1 focus:ring-[#191F00]"
						/>
					</label>

					<label>
						<span class="sr-only">Фільтр ролей</span>
						<select
							bind:value={roleFilter}
							class="h-11 w-full rounded-lg border border-[#B4B4B4] bg-white px-3 text-sm outline-none transition focus:border-[#191F00] focus:ring-1 focus:ring-[#191F00]"
						>
							<option value="all">Усі ({counts.all})</option>
							<option value="participant">Учасник ({counts.participant})</option>
							<option value="jury">Журі ({counts.jury})</option>
							<option value="admin">Адмін ({counts.admin})</option>
						</select>
					</label>
				</div>
			</div>

			<div class="overflow-hidden rounded-lg border border-[#B4B4B4]">
				<div class="hidden grid-cols-[minmax(18rem,1fr)_10rem_13rem] bg-[#CCFF00] px-5 py-3 text-xs font-bold uppercase md:grid">
					<span>Користувач</span>
					<span>Поточна роль</span>
					<span>Змінити роль</span>
				</div>

				<div class="divide-y divide-[#D8D8D8]">
					{#each filteredUsers as user}
						<article class="grid gap-4 bg-white p-4 md:grid-cols-[minmax(18rem,1fr)_10rem_13rem] md:items-center md:px-5">
							<div class="flex min-w-0 items-start gap-4">
								<img src="/icons/avatar.svg" alt="" class="h-12 w-12 shrink-0" />
								<div class="min-w-0">
									<h3 class="break-words text-sm font-bold">{user.name}</h3>
									<div class="mt-2 flex flex-col gap-2 text-xs text-[#696969] sm:flex-row sm:flex-wrap sm:items-center sm:gap-x-4">
										<span class="flex min-w-0 items-center gap-2">
											<img src="/icons/mail_dark.svg" alt="" class="h-3.5 w-3.5 shrink-0 opacity-70" />
											<span class="break-all">{user.email}</span>
										</span>
										<span class="flex min-w-0 items-center gap-2">
											<img src="/icons/school.svg" alt="" class="h-3.5 w-3.5 shrink-0 opacity-70" />
											<span class="break-words">{user.school}</span>
										</span>
									</div>
								</div>
							</div>

							<div class="flex items-center justify-between gap-3 md:block">
								<span class="text-xs font-semibold uppercase text-[#696969] md:hidden">Поточна роль</span>
								<span
									class="inline-flex min-w-20 justify-center rounded-full bg-[#191F00] px-3 py-1 text-xs font-medium
									{user.role === 'participant' ? 'text-[#CCFF00]' : ''}
									{user.role === 'jury' ? 'text-[#F97316] ring-1 ring-[#F97316]' : ''}
									{user.role === 'admin' ? 'text-gray-300 ring-1 ring-gray-500' : ''}"
								>
									{roles[user.role]}
								</span>
							</div>

							<label class="grid gap-2 sm:grid-cols-[8rem_minmax(0,1fr)] sm:items-center md:block">
								<span class="text-xs font-semibold uppercase text-[#696969] md:hidden">Змінити роль</span>
								<select
									value={user.role}
									on:change={(event) => changeRole(user.id, event.currentTarget.value)}
									class="h-10 w-full rounded-lg border border-[#B4B4B4] bg-white px-3 text-sm outline-none transition focus:border-[#191F00] focus:ring-1 focus:ring-[#191F00]"
								>
									<option value="participant">Учасник</option>
									<option value="jury">Журі</option>
									<option value="admin">Адмін</option>
								</select>
							</label>
						</article>
					{:else}
						<div class="px-5 py-10 text-center text-sm text-[#696969]">
							Користувачів за цими параметрами не знайдено.
						</div>
					{/each}
				</div>
			</div>

			<p class="mt-5 max-w-4xl text-xs leading-5 text-[#696969] sm:text-sm">
				Користувачі реєструються з роллю «Користувач». Після реєстрації на турнір вони стають «Учасниками».
				Ролі «Журі» та «Адміністратор» може призначити лише Найвищий адміністратор.
			</p>
		</section>
	</section>
</main>
