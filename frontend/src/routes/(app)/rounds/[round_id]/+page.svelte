<script>
	import { resolve } from '$app/paths';
	import StateTag from '../../../../components/StateTag.svelte';

	export let data;

	$: round = data.round;

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

	function formatTimeLeft(seconds) {
		const value = Number(seconds || 0);
		const days = Math.floor(value / 86400);
		const hours = Math.floor((value % 86400) / 3600);
		const minutes = Math.floor((value % 3600) / 60);

		if (value <= 0) return 'Дедлайн настав';
		if (days > 0) return `${days} дн ${hours} год ${minutes} хв`;
		return `${hours} год ${minutes} хв`;
	}
</script>

<svelte:head>
	<title>Hucky - {round.title}</title>
</svelte:head>

<div class="mx-auto mt-10 max-w-7xl space-y-6 px-6 pb-16 font-sans md:px-10 xl:px-0">
	<div class="rounded-3xl bg-[#191F00] p-8 text-white md:p-12 lg:p-15">
		<div class="flex flex-col gap-8 md:flex-row md:items-center md:justify-between">
			<div>
				<div class="mb-5 flex flex-wrap items-center gap-3">
					<h1 class="text-xl font-bold md:text-[1.7rem]">{round.title}</h1>
					<StateTag variant={round.status} />
				</div>
				<a
					href={resolve('/tournaments/[tournament_id]', {
						tournament_id: String(round.tournament_id)
					})}
					class="inline-flex rounded-lg bg-white px-5 py-2 text-sm font-bold text-[#191F00] hover:bg-[#CBCBCB]"
				>
					До турніру
				</a>
			</div>

			<div class="md:text-right">
				<p class="text-[1rem] opacity-80">Часу залишилося</p>
				<p class="mt-3 text-3xl font-semibold tracking-tight md:text-[2.2rem]">
					{formatTimeLeft(round.time_left_seconds)}
				</p>
			</div>
		</div>
	</div>

	<div class="rounded-2xl border border-[#B4B4B4] bg-white px-6 py-7 lg:px-8 lg:py-9">
		<div class="mb-3 flex items-center gap-2 font-medium text-gray-900">
			<img src="/icons/task_desc.svg" alt="" class="h-5 w-5" />
			<span class="font-semibold text-[#191F00]">Опис завдання</span>
		</div>
		<p class="text-[1rem] leading-relaxed whitespace-pre-line text-[#191F00]">{round.task_description}</p>
	</div>

	<div class="rounded-2xl border border-[#B4B4B4] bg-white px-6 py-7 lg:px-8 lg:py-9">
		<div class="mb-3 flex items-center gap-2 font-medium text-gray-900">
			<span class="font-semibold text-[#191F00]">Необхідні технології</span>
		</div>
		<p class="text-[1rem] leading-relaxed text-[#191F00]">
			{round.technology_requirements || 'Окремі технологічні вимоги не вказані.'}
		</p>
	</div>

	<div class="rounded-2xl border border-[#B4B4B4] bg-white px-6 py-7 lg:px-8 lg:py-9">
		<div class="mb-6 flex items-center gap-2 font-medium text-gray-900 lg:mb-8">
			<img src="/icons/required.svg" alt="" class="h-5 w-5" />
			<span class="font-semibold text-[#191F00]">Обов'язкові вимоги</span>
		</div>

		<div class="space-y-3">
			{#each round.must_have as requirement, index (requirement.id)}
				<div class="rounded-xl bg-[#CCFF00] p-4 text-[1rem] text-[#191F00] lg:p-5">
					<div class="flex items-center gap-4">
						<span
							class="flex h-8 w-8 shrink-0 items-center justify-center rounded-lg border border-[#191F00]/20 text-[1rem]"
						>
							{index + 1}
						</span>
						<p class="text-[16px]">{requirement.text}</p>
					</div>
				</div>
			{:else}
				<p class="rounded-xl bg-[#F4F4F5] px-5 py-4 text-sm font-semibold text-gray-600">
					Must-have вимоги не додані.
				</p>
			{/each}
		</div>
	</div>

	<div class="rounded-2xl border border-[#B4B4B4] bg-white px-6 py-7 lg:px-8 lg:py-9">
		<div class="mb-6 flex items-center gap-3 font-medium text-gray-900 lg:mb-8">
			<img src="/icons/schedule.svg" alt="" class="h-5 w-5" />
			<span class="font-semibold text-[#191F00]">Розклад</span>
		</div>
		<div class="flex w-full flex-col gap-8 sm:flex-row">
			<div class="flex-1 text-left">
				<h3 class="mb-2 text-[1.1rem] font-semibold text-[#696969] lg:text-[1.2rem]">Початок</h3>
				<p class="text-[1.1rem] font-semibold tracking-[0.05em] text-[#191F00] lg:text-[1.2rem]">
					{formatDate(round.starts_at)}
				</p>
			</div>

			<div class="flex-1 text-left">
				<h3 class="mb-2 text-[1.1rem] font-semibold text-[#696969] lg:text-[1.2rem]">Дедлайн</h3>
				<p class="text-[1.1rem] font-semibold tracking-[0.05em] text-[#191F00] lg:text-[1.2rem]">
					{formatDate(round.deadline_at)}
				</p>
			</div>
		</div>
	</div>
</div>
