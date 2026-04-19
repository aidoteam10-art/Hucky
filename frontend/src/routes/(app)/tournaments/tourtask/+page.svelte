<script>
  let timeLeft = "30:23:50:23";
  let tournament = {
    id: 1,
    roundId: 1,
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

<div class="max-w-7xl mx-auto mt-10 pb-21 space-y-6">

  <div class="bg-[#191F00] text-white rounded-3xl p-15 pl-21 pr-14 mb-12.5 flex justify-between items-center">
    <div>
      <h2 class="text-lg font-bold mb-12 text-[1.5rem]">{tournament.round}</h2>
      <a href={`/tournaments/${tournament.id}/rounds/${tournament.roundId}/submission`} class="text-[1rem] font-bold bg-linear-to-r from-[#BCEB01] to-[#EEFF00] mt-4 text-[#191F00] px-7.5 py-4 rounded-lg hover:brightness-90 transition-all">
        Здати роботу
      </a>
    </div>

    <div class="text-right">
      <p class="font-regular text-[1rem]">Часу залишилося</p>
      <p class="text-[2.5rem] font-semibold mt-5">{timeLeft}</p>
    </div>
  </div>

  <div class="border border-[#B4B4B4] rounded-2xl px-7.5 py-9 bg-white ">
    <div class="flex items-center gap-2 mb-3 text-gray-900 font-medium">
      <img src="/icons/task_desc.svg" alt="task_description">
      <span class='font-semibold text-[#191F00]'>Опис завдання</span>
    </div>
    <p class="text-[191F00] font-regular text-[1rem]">{tournament.taskDescription}</p>
  </div>

  <div class="border border-[#B4B4B4] rounded-2xl px-7.5 py-9 bg-white ">
    <div class="flex items-center gap-2 mb-3 text-gray-900 font-medium">
      <span class='font-semibold text-[#191F00]'>Необхідні технології</span>
    </div>
    <p class="text-[191F00] font-regular text-[1rem]">{tournament.technologies}</p>
  </div>

  <div class="border border-[#B4B4B4] rounded-2xl px-7.5 py-9 bg-white">
    <div class="flex items-center gap-2 mb-8 text-gray-900 font-medium">
      <img src="/icons/required.svg" alt="required">
      <span class='font-semibold text-[#191F00]'>Обов’язкові вимоги</span>
    </div>

    {#each tournament.mandatoryRequirements as requirement, i}
      <div class="bg-[#CCFF00] text-[#191F00] rounded-xl p-5 mb-3 text-[1rem] font-regular">
        <div class="flex items-center gap-4">
            <span class="w-8 h-8 flex items-center justify-center border border-current font-regular rounded-lg text-[1rem]">
              {i + 1}
            </span>

          <p class="font-regular text-[16px]">
            {requirement}
          </p>
        </div>
      </div>
    {/each}
  </div>

  <div class="border border-[#B4B4B4] rounded-2xl px-7.5 py-9 bg-white mb-4">
    <div class="flex items-center gap-3 mb-5 text-gray-900 font-medium">
      <img src="/icons/schedule.svg" alt="schedule">
      <span class='font-semibold text-[#191F00]'>Розклад</span>
    </div>
    <div class="flex w-full">
      <div class="flex-1 text-left">
        <h3 class = "font-semibold text-[1.2rem] text-[#696969] mb-2">Початок</h3>
        <p class="font-semibold text-[#191F00] text-[1.2rem] tracking-[0.15em]">21.03.2026, 10:00</p>
      </div>

      <div class="flex-1 text-left">
        <h3 class = "font-semibold text-[1.2rem] text-[#696969] mb-2">Дедлайн</h3>
        <p class="font-semibold text-[#191F00] text-[1.2rem] tracking-[0.15em]">30.04.2026, 17:00</p>
      </div>
    </div>
  </div>

</div>