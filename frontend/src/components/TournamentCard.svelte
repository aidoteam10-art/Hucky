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

  let cardElement;
  let transformStyle = '';
  let isPressed = false;

  function updateTransform(e) {
    if (!cardElement) return;
    const rect = cardElement.getBoundingClientRect();
    const x = e.clientX - rect.left;
    const y = e.clientY - rect.top;
    
    const xRel = (x / rect.width) * 2 - 1;
    const yRel = (y / rect.height) * 2 - 1;
    
    const tiltMultiplier = isPressed ? 7 : 5;
    const scale = isPressed ? 1 : 1.02;
    
    const rotateX = -yRel * tiltMultiplier;
    const rotateY = xRel * tiltMultiplier;
    
    transformStyle = `transform: perspective(1000px) rotateX(${rotateX}deg) rotateY(${rotateY}deg) scale3d(${scale}, ${scale}, ${scale}); transition: transform 0.1s ease-out;`;
  }

  function handlePointerMove(e) {
    updateTransform(e);
  }

  function handlePointerDown(e) {
    isPressed = true;
    updateTransform(e);
  }

  function handlePointerUp(e) {
    isPressed = false;
    updateTransform(e);
  }

  function handlePointerLeave() {
    isPressed = false;
    transformStyle = `transform: perspective(1000px) rotateX(0deg) rotateY(0deg) scale3d(1, 1, 1); transition: transform 0.3s ease-out;`;
    setTimeout(() => {
        transformStyle = '';
    }, 300);
  }
</script>

<article 
  bind:this={cardElement}
  on:pointermove={handlePointerMove}
  on:pointerdown={handlePointerDown}
  on:pointerup={handlePointerUp}
  on:pointerleave={handlePointerLeave}
  style={transformStyle}
  class="flex flex-col p-8 rounded-[18px] aspect-video {transformStyle ? '' : 'transition-transform duration-200 hover:scale-[1.02]'} {variant === 'green' ? 'bg-linear-to-r from-[#BCEB01] to-[#EEFF00]' : 'border border-[#c5c5c5] bg-[linear-gradient(180deg,#f4f4f4_0%,#ececec_100%)]'}">
    
    <div class = "flex justify-between items-center mb-2">
      
      <span><StateTag variant = "{current_state}"></StateTag></span>
      <span class = "select-none">{formattedRounds}</span>
    
    </div>
    
    <h3 class = "font-black text-[1.75rem] mb-4 select-none line-clamp-1">{title}</h3>
    <p class = " text-[1.25rem] mb-3 select-none line-clamp-2">{description}</p>
    
    <div class = "mt-auto">
      <div class = "flex mb-6 items-center gap-4">
        
        <div class = "flex gap-2">
          
          <img src="icons/calendar.svg" alt="Calendar icon" class = "w-4 h-auto select-none">
          <span class = "select-none">{formattedStartDate}</span>
        
        </div>

        <div class = "flex gap-2">
          <img src="icons/team_tournament.svg" alt="Teams icon" class = "w-3.5 h-auto select-none">
          <span class = "select-none">{registered_teams}/{max_teams} команд</span>
        </div>
      </div>
    </div>

    <a href = "/tournaments/{id}" class = "font-bold text-[1.25rem] flex gap-2 items-center w-fit select-none">Детальніше
      <img src = "arrow_right.svg" class = "w-2" alt="right arrow">
    </a>

</article>

