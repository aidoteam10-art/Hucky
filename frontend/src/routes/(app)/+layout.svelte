<script>
  <slot></slot>
</script>



<nav class = " flex items-center justify-between">
	
		<div class = "flex items-center">
			<img src="/icons/logo_black.png" class="mt-4 mb-4 ml-12 w-14 h-a flex-shrink-0" alt="logo">
			<span class = "flex items-center  py-3 px-5">
				<span class = "font-bold text-3xl">H</span>
				<span class = "text-[#89AB00] font-bold text-3xl">u</span>
				<span class = "font-bold text-3xl">cky</span>
			</span>
		</div>

	<ol class = "flex items-center gap-6 list-none ">
		<li><a href="/" class = "btn no-underline font-bold text-l bg-[#191F00] text-white py-3 px-5 rounded hover:bg-[#344100] transition-colors duration-300 ease-out">Головна</a></li>
		<li><a href="/" class = "btn no-underline font-bold text-l py-3 px-5 hover:ring-1 hover:outline-[#191F00] rounded transition-shadow duration-200 ease-out">Турніри</a></li>
	</ol>

	<ol class = "flex items-center gap-6 list-none">
		<li><a href ="/login" class ="btn no-underline font-bold text-l py-3 px-5 hover:ring-1 rounded hover:ring-[#191F00] transition-shadow duration-200 ease-out" >Увійти</a></li>
		<li><a href = "/registration" class = "btn mr-18 no-underline font-bold text-l bg-[#191F00] text-white py-3 px-5 rounded hover:bg-[#344100] transition-colors duration-300 ease-out">Реєстрація</a></li>

	</ol>

</nav>

<slot></slot>
