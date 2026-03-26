<script>
  export let variant = 'green'; 
  export let title = "";
  export let description = "";
  export let start_date = "";
  export let max_teams = 0;
  export let registered_teams = 0;
  export let id = 0;
</script>

<article class = "flex flex-col">
    <h3>{title}</h3>
    <p>{description}</p>
    <div class = "flex">
      <div>{start_date}</div>
      <div>{registered_teams}/{max_teams}</div>
    </div>
    <a href = "/">Детальніше</a>
</article>

