<script>
  import StateTag from "./StateTag.svelte";
  export let variant = 'green';
  export let current_state = "active"
  export let title = "Hackathon Ukraine 2026";
  export let description = "Національний хакатон для студентських команд. Створюйте інноваційні рішення за 48 годин.";
  export let start_date = "2026-03-15T08:44:21.000Z";
  export let rounds = 5;
  export let max_teams = 50;
  export let registered_teams = 24;
  export let id = 0;

  function formatIsoDate(isoString){
    if (!isoString) return "";
    const date = new Date(isoString);
    const shortMonths = ["Січ", "Лют", "Бер", "Кві", "Тра", "Чер", "Лип", "Сер", "Вер", "Жов", "Лис", "Гру"];
    return `${date.getDate()} ${shortMonths[date.getMonth()]} ${date.getFullYear()}`
  }

  function formatRounds(numRounds){
    const hundredRemainder = numRounds % 100;
    const tenRemainder = numRounds % 10;
    let round_word = ""
    if  ((hundredRemainder > 10 && hundredRemainder < 21) || tenRemainder == 0){
      round_word = "раундів";
    }
    else if (tenRemainder > 1 && tenRemainder < 5){
      round_word = "раунди";
    }
    else if (tenRemainder == 1){
      round_word = "раунд";
    } else{
      round_word = "раундів";
    }


    return `${numRounds} ${round_word}`
  }
  $: formattedRounds = formatRounds(rounds);
  $: formattedStartDate = formatIsoDate(start_date);
</script>

<article class="flex flex-col p-8 rounded-[18px] transition-transform duration-200 hover:scale-[1.02] active:scale-[0.98] {variant === 'green' ? 'bg-linear-to-r from-[#BCEB01] to-[#EEFF00]' : 'border border-[#c5c5c5] bg-[linear-gradient(180deg,#f4f4f4_0%,#ececec_100%)]'}">
    <div class = "flex justify-between items-center mb-2">
      <span><StateTag variant = "{current_state}"></StateTag></span>
      <span>{formattedRounds}</span>
    </div>
    <h3 class = "font-black text-[1.75rem] mb-4">{title}</h3>
    <p class = "text-[1.25rem] mb-3">{description}</p>
    <div class = "flex mb-6 items-center gap-4">
      <div class = "flex gap-2">
      <img src="icons/calendar.svg" alt="Calendar icon" class = "w-4 h-a">
        <span>{formattedStartDate}</span>
      </div>

      <div class = "flex gap-2">
        <img src="icons/team_tournament.svg" alt="Teams icon" class = "w-3.5 h-a">
        <span>{registered_teams}/{max_teams} команд</span>
      </div>
    </div>

    <a href = "/tournaments/{id}" class = "font-bold text-[1.25rem] flex gap-2 items-center w-fit">Детальніше
      <img src = "arrow_right.svg" class = "w-2" alt="right arrow">
    </a>

</article>

