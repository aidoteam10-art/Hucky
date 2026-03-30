
<script>
	import '../../app.css';
	import Link from '/src/components/Link.svelte';
</script>



<nav class = "py-5 px-12 flex items-center justify-between">
	
		<a href = "/" class = "hidden items-center md:flex">
			<img src="/icons/logo_black.png" class=" w-a h-20 shrink-0" alt="Hucky Logo">
			<span class = "flex items-center  py-3 px-5">
				<span class = "font-bold text-[2.25rem]">H</span>
				<span class = "text-[#89AB00] font-bold text-[2.25rem]">u</span>
				<span class = "font-bold text-[2.25rem]">cky</span>
			</span>
		</a>

	<ol class = "hidden items-center gap-6 list-none md:flex">
		<li><Link href = "/" variant = "primary">Головна</Link></li>
		<li><Link href = "/tournaments" variant = "ghost">Турніри</Link></li>
	</ol>

	<ol class = "hidden items-center gap-6 list-none  md:flex">
		<li><Link href = "/login" variant = "ghost">Увійти</Link></li>
		<li><Link href = "/registration" variant = "primary">Реєстрація</Link></li>

	</ol>

</nav>

<slot></slot>
