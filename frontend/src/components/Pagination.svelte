<script>
	export let currentPage = 1;
	export let totalPages = 3;

	$: pages = Array.from({ length: totalPages }, (_, index) => index + 1);

	function goToPage(page) {
		if (page >= 1 && page <= totalPages) {
			currentPage = page;
		}
	}
</script>

<div class="flex items-center justify-center gap-2 sm:gap-4">
	<button
		class="flex h-10 w-10 items-center justify-center rounded-full transition hover:bg-[#e7e7e7] sm:h-11 sm:w-11 {currentPage ===
		1
			? 'cursor-not-allowed opacity-50'
			: ''}"
		on:click={() => goToPage(currentPage - 1)}
		disabled={currentPage === 1}
		aria-label="Previous page"
	>
		<svg class="h-6 w-6 fill-none stroke-[#111111] stroke-2" viewBox="0 0 24 24">
			<path stroke-linecap="round" stroke-linejoin="round" d="M15 18L9 12L15 6" />
		</svg>
	</button>

	{#each pages as page (page)}
		<button
			class="flex h-10 w-10 items-center justify-center rounded-full text-lg font-bold text-[#202020] transition sm:h-11 sm:w-11 sm:text-xl {currentPage ===
			page
				? 'bg-[#CCFF00]'
				: 'bg-[#F4F4F5] hover:bg-[#dddddd]'}"
			on:click={() => goToPage(page)}
		>
			{page}
		</button>
	{/each}

	<button
		class="flex h-10 w-10 items-center justify-center rounded-full transition hover:bg-[#e7e7e7] sm:h-11 sm:w-11 {currentPage ===
		totalPages
			? 'cursor-not-allowed opacity-50'
			: ''}"
		on:click={() => goToPage(currentPage + 1)}
		disabled={currentPage === totalPages}
		aria-label="Next page"
	>
		<svg class="h-6 w-6 fill-none stroke-[#111111] stroke-2" viewBox="0 0 24 24">
			<path stroke-linecap="round" stroke-linejoin="round" d="M9 18L15 12L9 6" />
		</svg>
	</button>
</div>
