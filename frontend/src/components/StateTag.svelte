<script>
	export let variant = 'registration';
	let customClass = '';
	export { customClass as class };

	const styles = {
		registration: 'bg-[#191F00] text-[#CCFF00] ring-[1.5px] ring-[#CCFF00]',
		running: 'bg-[#191F00] text-[#F97316] ring-[1.5px] ring-[#F97316]',
		active: 'bg-[#191F00] text-[#F97316] ring-[1.5px] ring-[#F97316]',
		draft: 'bg-[#191F00] text-gray-200 ring-[1.5px] ring-gray-400',
		finished: 'bg-[#191F00] text-gray-400 ring-[1.5px] ring-gray-500',
		completed: 'bg-[#191F00] text-gray-400 ring-[1.5px] ring-gray-500',
		submission_closed: 'bg-[#191F00] text-sky-300 ring-[1.5px] ring-sky-300',
		evaluated: 'bg-[#191F00] text-gray-400 ring-[1.5px] ring-gray-500',
		participant: 'bg-[#191F00] text-[#CCFF00]',
		jury: 'bg-[#191F00] text-[#F97316] ring-[1.5px] ring-[#F97316]',
		admin: 'bg-[#191F00] text-gray-400 ring-[1.5px] ring-gray-500'
	};

	const labels = {
		registration: 'Реєстрація',
		running: 'Триває',
		active: 'Активний',
		draft: 'Чернетка',
		finished: 'Завершений',
		completed: 'Завершений',
		submission_closed: 'Прийом закрито',
		evaluated: 'Оцінено',
		participant: 'Учасник',
		jury: 'Журі',
		admin: 'Адмін'
	};
</script>

<span
	class="inline-flex items-center justify-center rounded-[2em] px-[1em] py-[0.35em] font-light tracking-wide select-none {styles[
		variant
	] || styles.registration} {customClass || 'text-[0.75rem] lg:text-[0.95rem]'}"
>
	<slot>{labels[variant] || labels.registration}</slot>
</span>
