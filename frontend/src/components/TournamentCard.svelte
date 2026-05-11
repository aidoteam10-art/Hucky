<script>
	import StateTag from './StateTag.svelte';

	export let variant = 'green';
	export let current_state = 'registration';
	export let title = 'Hackathon Ukraine 2026';
	export let description = '';
	export let start_date = '';
	export let rounds = 0;
	export let max_teams = null;
	export let registered_teams = 0;
	export let id = '';

	function formatIsoDate(value) {
		if (!value) return 'Дата не вказана';
		return new Intl.DateTimeFormat('uk-UA', {
			day: '2-digit',
			month: 'short',
			year: 'numeric'
		}).format(new Date(value));
	}

	function formatRounds(value) {
		const count = Number(value || 0);
		if (count === 1) return '1 раунд';
		if (count > 1 && count < 5) return `${count} раунди`;
		return `${count} раундів`;
	}

	$: formattedRounds = formatRounds(rounds);
	$: formattedStartDate = formatIsoDate(start_date);
	$: teamsLimit = max_teams ?? '∞';
</script>

<div class="@container h-full w-full">
	<article
		class="flex aspect-video h-full flex-col rounded-[4.5cqw] p-[6cqw] transition-transform duration-200 hover:scale-[1.02] xl:rounded-[4cqw] xl:p-[5cqw] {variant ===
		'green'
			? 'bg-linear-to-r from-[#BCEB01] to-[#EEFF00]'
			: 'border border-[#c5c5c5] bg-[linear-gradient(180deg,#f4f4f4_0%,#ececec_100%)]'}"
	>
		<div class="mb-[2cqw] flex items-center justify-between text-[#191F00]">
			<StateTag variant={current_state} class="text-[3cqw] xl:text-[2.5cqw]" />
			<span class="select-none text-[3.5cqw] font-medium xl:text-[3cqw]">{formattedRounds}</span>
		</div>

		<h3 class="mt-[2cqw] mb-[3cqw] line-clamp-1 select-none text-[6cqw] font-black xl:text-[5cqw]">
			{title}
		</h3>
		<p class="mb-[3cqw] line-clamp-2 select-none text-[4cqw] leading-snug xl:text-[3.5cqw]">
			{description}
		</p>

		<div class="mt-auto">
			<div class="mb-[4cqw] flex items-center gap-[4cqw] text-[#191F00] xl:mb-[3cqw] xl:gap-[3cqw]">
				<div class="flex items-center gap-[1.5cqw]">
					<img src="/icons/calendar.svg" alt="" class="w-[4cqw] select-none xl:w-[3.5cqw]" />
					<span class="select-none text-[3.5cqw] font-medium xl:text-[3cqw]">
						{formattedStartDate}
					</span>
				</div>

				<div class="flex items-center gap-[1.5cqw]">
					<img src="/icons/team_tournament.svg" alt="" class="w-[3.5cqw] select-none xl:w-[3cqw]" />
					<span class="select-none text-[3.5cqw] font-medium xl:text-[3cqw]">
						{registered_teams}/{teamsLimit} команд
					</span>
				</div>
			</div>
		</div>

		<a
			href={`/tournaments/${id}`}
			class="mt-auto flex w-fit select-none items-center gap-[2cqw] text-[4cqw] font-bold text-[#191F00] xl:text-[3.5cqw]"
		>
			Детальніше
			<img src="/arrow_right.svg" class="w-[2cqw] xl:w-[1.5cqw]" alt="" />
		</a>
	</article>
</div>
