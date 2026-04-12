<script>
	import Card from "../../../components/Form/Card.svelte";
    import InputField from "../../../components/Form/InputField.svelte";
	import Submit from "../../../components/Form/Submit.svelte";
</script>
<div class = "flex max-w-lg w-full p-6">
    <Card class = "flex flex-col gap-3 w-full">

        <div class = "flex flex-col items-center">
            <img src="/icons/logo_black.png" alt="Hucky Logo" class="w-10 h-auto mb-3 ">
            <h1 class = "text-2xl font-extrabold">Увійти</h1>
        </div>

            <InputField header = "Email" type="email" placeholder="hello@example.com" />
            <InputField header = "Пароль" type="password" placeholder="••••••••" />

        

        <Submit class = "mb-2" title = "Увійти" />
        
        <span class = "flex justify-center">
            <span class = "mr-1 text-[0.625rem] text-[#756157] ">Ще не маєте акаунту?</span>
            <a href = "/registration" class = "text-[0.625rem] font-semibold hover:underline">Зареєструватися</a>
        </span>

    </Card>
</div>
