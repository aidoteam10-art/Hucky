<script>
	import '../../app.css';
	import Link from '/src/components/Link.svelte';
	import { page } from '$app/stores';
	import { slide } from 'svelte/transition';

	$: isHomePage = $page.url.pathname === '/';

	let isMenuOpen = false;
	const toggleMenu = () => isMenuOpen = !isMenuOpen;
</script>

<nav class="relative py-3 lg:py-5 px-6 lg:px-12 flex items-center transition-colors duration-300 {isHomePage ? 'bg-white' : 'bg-[#191F00]'} z-50">

	<a href="/" class="flex items-center shrink-0">
		<img src={isHomePage ? "/icons/logo_black.png" : "/icons/logo_white.png"} class="w-auto h-12 lg:h-20 shrink-0" alt="Hucky Logo">
		<span class="flex items-center py-2 px-3 lg:py-3 lg:px-5 {isHomePage ? 'text-[#191F00]' : 'text-white'}">
            <span class="font-bold text-[1.5rem] lg:text-[2.25rem]">H</span>
            <span class="text-[#89AB00] font-bold text-[1.5rem] lg:text-[2.25rem]">u</span>
            <span class="font-bold text-[1.5rem] lg:text-[2.25rem]">cky</span>
        </span>
	</a>

	<div class="hidden md:flex flex-1 items-center">
		<ol class="flex flex-1 justify-center items-center gap-4 lg:gap-10 list-none">
			<li>
				<Link {isHomePage} href="/" variant={isHomePage ? "primary" : "second"}>Головна</Link>
			</li>
			<li>
				<Link {isHomePage} href="/tournaments" variant={isHomePage ? "ghost" : "second"}>Турніри</Link>
			</li>
		</ol>

		<ol class="flex items-center gap-2 lg:gap-6 list-none shrink-0">
			<li>
				<Link {isHomePage} href="/login" variant={isHomePage ? "ghost" : "second"}>Увійти</Link>
			</li>
			<li>
				<Link {isHomePage} href="/registration" variant={isHomePage ? "primary" : "green"}>Реєстрація</Link>
			</li>
		</ol>
	</div>

	<button
			on:click={toggleMenu}
			class="ml-auto flex flex-col gap-1.5 md:hidden p-2 focus:outline-none"
			aria-label="Toggle Menu"
	>
		<span class="w-6 h-0.5 {isHomePage ? 'bg-[#191F00]' : 'bg-white'} transition-all {isMenuOpen ? 'rotate-45 translate-y-2' : ''}"></span>
		<span class="w-6 h-0.5 {isHomePage ? 'bg-[#191F00]' : 'bg-white'} {isMenuOpen ? 'opacity-0' : ''}"></span>
		<span class="w-6 h-0.5 {isHomePage ? 'bg-[#191F00]' : 'bg-white'} transition-all {isMenuOpen ? '-rotate-45 -translate-y-2' : ''}"></span>
	</button>
</nav>

{#if isMenuOpen}
	<div
			transition:slide
			class="md:hidden absolute w-full left-0 flex flex-col gap-4 p-6 border-t {isHomePage ? 'bg-white border-gray-100' : 'bg-[#191F00] border-[#344100]'} z-40 shadow-xl"
	>
		<ol class="flex flex-col gap-3 list-none items-center">
			<li class="w-full">
				<Link
						on:click={toggleMenu}
						href="/"
						variant={isHomePage ? "ghost" : "second"}
						size="sm"
						className="w-full"
				>
					Головна
				</Link>
			</li>
			<li class="w-full">
				<Link on:click={toggleMenu} href="/tournaments" variant={isHomePage ? "ghost" : "second"} size="sm" className="w-full">Турніри</Link>
			</li>
			<hr class="w-full opacity-5 my-1" />
			<li class="w-full">
				<Link on:click={toggleMenu} href="/login" variant={isHomePage ? "ghost" : "second"} size="sm" className="w-full">Увійти</Link>
			</li>
			<li class="w-full">
				<Link on:click={toggleMenu} href="/registration" variant={isHomePage ? "primary" : "green"} size="sm" className="w-full">Реєстрація</Link>
			</li>
		</ol>
	</div>
{/if}

<slot></slot>