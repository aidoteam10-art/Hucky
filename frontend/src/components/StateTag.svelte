<script>
    // Доступні варіанти: "registration", "active", "draft", "completed"
    export let variant = "registration";
    let customClass = "";
    export { customClass as class };

    const styles = {
        registration: "bg-[#191F00] text-[#CCFF00] ring-[1.5px] ring-[#CCFF00]",
        active: "bg-[#191F00] text-[#F97316] ring-[1.5px] ring-[#F97316]", // Оранжевий
        draft: "bg-[#191F00] text-gray-200 ring-[1.5px] ring-gray-400", // Світло-сірий
        completed: "bg-[#191F00] text-gray-400 ring-[1.5px] ring-gray-500", // Темнішо-сірий
        participant: "bg-[#191F00] text-[#CCFF00]",
        jury: "bg-[#191F00] text-[#F97316] ring-[1.5px] ring-[#F97316]", // Оранжевий
        admin: "bg-[#191F00] text-gray-400 ring-[1.5px] ring-gray-500" // Темнішо-сірий
    };

    const labels = {
        registration: "Реєстрація",
        active: "Активний",
        draft: "Чернетка",
        completed: "Завершений",
        participant: "Учасник",
        jury: "Журі",
        admin: "Адмін"
    };
</script>

<span class="inline-flex items-center justify-center px-[1em] py-[0.35em] rounded-[2em] font-light tracking-wide select-none {styles[variant] || styles.registration} {customClass || 'text-[0.75rem] lg:text-[0.95rem]'}">
    <slot>{labels[variant] || labels.registration}</slot>
</span>
