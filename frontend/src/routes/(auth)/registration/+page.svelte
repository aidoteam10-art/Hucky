<script>
	import Card from "../../../components/Form/Card.svelte";
	import InputField from "../../../components/Form/InputField.svelte";
	import Submit from "../../../components/Form/Submit.svelte";

</script>
<div class = " max-w-lg w-full mx-auto p-6">
    <Card class = "flex w-full flex-col gap-3">
        <div class = "flex flex-col items-center">
            <img src="/icons/logo_black.png" alt="Hucky Logo" class="w-10 h-auto">
            <h1 class = "mt-3 text-2xl font-extrabold">Зареєструватися</h1>
        </div>

            <InputField type="text" placeholder="Ваш ПІБ" header = "Ваше ім'я" />
            
            <InputField type="email" placeholder="hello@example.com" header="Email" />
            
            <InputField class = "mb-3" type="password" placeholder="••••••••" header="Пароль" />


        <Submit class = "mb-3" title = "Зареєструватися" />
        
        <span class = "flex justify-center">
            <span class = "mr-1 text-[0.625rem] text-[#756157]">Уже маєте акаунт? </span>
            <a href = /login class = "text-[0.625rem] font-semibold hover:underline">Увійти</a>
        </span>
    </Card>
</div>