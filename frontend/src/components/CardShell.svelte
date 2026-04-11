<div class="card-shell">
  <slot></slot>
</div>

<style>
  .card-shell {
    min-height: 292px;
    border: 1px solid #c5c5c5;
    border-radius: 18px;
    overflow: hidden;
    background: linear-gradient(180deg, #f4f4f4 0%, #ececec 100%);
  }

  .card-shell > :global(*) {
    background: linear-gradient(180deg, #f4f4f4 0%, #ececec 100%) !important;
    border: none !important;
    box-shadow: none !important;
    border-radius: 18px !important;
    min-height: 292px;
  }

  .card-shell :global(article) {
    background: linear-gradient(180deg, #f4f4f4 0%, #ececec 100%) !important;
    border: none !important;
    box-shadow: none !important;
    min-height: 292px;
  }

  .card-shell :global(.tournament-card),
  .card-shell :global(.card),
  .card-shell :global(div) {
    box-shadow: none !important;
  }
</style>