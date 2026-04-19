<script>
    import Card from "/src/components/form/Card.svelte";
    import InputField from "/src/components/form/InputField.svelte";
    import TextArea from "/src/components/form/TextArea.svelte";
    import Submit from "/src/components/form/Submit.svelte";

    let timeLeft = "30:23:50:23";
</script>

<div class = "p-6 max-w-3xl w-full mx-auto mt-10">
    <div class = "flex flex-col items-center gap-1 sm:justify-between sm:flex-row mb-5">
        <h1 class = "font-bold text-[1.75rem]">Здати роботу</h1>
        <h3 class = "text-[0.8rem]">Часу залишилося: <b class ="ml-5">{timeLeft}</b></h3>
    </div>

    <Card class = "flex flex-col gap-5 w-full mb-4">
        <InputField src_icon="/icons/github.svg" header="GitHub Repository *" placeholder="https://github.com/..." />
        <InputField src_icon="/icons/camera.svg" header="Video Demo *" placeholder="https://youtube.com/..." />
        <InputField src_icon="/icons/globe.svg" header="Live Demo URL" placeholder="https://..." />
        <TextArea src_icon="/icons/docs.svg" header="Description" placeholder="What you've built, how to run it..." />
    </Card>

    <Submit title="Здати роботу" class = "w-full"/>
</div>
