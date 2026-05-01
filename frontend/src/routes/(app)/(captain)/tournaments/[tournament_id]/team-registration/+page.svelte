<script>
    import Card from "/src/components/form/Card.svelte";
    import InputField from "/src/components/form/InputField.svelte";
    import Submit from "/src/components/form/Submit.svelte";

    let tournamentName = "Hackathon Ukraine 2026";

    let teamName = "";
    let organization = "";
    let contact = "";
    
    let captainName = "";
    let captainEmail = "";

    let participants = [{ email: "", name: "" }];

    let errors = { participants: [{}] };

    function addParticipant() {
        if (participants.length < 5) {
            participants = [...participants, { email: "", name: "" }];
            errors.participants = [...errors.participants, {}];
        }
    }

    function removeParticipant(index) {
        participants = participants.filter((_, i) => i !== index);
        errors.participants = errors.participants.filter((_, i) => i !== index);
    }

    function validate() {
        let valid = true;
        errors = { participants: participants.map(() => ({})) };

        if (!teamName || teamName.length < 3) {
            errors.teamName = "Назва повинна містити мінімум 3 символи";
            valid = false;
        }

        if (!captainName) {
            errors.captainName = "Поле обов'язкове";
            valid = false;
        }

        if (!captainEmail || !/^[^\s@]+@[^\s@]+\.[^\s@]+$/.test(captainEmail)) {
            errors.captainEmail = "Невірний email";
            valid = false;
        }

        const emails = new Set(captainEmail ? [captainEmail] : []);

        participants.forEach((p, i) => {
            if (!p.name) {
                errors.participants[i].name = "Поле обов'язкове";
                valid = false;
            }
            if (!p.email || !/^[^\s@]+@[^\s@]+\.[^\s@]+$/.test(p.email)) {
                errors.participants[i].email = "Невірний email";
                valid = false;
            } else if (emails.has(p.email)) {
                errors.participants[i].email = "Email вже використовується";
                valid = false;
            }
            if (p.email) emails.add(p.email);
        });

        return valid;
    }

    function handleSubmit() {
        if (validate()) {
            console.log("Team Registration Valid", { teamName, organization, contact, captainName, captainEmail, participants });
        }
    }
</script>

<form on:submit|preventDefault={handleSubmit} class="flex flex-col gap-6 max-w-4xl mx-auto w-full p-6 mt-10">
    <div class="flex flex-col items-start gap-1 text-center mb-2 px-2">
        <h1 class="text-[1.75rem] font-bold text-[#1F1F1F]">Зареєструйте вашу команду</h1>
        <h2 class="text-xs text-[#756157]">Реєстрація команди на <b>{tournamentName}</b></h2>
    </div>

    <Card class="flex flex-col gap-5 w-full">
        <h2 class="text-base font-bold text-[#32221B]">Інформація про команду</h2>

        <InputField bind:value={teamName} error={errors.teamName} header="Назва команди *" placeholder="e.g. Code Warriors" class="w-full" />

        <div class="flex flex-col sm:flex-row gap-8 w-full">
            <InputField bind:value={organization} header="Організація" placeholder="Навчальний заклад / Компанія" class="flex-1" />
            <InputField bind:value={contact} error={errors.contact} header="Контакт (Telegram / Discord)" placeholder="username, нікнейм або посилання" class="flex-1" />
        </div>
    </Card>

    <Card class="flex flex-col gap-5 w-full">
        <h2 class="text-base font-bold text-[#32221B]">Капітан команди</h2>

        <div class="flex flex-col sm:flex-row gap-8 w-full">
            <InputField bind:value={captainName} error={errors.captainName} header="Повне ім'я *" placeholder="ПІБ капітана" class="flex-1" />
            <InputField bind:value={captainEmail} error={errors.captainEmail} header="Email *" type="email" placeholder="hello@example.com" class="flex-1" />
        </div>
    </Card>

    <Card class="flex flex-col gap-5 w-full">
        <div class="flex justify-between items-center px-1">
            <h2 class="text-base font-bold text-[#32221B]">Учасники команди</h2>
            {#if participants.length < 5}
            <button on:click={addParticipant} class="text-sm font-bold text-[#32221B] flex items-center gap-1 hover:underline cursor-pointer" type="button">
                <span class="text-lg leading-none pb-0.5">+</span> Додати учасника
            </button>
            {/if}
        </div>

        {#each participants as participant, i}
        <div class="flex flex-col sm:flex-row gap-8 w-full relative">
            <InputField bind:value={participant.email} error={errors.participants[i]?.email} header="Email *" type="email" placeholder="hello@example.com" class="flex-1" />
            <InputField bind:value={participant.name} error={errors.participants[i]?.name} header="Повне ім'я *" placeholder="ПІБ учасника" class="flex-1" />
            
            {#if participants.length > 1}
            <button on:click={() => removeParticipant(i)} type="button" class="absolute -right-3 top-8 text-red-500 hover:text-red-700 font-bold px-2 py-1 leading-none rounded-full cursor-pointer" title="Видалити">✕</button>
            {/if}
        </div>
        {/each}

        <p class="text-[0.625rem] text-[#756157] mt-1 pl-1">Мінімум 2 учасники, максимум 6. ({participants.length + 1} / 6)</p>
    </Card>

    <Submit class="w-full mx-auto mt-4 h-12 flex justify-center items-center" title="Зареєструвати команду" />
</form>
