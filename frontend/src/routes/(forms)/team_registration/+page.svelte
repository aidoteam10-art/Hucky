<script>
    import Card from "../../../components/Form/Card.svelte";
    import InputField from "../../../components/Form/InputField.svelte";
    import Submit from "../../../components/Form/Submit.svelte";

    let tournament_name = "Hackathon Ukraine 2026";
</script>

<div class="flex flex-col gap-6 max-w-4xl mx-auto w-full p-6 mt-10">
    <div class="flex flex-col items-start gap-1 text-center mb-2 px-2">
        <h1 class="text-[1.75rem] font-bold text-[#1F1F1F]">Зареєструйте вашу команду</h1>
        <h2 class="text-xs text-[#756157]">Реєстрація команди на <b>{tournament_name}</b></h2>
    </div>

    <Card class="flex flex-col gap-5 w-full">
        <h2 class="text-base font-bold text-[#32221B]">Інформація про команду</h2>
        
        <InputField header="Назва команди *" placeholder="e.g. Code Warriors" class="w-full" />
        
        <div class="flex flex-col sm:flex-row gap-8 w-full">
            <InputField header="Організація" placeholder="Навчальний заклад / Компанія" class="flex-1" />
            <InputField header="Контакт (Telegram / Discord)" placeholder="@username" class="flex-1" />
        </div>
    </Card>

    <Card class="flex flex-col gap-5 w-full">
        <h2 class="text-base font-bold text-[#32221B]">Капітан команди</h2>
        
        <div class="flex flex-col sm:flex-row gap-8 w-full">
            <InputField header="Повне ім'я *" placeholder="ПІБ капітана" class="flex-1" />
            <InputField header="Email *" type="email" placeholder="hello@example.com" class="flex-1" />
        </div>
    </Card>

    <Card class="flex flex-col gap-5 w-full">
        <div class="flex justify-between items-center px-1">
            <h2 class="text-base font-bold text-[#32221B]">Учасники команди</h2>
            <button class="text-sm font-bold text-[#32221B] flex items-center gap-1 hover:underline cursor-pointer">
                <span class="text-lg leading-none pb-0.5">+</span> Додати учасника
            </button>
        </div>
        
        <div class="flex flex-col sm:flex-row gap-8 w-full">
            <InputField header="Email *" type="email" placeholder="hello@example.com" class="flex-1" />
            <InputField header="Повне ім'я *" placeholder="ПІБ учасника" class="flex-1" />
        </div>
        
        <p class="text-[0.625rem] text-[#756157] mt-1 pl-1">Мінімум 2 учасники, максимум 6.</p>
    </Card>

    <Submit class="w-full mx-auto mt-4 h-12 flex justify-center items-center" title="Зареєструвати команду" />
</div>