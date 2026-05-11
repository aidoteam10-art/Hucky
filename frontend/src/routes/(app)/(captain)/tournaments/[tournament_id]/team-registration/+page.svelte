<script>
	import Card from '/src/components/form/Card.svelte';
	import InputField from '/src/components/form/InputField.svelte';
	import Submit from '/src/components/form/Submit.svelte';

	export let data;
	export let form;

	let teamName = form?.values?.name || '';
	let organization = form?.values?.organization || '';
	let contact = form?.values?.contact || '';
	let memberEmails =
		form?.values?.member_emails?.length > 0 ? form.values.member_emails : [''];

	function addMemberEmail() {
		if (memberEmails.length < 5) {
			memberEmails = [...memberEmails, ''];
		}
	}

	function removeMemberEmail(index) {
		memberEmails = memberEmails.filter((_, i) => i !== index);
		if (memberEmails.length === 0) {
			memberEmails = [''];
		}
	}
</script>

<form method="POST" class="flex flex-col gap-6 max-w-4xl mx-auto w-full p-6 mt-10">
	<div class="flex flex-col items-start gap-1 text-center mb-2 px-2">
		<h1 class="text-[1.75rem] font-bold text-[#1F1F1F]">Зареєструйте вашу команду</h1>
		<h2 class="text-xs text-[#756157]">
			Реєстрація команди на <b>{data.tournament.title}</b>
		</h2>
	</div>

	{#if form?.message}
		<div class="rounded-lg border border-red-200 bg-red-50 px-4 py-3 text-sm text-red-700">
			{form.message}
		</div>
	{/if}

	<Card class="flex flex-col gap-5 w-full">
		<h2 class="text-base font-bold text-[#32221B]">Інформація про команду</h2>

		<InputField
			name="name"
			bind:value={teamName}
			required
			header="Назва команди *"
			placeholder="e.g. Code Warriors"
			class="w-full"
		/>

		<div class="flex flex-col sm:flex-row gap-8 w-full">
			<InputField
				name="organization"
				bind:value={organization}
				header="Організація"
				placeholder="Навчальний заклад / Компанія"
				class="flex-1"
			/>
			<InputField
				name="contact"
				bind:value={contact}
				header="Контакт (Telegram / Discord)"
				placeholder="username, нікнейм або посилання"
				class="flex-1"
			/>
		</div>
	</Card>

	<Card class="flex flex-col gap-5 w-full">
		<h2 class="text-base font-bold text-[#32221B]">Капітан команди</h2>

		<div class="grid gap-4 sm:grid-cols-2 text-sm text-[#32221B]">
			<div>
				<p class="font-semibold">Повне ім'я</p>
				<p class="text-[#756157]">{data.user.full_name}</p>
			</div>
			<div>
				<p class="font-semibold">Email</p>
				<p class="text-[#756157]">{data.user.email}</p>
			</div>
		</div>
	</Card>

	<Card class="flex flex-col gap-5 w-full">
		<div class="flex justify-between items-center px-1 gap-4">
			<h2 class="text-base font-bold text-[#32221B]">Email-інвайти учасників</h2>
			{#if memberEmails.length < 5}
				<button
					on:click={addMemberEmail}
					class="text-sm font-bold text-[#32221B] flex items-center gap-1 hover:underline cursor-pointer"
					type="button"
				>
					<span class="text-lg leading-none pb-0.5">+</span> Додати email
				</button>
			{/if}
		</div>

		{#each memberEmails as email, i}
			<div class="flex flex-col sm:flex-row gap-3 w-full relative">
				<InputField
					name="member_emails"
					bind:value={memberEmails[i]}
					required={i === 0}
					header="Email учасника *"
					type="email"
					placeholder="member@example.com"
					class="flex-1"
				/>

				{#if memberEmails.length > 1}
					<button
						on:click={() => removeMemberEmail(i)}
						type="button"
						class="sm:mt-8 text-red-500 hover:text-red-700 font-bold px-2 py-1 leading-none rounded-full cursor-pointer"
						title="Видалити"
					>
						✕
					</button>
				{/if}
			</div>
		{/each}

		<p class="text-[0.625rem] text-[#756157] mt-1 pl-1">
			Капітан береться з поточного акаунта. Мінімум 2 учасники разом з капітаном, максимум 6.
			({memberEmails.filter(Boolean).length + 1} / 6)
		</p>
	</Card>

	<Submit class="w-full mx-auto mt-4 h-12 flex justify-center items-center" title="Зареєструвати команду" />
</form>
