<script>
    import Card from "/src/components/form/Card.svelte";
    import InputField from "/src/components/form/InputField.svelte";
    import Submit from "/src/components/form/Submit.svelte";

    let name = "";
    let email = "";
    let password = "";
    let repeatPassword = "";

    let nameError = "";
    let emailError = "";
    let passwordError = "";
    let repeatPasswordError = "";

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

    function validate() {
        let valid = true;
        nameError = "";
        emailError = "";
        passwordError = "";
        repeatPasswordError = "";

        if (!name) {
            nameError = "ПІБ обов'язковий";
            valid = false;
        }
        
        if (!email) {
            emailError = "Email обов'язковий";
            valid = false;
        } else if (!/^[^\s@]+@[^\s@]+\.[^\s@]+$/.test(email)) {
            emailError = "Невірний формат email";
            valid = false;
        }

        if (!password) {
            passwordError = "Пароль обов'язковий";
            valid = false;
        } else if (passwordStrength < 3) {
            passwordError = "Пароль занадто слабкий (мінімум 8 символів, 1 велика літера, 1 цифра)";
            valid = false;
        }

        if (password !== repeatPassword) {
            repeatPasswordError = "Паролі не співпадають";
            valid = false;
        }

        return valid;
    }

    function handleSubmit() {
        if (validate()) {
            console.log("Registration valid", { name, email, password });
        }
    }
</script>
<div class="flex w-full justify-center p-6">
    <Card class="flex flex-col gap-3 w-full max-w-sm">
        <div class="flex flex-col items-center">
            <img src="/icons/logo_black.png" alt="Hucky Logo" class="w-10 h-auto">
            <h1 class="mt-3 text-2xl font-extrabold">Зареєструватися</h1>
        </div>

        <form on:submit|preventDefault={handleSubmit} class="flex flex-col gap-3 w-full">
            <InputField bind:value={name} error={nameError} type="text" placeholder="Ваш ПІБ" header="Ваше ім'я *" />
            
            <InputField bind:value={email} error={emailError} type="email" placeholder="hello@example.com" header="Email *" />
            
            <div class="flex flex-col gap-1 w-full relative">
                <InputField bind:value={password} error={passwordError} type="password" placeholder="••••••••" header="Пароль *" />
                {#if password.length > 0}
                    <div class="flex items-center gap-2 mt-1 px-1">
                        <div class="flex-1 h-1.5 rounded-full bg-gray-200 overflow-hidden">
                            <div class="h-full {passwordStrengthColor} transition-all duration-300" style="width: {(passwordStrength / 4) * 100}%"></div>
                        </div>
                        <span class="text-[0.625rem] text-[#756157] w-12 text-right">{passwordStrengthLabel}</span>
                    </div>
                {/if}
            </div>

            <InputField bind:value={repeatPassword} error={repeatPasswordError} success={repeatPassword.length > 0 && password === repeatPassword} class="mb-3" type="password" placeholder="••••••••" header="Повторіть пароль *" />

            <Submit class="mb-3" title="Зареєструватися" />
        </form>
        
        <span class="flex justify-center">
            <span class="mr-1 text-[0.625rem] text-[#756157]">Уже маєте акаунт? </span>
            <a href="/login" class="text-[0.625rem] font-semibold hover:underline">Увійти</a>
        </span>
    </Card>
</div>