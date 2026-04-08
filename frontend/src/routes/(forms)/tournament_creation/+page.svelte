<script>
    import Card from "../../../components/Form/Card.svelte";
    import InputField from "../../../components/Form/InputField.svelte";
    import TextArea from "../../../components/Form/TextArea.svelte";
    import DateField from "../../../components/Form/DateField.svelte";
    import Submit from "../../../components/Form/Submit.svelte";
</script>

<div class="flex flex-col gap-6 max-w-4xl w-full mt-10 p-10">
    <div class="flex justify-between items-start mb-2 px-2">
        <div class="flex flex-col gap-1">
            <h1 class="text-[1.75rem] font-bold text-[#1F1F1F]">Створіть турнір</h1>
            <h2 class="text-sm font-bold text-[#1F1F1F]">Деталі турніру</h2>
        </div>
        <button class="flex items-center gap-2 border border-[#756157] rounded-lg px-4 py-2 bg-transparent text-sm font-semibold text-[#1F1F1F] hover:bg-black/5 transition-colors cursor-pointer">
            <div class="w-5 h-5 rounded-full bg-[#CCFF00] flex items-center justify-center font-bold text-lg leading-none pt-0.5 text-black">+</div>
            Зберегти чернетку
        </button>
    </div>

    <Card class="flex flex-col gap-5 w-full !w-[100%] shadow-sm">
        <h2 class="text-base font-bold text-[#32221B]">Деталі турніру</h2>
        <InputField header="Назва*" placeholder="e.g. Spring Hackathon" class="w-full" />
        <TextArea header="Опис / Правила" placeholder="Правила турніру, формат, цілі..." rows={4} class="w-full" />
    </Card>

    <Card class="flex flex-col gap-5 w-full !w-[100%] shadow-sm">
        <h2 class="text-base font-bold text-[#32221B]">Реєстрація і розклад</h2>
        <div class="flex flex-col sm:flex-row gap-8 w-full">
            <DateField header="Початок реєстрації *" class="flex-1" />
            <DateField header="Завершення реєстрації *" class="flex-1" />
        </div>
        <div class="flex flex-col sm:flex-row gap-8 w-full">
            <DateField header="Початок турніру*" class="flex-1" />
            <InputField header="Максимальна кількість команд *" placeholder="Enter number" class="flex-1" />
        </div>
    </Card>

    <Card class="flex flex-col gap-5 w-full !w-[100%] shadow-sm">
        <h2 class="text-base font-bold text-[#32221B]">Перший раунд</h2>
        <InputField header="Назва раунду*" placeholder="e.g. Round 1: MVP Development" class="w-full" />
        <TextArea header="Опис завдання *" placeholder="Що команди мають зробити..." rows={4} class="w-full" />
        <TextArea header="Необхідні технології" placeholder="Необхідний стек..." rows={2} class="w-full" />

        <div class="flex flex-col gap-3 mt-2 w-full">
            <div class="flex justify-between items-center px-1">
                <h3 class="text-sm font-bold text-[#32221B]">Обов'язкові вимоги</h3>
                <button class="text-sm font-bold text-[#32221B] flex items-center gap-1 hover:underline cursor-pointer">
                    <span class="text-lg leading-none">+</span> Додати
                </button>
            </div>
            <InputField placeholder="Вимога 1" header="" class="w-full" />
        </div>

        <div class="flex flex-col sm:flex-row gap-8 mt-2 w-full">
            <DateField header="Початок раунду *" class="flex-1" />
            <DateField header="Дедлайн *" class="flex-1" />
        </div>
    </Card>

    <Submit class="w-full mt-4 h-12 flex justify-center items-center" title="Створити турнір" />
</div>