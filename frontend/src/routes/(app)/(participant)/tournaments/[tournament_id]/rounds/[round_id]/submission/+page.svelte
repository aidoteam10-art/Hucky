<script>
	import Card from '/src/components/form/Card.svelte';
	import InputField from '/src/components/form/InputField.svelte';
	import TextArea from '/src/components/form/TextArea.svelte';
	import Submit from '/src/components/form/Submit.svelte';

	export let data;
	export let form;

	$: values = {
		github_url: form?.values?.github_url ?? data.submission?.github_url ?? '',
		video_demo_url: form?.values?.video_demo_url ?? data.submission?.video_demo_url ?? '',
		live_demo_url: form?.values?.live_demo_url ?? data.submission?.live_demo_url ?? '',
		description: form?.values?.description ?? data.submission?.description ?? ''
	};

	$: isLocked = data.submission?.status === 'locked';
	$: timeLeft = formatTimeLeft(data.round?.time_left_seconds);

	function formatTimeLeft(seconds) {
		const value = Number(seconds || 0);
		if (value <= 0) return 'Дедлайн настав';
		const days = Math.floor(value / 86400);
		const hours = Math.floor((value % 86400) / 3600);
		const minutes = Math.floor((value % 3600) / 60);
		if (days > 0) return `${days} дн ${hours} год ${minutes} хв`;
		return `${hours} год ${minutes} хв`;
	}
</script>

<div class="mx-auto mt-10 w-full max-w-3xl p-6">
	<div class="mb-5 flex flex-col items-center gap-1 sm:flex-row sm:justify-between">
		<div>
			<h1 class="font-bold text-[1.75rem]">Здати роботу</h1>
			<p class="mt-1 text-sm text-[#696969]">{data.round.title}</p>
		</div>
		<h3 class="text-[0.8rem]">
			Часу залишилося: <b class="ml-5">{timeLeft}</b>
		</h3>
	</div>

	{#if form?.message}
		<div class="mb-4 rounded-xl border border-red-200 bg-red-50 px-5 py-4 text-sm font-semibold text-red-700">
			{form.message}
		</div>
	{/if}

	{#if isLocked}
		<div class="mb-4 rounded-xl border border-[#B4B4B4] bg-[#F4F4F5] px-5 py-4 text-sm font-semibold text-[#696969]">
			Роботу заблоковано для перевірки. Редагування недоступне.
		</div>
	{/if}

	<form method="POST" class="w-full">
		<Card class="mb-4 flex w-full flex-col gap-5">
			<InputField
				name="github_url"
				bind:value={values.github_url}
				disabled={isLocked}
				src_icon="/icons/github.svg"
				header="GitHub Repository *"
				placeholder="https://github.com/..."
			/>
			<InputField
				name="video_demo_url"
				bind:value={values.video_demo_url}
				disabled={isLocked}
				src_icon="/icons/camera.svg"
				header="Video Demo *"
				placeholder="https://youtube.com/..."
			/>
			<InputField
				name="live_demo_url"
				bind:value={values.live_demo_url}
				disabled={isLocked}
				src_icon="/icons/globe.svg"
				header="Live Demo URL"
				placeholder="https://..."
			/>
			<TextArea
				name="description"
				bind:value={values.description}
				disabled={isLocked}
				src_icon="/icons/docs.svg"
				header="Description"
				placeholder="What you've built, how to run it..."
			/>
		</Card>

		<Submit title={data.submission ? 'Оновити роботу' : 'Здати роботу'} class="w-full" disabled={isLocked} />
	</form>
</div>
