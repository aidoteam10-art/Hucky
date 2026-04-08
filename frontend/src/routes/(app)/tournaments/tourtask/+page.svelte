<script>
  let timeLeft = "30:23:50:23";
</script>

<div class="max-w-6xl mx-auto mt-10 space-y-6">

  <div class="bg-gradient-to-r from-[#191F00] to-[#191F00] text-white rounded-2xl p-10 pl-14 pr-14 flex justify-between items-center">
    <div>
      <h2 class="text-lg font-semibold mb-4">Раунд 1: MVP Development</h2>
      <button class="bg-gradient-to-r from-[#BCEB01] to-[#EEFF00] mt-[16px] text-black px-4 py-2 rounded-lg font-medium hover:bg-lime-300 transition">
        Здати роботу
      </button>
    </div>

    <div class="text-right">
      <p class="text-sm opacity-80 text-[11px]">Часу залишилося</p>
      <p class="text-[24px] font-semibold mt-1">{timeLeft}</p>
    </div>
  </div>

  <div class="border border-gray-300 rounded-2xl p-5 bg-white ">
    <div class="flex items-center gap-2 mb-3 text-gray-900 font-medium">
      <span>&lt;&gt;</span>
      <span class='font-semibold'>Опис завдання</span>
    </div>
    <p class="text-gray-800 text-[14px]">
      Створіть мінімально життєздатний продукт для програми управління завданнями 
      з функціями співпраці в команді.
    </p>
  </div>

  <div class="border border-gray-300 rounded-2xl p-5 bg-white">
    <div class="font-semibold text-gray-900 mb-3">
      Необхідні технології
    </div>
    <p class="text-gray-800 text-[14px]">
      React/Next.js frontend, Node.js/Python backend, PostgreSQL database, 
      deployed to a cloud platform.
    </p>
  </div>

  <div class="border border-gray-300 rounded-2xl p-5 bg-white">
    <h3 class="font-semibold mb-5">Обов’язкові вимоги</h3>
    <div class="bg-[#ccff33] text-[#1a1a1a] rounded-xl p-4 mb-3">
        <div class="flex items-center gap-3">
          <span class="w-8 h-8 flex items-center justify-center border border-current font-bold rounded-lg text-[15px]">
            1
          </span>

          <p class="font-medium text-[16px]">
            Автентифікація користувача (реєстрація/вхід)
          </p>
        </div>
    </div>
    <div class="bg-[#ccff33] text-[#1a1a1a] rounded-xl p-4 mb-3">
        <div class="flex items-center gap-3">
          <span class="w-8 h-8 flex items-center justify-center border border-current font-bold rounded-lg text-[15px]">
            2
          </span>

          <p class="font-medium text-[16px]">
            CRUD-операції для завдань
          </p>
        </div>
    </div>
    <div class="bg-[#ccff33] text-[#1a1a1a] rounded-xl p-4 mb-3">
        <div class="flex items-center gap-3">
          <span class="w-8 h-8 flex items-center justify-center border border-current font-bold rounded-lg text-[15px]">
            3
          </span>

          <p class="font-medium text-[16px]">
            Функція призначення команди
          </p>
        </div>
    </div>
    <div class="bg-[#ccff33] text-[#1a1a1a] rounded-xl p-4 mb-3">
        <div class="flex items-center gap-3">
          <span class="w-8 h-8 flex items-center justify-center border border-current font-bold rounded-lg text-[15px]">
            4
          </span>

          <p class="font-medium text-[16px]">
            REST API з належною обробкою помилок
          </p>
        </div>
    </div>
    <div class="bg-[#ccff33] text-[#1a1a1a] rounded-xl p-4 mb-3">
        <div class="flex items-center gap-3">
          <span class="w-8 h-8 flex items-center justify-center border border-current font-bold rounded-lg text-[15px]">
            5
          </span>

          <p class="font-medium text-[16px]">
            Database with at least 3 tables
          </p>
        </div>
    </div>
    <div class="bg-[#ccff33] text-[#1a1a1a] rounded-xl p-4 mb-3">
        <div class="flex items-center gap-3">
          <span class="w-8 h-8 flex items-center justify-center border border-current font-bold rounded-lg text-[15px]">
            6
          </span>

          <p class="font-medium text-[16px]">
            Адаптивний дизайн
          </p>
        </div>
    </div>
    
  </div>
  <div class="border border-gray-300 rounded-2xl p-5 bg-white mb-4">
    <h3 class="font-semibold mb-5">Розклад</h3>
    <div class="flex w-full">
      <div class="flex-1 text-left">
        <h3>Початок</h3>
        <h3 class="font-semibold">21.03.2026, 10:00</h3>
      </div>

      <div class="flex-1 text-left">
        <h3>Дедлайн</h3>
        <h3 class="font-semibold">30.04.2026, 17:00</h3>
      </div>
    </div>
  </div>

</div>