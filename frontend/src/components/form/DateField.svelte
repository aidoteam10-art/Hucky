<script>
	export let placeholder = 'yyyy-mm-dd --:--';
	export let header = 'header';
	export let variant = 'big-header';
	export let src_icon = '';
	export let name = undefined;
	export let required = false;
	export let value = '';
	export let error = '';
	export let success = false;
</script>

<div class="flex flex-col gap-2 {$$props.class ?? ''}">
	<div class="flex items-center gap-1.5">
		{#if src_icon}
			<img src={src_icon} alt="" class="h-4 w-4 object-contain" />
		{/if}
		{#if header}
			<h3
				class="font-medium {error
					? 'text-red-600'
					: success
						? 'text-green-600'
						: 'text-[#32221B]'} {variant === 'big-header'
					? 'text-sm font-semibold'
					: 'text-[0.625rem]'} transition-colors"
			>
				{header}
			</h3>
		{/if}
	</div>
	<input
		type="datetime-local"
		{name}
		{required}
		{placeholder}
		bind:value
		class="w-full rounded-lg border {error
			? 'border-red-500 bg-red-50 shake'
			: success
				? 'border-green-500 bg-green-50'
				: 'border-[#B4B4B4] bg-[#F9FFE5]'} p-2 pl-2 text-[0.75rem] text-[#756157] outline-none transition-all focus:border-[#756157]"
	/>
	{#if error}
		<span class="mt-[-0.25rem] text-[0.625rem] text-red-500 transition-all">{error}</span>
	{/if}
</div>
