<script>
    import Card from "/src/components/form/Card.svelte";
    import InputField from "/src/components/form/InputField.svelte";
    import TextArea from "/src/components/form/TextArea.svelte";
    import DateField from "/src/components/form/DateField.svelte";
    import Submit from "/src/components/form/Submit.svelte";

    let title = "";
    let description = "";
    
    let regStart = "";
    let regEnd = "";
    let tourStart = "";
    let maxTeams = "";
    
    let roundTitle = "";
    let roundDesc = "";
    let techStack = "";
    let roundStart = "";
    let roundDeadline = "";

    let requirements = [""];
    
    let errors = { requirements: [] };

    function addRequirement() {
        requirements = [...requirements, ""];
        errors.requirements = [...errors.requirements, ""];
    }

    function saveDraft() {
        const draft = { title, description, regStart, regEnd, tourStart, maxTeams, roundTitle, roundDesc, techStack, roundStart, roundDeadline, requirements };
        localStorage.setItem("tournamentDraft", JSON.stringify(draft));
        alert("Чернетка збережена!");
    }

    function validate() {
        errors = { requirements: requirements.map(() => "") };
        let valid = true;

        if (!title || title.length < 5) {
            errors.title = "Мінімум 5 символів";
            valid = false;
        }

        if (!description || description.length < 50) {
            errors.description = "Опис має містити мінімум 50 символів";
            valid = false;
        }

        const now = new Date();
        const rStart = new Date(regStart);
        const rEnd = new Date(regEnd);
        const tStart = new Date(tourStart);
        const rndStart = new Date(roundStart);
        const rndDeadline = new Date(roundDeadline);

        if (!regStart) { errors.regStart = "Обов'язково"; valid = false; }
        else if (rStart < now) { errors.regStart = "Дата не може бути в минулому"; valid = false; }

        if (!regEnd) { errors.regEnd = "Обов'язково"; valid = false; }
        else if (rEnd <= rStart) { errors.regEnd = "Завершення має бути після початку"; valid = false; }

        if (!tourStart) { errors.tourStart = "Обов'язково"; valid = false; }
        else if (tStart < rEnd) { errors.tourStart = "Турнір має початися після завершення реєстрації"; valid = false; }

        if (!maxTeams || isNaN(maxTeams) || Number(maxTeams) < 2) {
            errors.maxTeams = "Мінімум 2 команди";
            valid = false;
        }

        if (!roundTitle) { errors.roundTitle = "Обов'язково"; valid = false; }
        if (!roundDesc || roundDesc.length < 20) { errors.roundDesc = "Мінімум 20 символів"; valid = false; }

        if (!roundStart) { errors.roundStart = "Обов'язково"; valid = false; }
        else if (rndStart < tStart) { errors.roundStart = "Раунд не може початися до початку турніру"; valid = false; }

        if (!roundDeadline) { errors.roundDeadline = "Обов'язково"; valid = false; }
        else if (rndDeadline <= rndStart) { errors.roundDeadline = "Дедлайн має бути після початку раунду"; valid = false; }

        let hasValidReq = false;
        requirements.forEach((req, i) => {
            if (!req.trim()) {
                errors.requirements[i] = "Поле обов'язкове";
            } else {
                hasValidReq = true;
            }
        });
        if (!hasValidReq) valid = false;

        return valid;
    }

    function handleSubmit() {
        if (validate()) {
            console.log("Valid tournament:", { title, description, regStart, regEnd, tourStart, roundStart, requirements });
        }
    }
</script>

<form on:submit|preventDefault={handleSubmit} class="flex flex-col gap-6 max-w-4xl w-full mt-10 p-6 mx-auto">
    <div class="flex justify-between items-start mb-2 px-2 gap-3">
        <div class="flex flex-col gap-1">
            <h1 class="text-[1.75rem] font-bold text-[#1F1F1F]">Створіть турнір</h1>
            <h2 class="text-sm font-bold text-[#1F1F1F]">Деталі турніру</h2>
        </div>
        <button on:click={saveDraft} class="flex items-center gap-2 border border-[#756157] rounded-lg px-4 py-2 bg-transparent text-sm font-semibold text-[#1F1F1F] hover:bg-black/5 transition-colors cursor-pointer" type="button">
            <div class="w-5 h-5 rounded-full bg-[#CCFF00] flex items-center justify-center font-bold text-lg leading-none pt-0.5 text-black">+</div>
            Зберегти чернетку
        </button>
    </div>

    <Card class="flex flex-col gap-5 w-full">
        <h2 class="text-base font-bold text-[#32221B]">Деталі турніру</h2>
        <InputField bind:value={title} error={errors.title} header="Назва*" placeholder="e.g. Spring Hackathon" class="w-full" />
        <TextArea bind:value={description} error={errors.description} header="Опис / Правила" placeholder="Правила турніру, формат, цілі..." rows={4} class="w-full" />
    </Card>

    <Card class="flex flex-col gap-5 w-full">
        <h2 class="text-base font-bold text-[#32221B]">Реєстрація і розклад</h2>
        <div class="flex flex-col sm:flex-row gap-8 w-full">
            <DateField bind:value={regStart} error={errors.regStart} header="Початок реєстрації *" class="flex-1" />
            <DateField bind:value={regEnd} error={errors.regEnd} header="Завершення реєстрації *" class="flex-1" />
        </div>
        <div class="flex flex-col sm:flex-row gap-8 w-full">
            <DateField bind:value={tourStart} error={errors.tourStart} header="Початок турніру*" class="flex-1" />
            <InputField bind:value={maxTeams} error={errors.maxTeams} type="number" header="Максимальна кількість команд *" placeholder="Наприклад, 20" class="flex-1" />
        </div>
    </Card>

    <Card class="flex flex-col gap-5 w-full">
        <h2 class="text-base font-bold text-[#32221B]">Перший раунд</h2>
        <InputField bind:value={roundTitle} error={errors.roundTitle} header="Назва раунду*" placeholder="e.g. Round 1: MVP Development" class="w-full" />
        <TextArea bind:value={roundDesc} error={errors.roundDesc} header="Опис завдання *" placeholder="Що команди мають зробити..." rows={4} class="w-full" />
        <TextArea bind:value={techStack} header="Необхідні технології" placeholder="Необхідний стек..." rows={2} class="w-full" />

        <div class="flex flex-col gap-3 mt-2 w-full">
            <div class="flex justify-between items-center px-1">
                <h3 class="text-sm font-bold text-[#32221B]">Обов'язкові вимоги</h3>
                <button on:click={addRequirement} class="text-sm font-bold text-[#32221B] flex items-center gap-1 hover:underline cursor-pointer" type="button">
                    <span class="text-lg leading-none">+</span> Додати
                </button>
            </div>
            {#each requirements as req, i}
                <InputField bind:value={requirements[i]} error={errors.requirements[i]} placeholder="Вимога {i + 1}" header="" class="w-full" />
            {/each}
        </div>

        <div class="flex flex-col sm:flex-row gap-8 mt-2 w-full">
            <DateField bind:value={roundStart} error={errors.roundStart} header="Початок раунду *" class="flex-1" />
            <DateField bind:value={roundDeadline} error={errors.roundDeadline} header="Дедлайн *" class="flex-1" />
        </div>
    </Card>

    <Submit class="w-full mt-4 h-12 flex justify-center items-center" title="Створити турнір" />
</form>
