<script>
  export let currentPage = 1;
  export let totalPages = 3;

  function goToPage(page) {
    if (page >= 1 && page <= totalPages) {
      currentPage = page;
    }
  }
</script>

<div class="mt-[52px] flex items-center justify-center gap-4">
  <button
    class="flex h-[44px] w-[44px] items-center justify-center rounded-full transition hover:bg-[#e7e7e7] {currentPage === 1 ? 'opacity-50 cursor-not-allowed' : ''}"
    on:click={() => goToPage(currentPage - 1)}
    disabled={currentPage === 1}
  >
    <svg
      class="h-[26px] w-[26px] fill-none stroke-[2.4] stroke-[#111111]"
      viewBox="0 0 24 24"
    >
      <path d="M15 18L9 12L15 6" />
    </svg>
  </button>

  {#each Array(totalPages) as _, i}
    <button
            class="flex h-[42px] w-[42px] items-center justify-center rounded-full text-[20px] font-[700] text-[#202020] transition {
      currentPage === i + 1
        ? 'bg-[#CCFF00]'
        : 'bg-[#F4F4F5] hover:bg-[#dddddd]'
    }"
            on:click={() => goToPage(i + 1)}
    >
      {i + 1}
    </button>
  {/each}

  <button
    class="flex h-[44px] w-[44px] items-center justify-center rounded-full transition hover:bg-[#e7e7e7] {currentPage === totalPages ? 'opacity-50 cursor-not-allowed' : ''}"
    on:click={() => goToPage(currentPage + 1)}
    disabled={currentPage === totalPages}
  >
    <svg
      class="h-[26px] w-[26px] fill-none stroke-[2.4] stroke-[#111111]"
      viewBox="0 0 24 24"
    >
      <path d="M9 18L15 12L9 6" />
    </svg>
  </button>
</div>
