<script>
	import Card from "../../../components/Form/Card.svelte";
    import InputField from "../../../components/Form/InputField.svelte";
	import Submit from "../../../components/Form/Submit.svelte";
</script>
<Card>

    <div class = "flex flex-col items-center mb-3">
        <img src="/icons/logo_black.png" alt="Hucky Logo" class="w-10 h-a mb-3 ">
        <h1 class = "text-xl font-bold">Увійти</h1>
    </div>


    <div class = "mb-6 flex flex-col gap-3"> 
        <InputField header = "Email" type="text" placeholder="hello@example.com"></InputField>
        <InputField header = "Пароль" type="password" placeholder="••••••••"></InputField>
    </div>
    

    <Submit title = "Увійти"></Submit>
    
    <span class = "mt-5 flex justify-center">
        <span class = "mr-1 text-[0.625rem] text-[#756157] ">Ще не маєте акаунту?</span>
        <a href = /registration class = "text-[0.625rem] font-semibold hover:underline">Зареєструватися</a>
    </span>

</Card>
