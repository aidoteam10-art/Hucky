<script>
    import { enhance } from '$app/forms'
    import Card from "/src/components/form/Card.svelte";
    import InputField from "/src/components/form/InputField.svelte";
    import Submit from "/src/components/form/Submit.svelte";

    let email = "";
    let password = "";
    let emailError = "";
    let passwordError = "";

    function validate() {
        let valid = true;
        emailError = "";
        passwordError = "";

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
        }

        return valid;
    }

    function handleSubmit() {
        if (validate()) {
            // handle login
            console.log("Login valid", { email, password });
        }
    }

    export let form;
</script>
<div class = "flex w-full justify-center p-6">
    <Card class = "flex flex-col gap-3 w-full max-w-sm">

        <div class = "flex flex-col items-center">
            <img src="/icons/logo_black.png" alt="Hucky Logo" class="w-10 h-auto mb-3 ">
            <h1 class = "text-2xl font-extrabold">Увійти</h1>
        </div>

        <form method="POST" use:enhance class="flex flex-col gap-3 w-full">
            <InputField name="email" value={form?.email ?? ''} header="Email" type="email" placeholder="hello@example.com" required/>
            <InputField name = "password" header="Пароль" type="password" placeholder="••••••••" required/>

            <Submit class="mb-2 mt-2" title="Увійти" />
        </form>
        
        <span class = "flex justify-center">
            <span class = "mr-1 text-[0.625rem] text-[#756157] ">Ще не маєте акаунту?</span>
            <a href = "/registration" class = "text-[0.625rem] font-semibold hover:underline">Зареєструватися</a>
        </span>

        {#if form?.message}
            <div class="p-2 mb-2 text-sm text-red-600 bg-red-100 rounded text-center">
                {form.message}
            </div>
        {/if}

    </Card>
</div>
