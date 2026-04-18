<script>
  let timeLeft = "30:23:50:23";
  let tournament = {
    round: "Раунд 1: MVP Development",
    taskDescription: "Створіть мінімально життєздатний продукт для програми управління завданнями з функціями співпраці в команді.",
    technologies: "React/Next.js frontend, Node.js/Python backend, PostgreSQL database, deployed to a cloud platform.",
    mandatoryRequirements: [
      "Автентифікація користувача (реєстрація/вхід)",
      "CRUD-операції для завдань",
      "Функція призначення команди",
      "REST API з належною обробкою помилок",
      "База даних щонайменше з 3 таблицями",
      "Адаптивний дизайн"
    ],
    startDate: "21.03.2026, 10:00",
    endDate: "30.04.2026, 17:00"
  }
</script>

<div class="max-w-7xl mx-auto mt-10 px-6 md:px-10 xl:px-0 pb-21 space-y-6 font-sans">

  <div class="bg-[#191F00] text-white rounded-3xl p-8 md:p-12 lg:p-15 lg:pl-21 lg:pr-14 mb-12.5 flex flex-col md:flex-row justify-between items-center gap-8 text-center md:text-left">
    <div class="w-full md:w-auto">
      <h2 class="text-xl md:text-[1.5rem] font-bold mb-8 md:mb-12">{tournament.round}</h2>
      <a href="/task_submission" class="inline-block text-[1rem] font-bold bg-linear-to-r from-[#BCEB01] to-[#EEFF00] text-[#191F00] px-9 py-3.5 rounded-lg hover:brightness-90 transition-all w-full md:w-auto text-center">
        Здати роботу
      </a>
    </div>

    <div class="md:text-right">
      <p class="font-regular text-[1rem] opacity-80">Часу залишилося</p>
      <p class="text-3xl md:text-[2.5rem] font-semibold mt-3 md:mt-5 tracking-tight md:tracking-normal">{timeLeft}</p>
    </div>
  </div>

  <div class="border border-[#B4B4B4] rounded-2xl px-6 lg:px-7.5 py-7 lg:py-9 bg-white">
    <div class="flex items-center gap-2 mb-3 text-gray-900 font-medium">
      <img src="/icons/task_desc.svg" alt="task_description" class="w-5 h-5 lg:w-auto">
      <span class='font-semibold text-[#191F00]'>Опис завдання</span>
    </div>
    <p class="text-[#191F00] font-regular text-[1rem] leading-relaxed">{tournament.taskDescription}</p>
  </div>

  <div class="border border-[#B4B4B4] rounded-2xl px-6 lg:px-7.5 py-7 lg:py-9 bg-white">
    <div class="flex items-center gap-2 mb-3 text-gray-900 font-medium">
      <span class='font-semibold text-[#191F00]'>Необхідні технології</span>
    </div>
    <p class="text-[#191F00] font-regular text-[1rem] leading-relaxed">{tournament.technologies}</p>
  </div>

  <div class="border border-[#B4B4B4] rounded-2xl px-6 lg:px-7.5 py-7 lg:py-9 bg-white">
    <div class="flex items-center gap-2 mb-6 lg:mb-8 text-gray-900 font-medium">
      <img src="/icons/required.svg" alt="required" class="w-5 h-5 lg:w-auto">
      <span class='font-semibold text-[#191F00]'>Обов’язкові вимоги</span>
    </div>

    <div class="space-y-3">
      {#each tournament.mandatoryRequirements as requirement, i}
        <div class="bg-[#CCFF00] text-[#191F00] rounded-xl p-4 lg:p-5 text-[1rem] font-regular">
          <div class="flex items-center gap-4">
              <span class="shrink-0 w-8 h-8 flex items-center justify-center border border-[#191F00]/20 font-regular rounded-lg text-[1rem]">
                {i + 1}
              </span>
            <p class="font-regular text-[16px]">
              {requirement}
            </p>
          </div>
        </div>
      {/each}
    </div>
  </div>

  <div class="border border-[#B4B4B4] rounded-2xl px-6 lg:px-7.5 py-7 lg:py-9 bg-white mb-4">
    <div class="flex items-center gap-3 mb-6 lg:mb-8 text-gray-900 font-medium">
      <img src="/icons/schedule.svg" alt="schedule" class="w-5 h-5 lg:w-auto">
      <span class='font-semibold text-[#191F00]'>Розклад</span>
    </div>
    <div class="flex flex-col sm:flex-row w-full gap-8 sm:gap-0">
      <div class="flex-1 text-left">
        <h3 class="font-semibold text-[1.1rem] lg:text-[1.2rem] text-[#696969] mb-2">Початок</h3>
        <p class="font-semibold text-[#191F00] text-[1.1rem] lg:text-[1.2rem] tracking-[0.1em] lg:tracking-[0.15em]">{tournament.startDate}</p>
      </div>

      <div class="flex-1 text-left">
        <h3 class="font-semibold text-[1.1rem] lg:text-[1.2rem] text-[#696969] mb-2">Дедлайн</h3>
        <p class="font-semibold text-[#191F00] text-[1.1rem] lg:text-[1.2rem] tracking-[0.1em] lg:tracking-[0.15em]">{tournament.endDate}</p>
      </div>
    </div>
  </div>

</div>