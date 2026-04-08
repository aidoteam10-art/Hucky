<script>
	import Card from "../../../components/Form/Card.svelte";
	import InputField from "../../../components/Form/InputField.svelte";
	import Submit from "../../../components/Form/Submit.svelte";

</script>

<Card>
    <div class = "flex flex-col items-center mb-3">
        <img src="/icons/logo_black.png" alt="Hucky Logo" class="w-10 h-a">
        <h1 class = "mt-3 text-xl font-bold">Зареєструватися</h1>
    </div>

    <div class = "flex flex-col gap-3 mb-6">
        <InputField type="text" placeholder="Ваш ПІБ" header = "Ваше ім'я"></InputField>
        
        <InputField type = "text" placeholder = "hello@example.com" header = "Email"></InputField>
        
        <InputField type="password" placeholder="••••••••" header = "Пароль"></InputField>
    </div>

    <Submit title = "Зареєструватися"></Submit>
    
    <span class = "mt-5 flex justify-center">
        <span class = "mr-1 text-[0.625rem] text-[#756157]">Уже маєте акаунт? </span>
        <a href = /login class = "text-[0.625rem] font-semibold hover:underline">Увійти</a>
    </span>
</Card>