<script>
	import Card from '/src/components/form/Card.svelte';
	import DateField from '/src/components/form/DateField.svelte';
	import InputField from '/src/components/form/InputField.svelte';
	import Submit from '/src/components/form/Submit.svelte';
	import TextArea from '/src/components/form/TextArea.svelte';

	export let data;
	export let form;

	let title = form?.values?.title || '';
	let description = form?.values?.description || '';
	let rules = form?.values?.rules || '';
	let regStart = form?.values?.registration_starts_at || '';
	let regEnd = form?.values?.registration_ends_at || '';
	let tourStart = form?.values?.starts_at || '';
	let maxTeams = form?.values?.max_teams || '';
	let roundTitle = form?.values?.round_title || '';
	let roundDesc = form?.values?.task_description || '';
	let techStack = form?.values?.technology_requirements || '';
	let roundStart = form?.values?.round_starts_at || '';
	let roundDeadline = form?.values?.deadline_at || '';
	let requirements = form?.values?.must_have ? [form.values.must_have] : [''];

	function addRequirement() {
		requirements = [...requirements, ''];
	}

	function removeRequirement(index) {
		requirements = requirements.filter((_, itemIndex) => itemIndex !== index);
		if (requirements.length === 0) requirements = [''];
	}

	function saveDraft() {
		const draft = {
			title,
			description,
			rules,
			regStart,
			regEnd,
			tourStart,
			maxTeams,
			roundTitle,
			roundDesc,
			techStack,
			roundStart,
			roundDeadline,
			requirements
		};
		localStorage.setItem('tournamentDraft', JSON.stringify(draft));
		alert('Чернетка збережена локально');
	}
</script>

<svelte:head>
	<title>Hucky - Створити турнір</title>
</svelte:head>

<form method="POST" class="mx-auto mt-10 flex w-full max-w-4xl flex-col gap-6 p-6">
	<div class="mb-2 flex items-start justify-between gap-3 px-2">
		<div class="flex flex-col gap-1">
			<h1 class="text-[1.75rem] font-bold text-[#1F1F1F]">Створити турнір</h1>
			<h2 class="text-sm font-bold text-[#1F1F1F]">Турнір, перший раунд і задача</h2>
		</div>
		<button
			on:click={saveDraft}
			class="flex items-center gap-2 rounded-lg border border-[#756157] bg-transparent px-4 py-2 text-sm font-semibold text-[#1F1F1F] transition-colors hover:bg-black/5"
			type="button"
		>
			<span class="flex h-5 w-5 items-center justify-center rounded-full bg-[#CCFF00] pt-0.5 text-lg leading-none font-bold text-black">
				+
			</span>
			Зберегти чернетку
		</button>
	</div>

	{#if !data.isAuthenticated}
		<div class="rounded-xl border border-amber-200 bg-amber-50 px-5 py-4 text-sm font-semibold text-amber-800">
			Потрібно увійти в акаунт, щоб створити турнір.
		</div>
	{/if}

	{#if form?.message}
		<div class="rounded-xl border border-red-200 bg-red-50 px-5 py-4 text-sm font-semibold text-red-700">
			{form.message}
		</div>
	{/if}

	<Card class="flex w-full flex-col gap-5">
		<h2 class="text-base font-bold text-[#32221B]">Деталі турніру</h2>
		<InputField
			bind:value={title}
			name="title"
			required
			header="Назва*"
			placeholder="Spring Hackathon"
			class="w-full"
		/>
		<TextArea
			bind:value={description}
			name="description"
			required
			header="Опис*"
			placeholder="Коротко опишіть формат, ціль і аудиторію турніру"
			rows={4}
			class="w-full"
		/>
		<TextArea
			bind:value={rules}
			name="rules"
			required
			header="Правила*"
			placeholder="Опишіть правила участі, дедлайни, вимоги та обмеження"
			rows={4}
			class="w-full"
		/>
	</Card>

	<Card class="flex w-full flex-col gap-5">
		<h2 class="text-base font-bold text-[#32221B]">Реєстрація і розклад</h2>
		<div class="flex w-full flex-col gap-8 sm:flex-row">
			<DateField
				bind:value={regStart}
				name="registration_starts_at"
				required
				header="Початок реєстрації*"
				class="flex-1"
			/>
			<DateField
				bind:value={regEnd}
				name="registration_ends_at"
				required
				header="Завершення реєстрації*"
				class="flex-1"
			/>
		</div>
		<div class="flex w-full flex-col gap-8 sm:flex-row">
			<DateField
				bind:value={tourStart}
				name="starts_at"
				required
				header="Початок турніру*"
				class="flex-1"
			/>
			<InputField
				bind:value={maxTeams}
				name="max_teams"
				type="number"
				min="1"
				header="Максимум команд"
				placeholder="30"
				class="flex-1"
			/>
		</div>
	</Card>

	<Card class="flex w-full flex-col gap-5">
		<h2 class="text-base font-bold text-[#32221B]">Перший раунд</h2>
		<InputField
			bind:value={roundTitle}
			name="round_title"
			required
			header="Назва раунду*"
			placeholder="Round 1: MVP Development"
			class="w-full"
		/>
		<TextArea
			bind:value={roundDesc}
			name="task_description"
			required
			header="Опис завдання*"
			placeholder="Що команди мають зробити в межах раунду"
			rows={4}
			class="w-full"
		/>
		<TextArea
			bind:value={techStack}
			name="technology_requirements"
			header="Технологічні вимоги"
			placeholder="Rust, SvelteKit, PostgreSQL"
			rows={2}
			class="w-full"
		/>

		<div class="mt-2 flex w-full flex-col gap-3">
			<div class="flex items-center justify-between px-1">
				<h3 class="text-sm font-bold text-[#32221B]">Must-have вимоги</h3>
				<button
					on:click={addRequirement}
					class="flex cursor-pointer items-center gap-1 text-sm font-bold text-[#32221B] hover:underline"
					type="button"
				>
					<span class="text-lg leading-none">+</span>
					Додати
				</button>
			</div>
			{#each requirements as requirement, index (index)}
				<div class="flex items-end gap-3" data-filled={Boolean(requirement)}>
					<InputField
						bind:value={requirements[index]}
						name="must_have"
						placeholder="Вимога {index + 1}"
						header=""
						class="w-full"
					/>
					<button
						type="button"
						on:click={() => removeRequirement(index)}
						class="mb-2 rounded-lg border border-[#B4B4B4] px-3 py-2 text-sm font-bold text-[#191F00] hover:bg-[#F4F4F5]"
						aria-label="Видалити вимогу"
					>
						×
					</button>
				</div>
			{/each}
		</div>

		<div class="mt-2 flex w-full flex-col gap-8 sm:flex-row">
			<DateField
				bind:value={roundStart}
				name="round_starts_at"
				required
				header="Початок раунду*"
				class="flex-1"
			/>
			<DateField
				bind:value={roundDeadline}
				name="deadline_at"
				required
				header="Дедлайн*"
				class="flex-1"
			/>
		</div>
	</Card>

	<Submit class="mt-4 flex h-12 w-full items-center justify-center" title="Створити турнір" />
</form>
