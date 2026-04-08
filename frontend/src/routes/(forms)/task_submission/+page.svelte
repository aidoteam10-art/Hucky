<script>
    import Card from "../../../components/Form/Card.svelte";
    import InputField from "../../../components/Form/InputField.svelte";
    import TextArea from "../../../components/Form/TextArea.svelte";
    import Submit from "../../../components/Form/Submit.svelte";
</script>

<Card class = "flex flex-col gap-5 !w-150">
    <div class = "flex justify-between">
        <h1 class = "font-bold text-[1.275rem]">Здати роботу</h1>
        <h3 class = "text-[0.75rem]">Часу залишилося: <b class = "ml-5">30:23:50:23</b></h3>


    </div>

    <InputField src_icon = "icons/github.svg" header = "GitHub Repository *" placeholder= "https://github.com/..."></InputField>
    <InputField src_icon = "icons/camera.svg" header = "Video Demo *" placeholder = "https://youtube.com/..."></InputField>
    <InputField src_icon = "icons/globe.svg" header = "Live Demo URL" placeholder = "https://..."></InputField>
    <TextArea src_icon = "icons/docs.svg" header = "Description" placeholder="What you've built, how to run it..."></TextArea>
    <Submit title = "Здати роботу"></Submit>
</Card>