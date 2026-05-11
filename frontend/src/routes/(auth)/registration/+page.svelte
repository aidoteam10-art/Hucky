<script>
    import { enhance } from '$app/forms';
    import Card from "/src/components/form/Card.svelte";
    import InputField from "/src/components/form/InputField.svelte";
    import Submit from "/src/components/form/Submit.svelte";

    export let form;

    let password = "";
    let repeatPassword = "";

    $: passwordStrength = calculateStrength(password);
    $: passwordStrengthColor = getStrengthColor(passwordStrength);
    $: passwordStrengthLabel = getStrengthLabel(passwordStrength);

    function calculateStrength(pass) {
        if (!pass) return 0;
        let score = 0;
        if (pass.length >= 8) score += 1;
        if (/[A-Z]/.test(pass)) score += 1;
        if (/[0-9]/.test(pass)) score += 1;
        if (/[^A-Za-z0-9]/.test(pass)) score += 1;
        return score;
    }

    function getStrengthColor(score) {
        if (score === 0) return "bg-gray-200";
        if (score <= 2) return "bg-red-500";
        if (score === 3) return "bg-yellow-500";
        return "bg-green-500";
    }

    function getStrengthLabel(score) {
        if (score === 0) return "";
        if (score <= 2) return "Слабкий";
        if (score === 3) return "Середній";
        return "Надійний";
    }
</script>
<div class="flex w-full justify-center p-6">
    <Card class="flex flex-col gap-3 w-full max-w-sm">
        <div class="flex flex-col items-center">
            <img src="/icons/logo_black.png" alt="Hucky Logo" class="w-10 h-auto">
            <h1 class="mt-3 text-2xl font-extrabold">Зареєструватися</h1>
        </div>

        {#if form?.message}
            <div class="p-2 mb-2 text-sm text-red-600 bg-red-100 rounded text-center">
                {form.message}
            </div>
        {/if}

        <form method="POST" use:enhance class="flex flex-col gap-3 w-full">
            <InputField name="name" value={form?.full_name ?? ''} type="text" placeholder="Ваш ПІБ" header="Ваше ім'я *" required />
            
            <InputField name="email" value={form?.email ?? ''} type="email" placeholder="hello@example.com" header="Email *" required />
            
            <div class="flex flex-col gap-1 w-full relative">
                <InputField name="password" bind:value={password} type="password" placeholder="••••••••" header="Пароль *" required />
                {#if password.length > 0}
                    <div class="flex items-center gap-2 mt-1 px-1">
                        <div class="flex-1 h-1.5 rounded-full bg-gray-200 overflow-hidden">
                            <div class="h-full {passwordStrengthColor} transition-all duration-300" style="width: {(passwordStrength / 4) * 100}%"></div>
                        </div>
                        <span class="text-[0.625rem] text-[#756157] w-12 text-right">{passwordStrengthLabel}</span>
                    </div>
                {/if}
            </div>

            <InputField name="repeatPassword" bind:value={repeatPassword} success={repeatPassword.length > 0 && password === repeatPassword} class="mb-3" type="password" placeholder="••••••••" header="Повторіть пароль *" required />

            <Submit class="mb-3" title="Зареєструватися" />

            <p class="text-center text-[0.625rem] leading-4 text-[#756157]">
                Реєструючись, ви погоджуєтеся з
                <a href="/terms" class="font-semibold text-[#191F00] hover:underline">Умовами використання</a>
                та підтверджуєте, що ознайомилися з
                <a href="/privacy-policy" class="font-semibold text-[#191F00] hover:underline">Політикою конфіденційності</a>
                Hucky.
            </p>
        </form>
        
        <span class="flex justify-center">
            <span class="mr-1 text-[0.625rem] text-[#756157]">Уже маєте акаунт? </span>
            <a href="/login" class="text-[0.625rem] font-semibold hover:underline">Увійти</a>
        </span>
    </Card>
</div>
