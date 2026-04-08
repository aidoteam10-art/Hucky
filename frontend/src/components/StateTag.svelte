<script>
    // Доступні варіанти: "registration", "active", "draft", "completed"
    export let variant = "registration";

    const styles = {
        registration: "bg-[#191F00] text-[#CCFF00]",
        active: "bg-[#191F00] text-[#F97316] ring-[1.5px] ring-[#F97316]", // Оранжевий
        draft: "bg-[#191F00] text-gray-200 ring-[1.5px] ring-gray-400", // Світло-сірий
        completed: "bg-[#191F00] text-gray-400 ring-[1.5px] ring-gray-500" // Темнішо-сірий
    };

    const labels = {
        registration: "Реєстрація",
        active: "Активний",
        draft: "Чернетка",
        completed: "Завершений"
    };
</script>

<span class="inline-flex items-center justify-center px-4 py-1.5 rounded-4xl font-light text-[0.95rem] tracking-wide {styles[variant] || styles.registration}">
    <slot>{labels[variant] || labels.registration}</slot>
</span>
