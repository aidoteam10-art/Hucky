<script>
  const tournament = { name: "Hackathon Ukraine 2026" };

  const teams = Array.from({ length: 6 }, (_, i) => ({
    id: i,
    name: "Code Warriors",
    scores: {
      backend: 88, database: 92, frontend: 89, requirements: 95, functionality: 90, usability: 87
    },
    total: 537
  }));
</script>

<div style="max-width: 1440px; width: 100%; margin: 0 auto; padding: 32px 24px;">
  
  <h1 style="font-size: 36px; font-weight: 700; color: #111; margin-bottom: 32px;">
    {tournament.name}
  </h1>

  <div style="display: flex; justify-content: center; align-items: flex-end; margin-bottom: 40px;">

    <div style="width: 240px; height: 260px; flex-shrink: 0; background: #fff; border: 1px solid #111; border-radius: 16px 0 0 16px; display: flex; flex-direction: column; overflow: hidden; position: relative; z-index: 1; margin-right: -1px;">
      <div style="height: 120px; border-bottom: 1px solid #111; padding: 24px; display: flex; justify-content: space-between; align-items: flex-start;">
        <div style="font-size: 15px; font-weight: 700; line-height: 1.2; color: #111;">SILVER<br/>MEDALIST</div>
        <div style="font-size: 48px; font-weight: 800; line-height: 1; color: #111;">02</div>
      </div>
      <div style="flex-grow: 1; background: #F8F9FA; padding: 24px; display: flex; flex-direction: column; justify-content: space-between;">
        <div>
          <div style="font-size: 16px; font-weight: 700; color: #111;">Code Warriors</div>
          <div style="font-size: 11px; font-weight: 500; color: #6B7280; margin-top: 4px;">MIT Research Labs</div>
        </div>
        <div style="font-size: 32px; font-weight: 800; color: #111;">500</div>
      </div>
    </div>

    <div style="width: 280px; height: 320px; flex-shrink: 0; background: #fff; border: 1px solid #111; border-radius: 16px 16px 0 0; display: flex; flex-direction: column; overflow: hidden; position: relative; z-index: 10; box-shadow: 0 10px 25px rgba(0,0,0,0.05);">
      <div style="height: 140px; border-bottom: 1px solid #111; padding: 24px; display: flex; justify-content: space-between; align-items: flex-start;">
        <div style="font-size: 16px; font-weight: 700; line-height: 1.2; color: #111;">GRAND<br/>CHAMPION</div>
        <div style="font-size: 64px; font-weight: 800; line-height: 0.9; color: #111;">01</div>
      </div>
      <div style="flex-grow: 1; background: #FCF6D6; padding: 24px; display: flex; flex-direction: column; justify-content: space-between;">
        <div>
          <div style="font-size: 20px; font-weight: 700; color: #111;">Code Warriors</div>
          <div style="font-size: 12px; font-weight: 500; color: #6B7280; margin-top: 4px;">MIT Research Labs</div>
        </div>
        <div style="font-size: 40px; font-weight: 800; color: #111;">537</div>
      </div>
    </div>

    <div style="width: 240px; height: 200px; flex-shrink: 0; background: #fff; border: 1px solid #111; border-radius: 0 16px 16px 0; display: flex; flex-direction: column; overflow: hidden; position: relative; z-index: 1; margin-left: -1px;">
      <div style="height: 85px; border-bottom: 1px solid #111; padding: 16px 24px; display: flex; justify-content: space-between; align-items: flex-start;">
        <div style="font-size: 15px; font-weight: 700; line-height: 1.2; color: #111;">BRONZE<br/>MEDALIST</div>
        <div style="font-size: 48px; font-weight: 800; line-height: 1; color: #111;">03</div>
      </div>
      <div style="flex-grow: 1; background: #FBE4D6; padding: 0 24px; display: flex; flex-direction: column; justify-content: center; gap: 8px;">
        <div>
          <div style="font-size: 16px; font-weight: 700; color: #111;">Code Warriors</div>
          <div style="font-size: 11px; font-weight: 500; color: #6B7280; margin-top: 4px;">MIT Research Labs</div>
        </div>
        <div style="font-size: 32px; font-weight: 800; color: #111;">490</div>
      </div>
    </div>

  </div>

  <div style="border: 1px solid #111; border-radius: 16px; overflow: hidden; background: #fff;">
    <table style="width: 100%; text-align: left; border-collapse: collapse; font-size: 14px;">
      
      <thead style="background: linear-gradient(90deg, #BCEB01 0%, #EEFF00 100%); border-bottom: 1px solid #111;">
        <tr>
          <th style="padding: 20px 24px; font-weight: 700; color: #111; width: 60px;">#</th>
          <th style="padding: 20px 24px; font-weight: 700; color: #111;">Team</th>
          <th style="padding: 20px 16px; font-weight: 700; color: #111; text-align: center;">Backend</th>
          <th style="padding: 20px 16px; font-weight: 700; color: #111; text-align: center;">Database</th>
          <th style="padding: 20px 16px; font-weight: 700; color: #111; text-align: center;">Frontend</th>
          <th style="padding: 20px 16px; font-weight: 700; color: #111; text-align: center;">Requirements</th>
          <th style="padding: 20px 16px; font-weight: 700; color: #111; text-align: center;">Functionality</th>
          <th style="padding: 20px 16px; font-weight: 700; color: #111; text-align: center;">Usability</th>
          <th style="padding: 20px 24px; font-weight: 700; color: #111; text-align: center;">Total</th>
        </tr>
      </thead>
      
      <tbody>
        {#each teams as team, i (team.id)}
          <tr style="border-bottom: 1px solid #E5E7EB;">
            <td style="padding: 20px 24px; font-weight: 700; color: #111;">{i + 1}</td>
            <td style="padding: 20px 24px; font-weight: 700; color: #111;">{team.name}</td>
            <td style="padding: 20px 16px; font-weight: 500; color: #6B7280; text-align: center;">{team.scores.backend}</td>
            <td style="padding: 20px 16px; font-weight: 500; color: #6B7280; text-align: center;">{team.scores.database}</td>
            <td style="padding: 20px 16px; font-weight: 500; color: #6B7280; text-align: center;">{team.scores.frontend}</td>
            <td style="padding: 20px 16px; font-weight: 500; color: #6B7280; text-align: center;">{team.scores.requirements}</td>
            <td style="padding: 20px 16px; font-weight: 500; color: #6B7280; text-align: center;">{team.scores.functionality}</td>
            <td style="padding: 20px 16px; font-weight: 500; color: #6B7280; text-align: center;">{team.scores.usability}</td>
            <td style="padding: 20px 24px; font-weight: 700; color: #111; text-align: center;">{team.total}</td>
          </tr>
        {/each}
      </tbody>

    </table>
  </div>

</div>