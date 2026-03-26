
<script>
	import Link from '/src/components/Link.svelte';
</script>



<nav class = "py-2.5 px-12 flex items-center justify-between">
	
		<a href = "/" class = "hidden items-center lg:flex">
			<img src="/icons/logo_black.png" class=" w-10 h-a shrink-0" alt="logo">
			<span class = "flex items-center  py-3 px-5">
				<span class = "font-bold text-2xl">H</span>
				<span class = "text-[#89AB00] font-bold text-2xl">u</span>
				<span class = "font-bold text-2xl">cky</span>
			</span>
		</a>

	<ol class = "hidden items-center gap-6 list-none lg:flex">
		<li><Link href = "/" variant = "primary">Головна</Link></li>
		<li><Link href = "/tournaments" variant = "ghost">Турніри</Link></li>
	</ol>

	<ol class = "hidden items-center gap-6 list-none  lg:flex">
		<li><Link href = "/login" variant = "ghost">Увійти</Link></li>
		<li><Link href = "/registration" variant = "primary">Реєстрація</Link></li>

	</ol>

</nav>

<slot></slot>
