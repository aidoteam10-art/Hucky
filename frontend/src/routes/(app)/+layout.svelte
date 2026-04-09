
<script>
	import '../../app.css';
	import Link from '/src/components/Link.svelte';
	import { page } from '$app/stores';

	$: isHomePage = $page.url.pathname === '/';
</script>



<nav class="py-5 px-12 flex items-center justify-between transition-colors duration-300 {isHomePage ? 'bg-white' : 'bg-[#191F00]'}">
	
		<a href = "/" class = "hidden items-center md:flex">
			<img src={isHomePage ? "/icons/logo_black.png" : "/icons/logo_white.png"} class="w-auto h-20 shrink-0" alt="Hucky Logo">
			<span class="flex items-center py-3 px-5 {isHomePage ? 'text-[#191F00]' : 'text-white'}">
            <span class="font-bold text-[2.25rem]">H</span>
            <span class="text-[#89AB00] font-bold text-[2.25rem]">u</span>
            <span class="font-bold text-[2.25rem]">cky</span>
        </span>
		</a>

	<ol class="hidden items-center gap-6 list-none md:flex">
		<li>
			<Link href="/" variant={isHomePage ? "primary" : "second"}>
				Головна
			</Link>
		</li>
		<li>
			<Link href="/tournaments" variant={isHomePage ? "ghost" : "second"}>
				Турніри
			</Link>
		</li>
	</ol>

	<ol class="hidden items-center gap-6 list-none md:flex">
		<li>
			<Link href="/login" variant={isHomePage ? "ghost" : "second"}>
				Увійти
			</Link>
		</li>
		<li>
			<Link href="/registration" variant={isHomePage ? "primary" : "green"}>
				Реєстрація
			</Link>
		</li>
	</ol>
</nav>

<slot></slot>
