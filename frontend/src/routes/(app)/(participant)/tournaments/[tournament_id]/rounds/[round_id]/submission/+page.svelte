<script>
    import Card from "/src/components/form/Card.svelte";
    import InputField from "/src/components/form/InputField.svelte";
    import TextArea from "/src/components/form/TextArea.svelte";
    import Submit from "/src/components/form/Submit.svelte";

    let timeLeft = "30:23:50:23";

    let githubUrl = "";
    let videoUrl = "";
    let liveUrl = "";
    let description = "";

    let errors = {};

    function validate() {
        errors = {};
        let valid = true;

        if (!githubUrl) {
            errors.githubUrl = "Репозиторій обов'язковий";
            valid = false;
        } else if (!/^https?:\/\/(www\.)?github\.com\//.test(githubUrl)) {
            errors.githubUrl = "Має бути коректним посиланням на GitHub";
            valid = false;
        }

        if (!videoUrl) {
            errors.videoUrl = "Відео демо обов'язкове";
            valid = false;
        } else if (!/^https?:\/\/.+/.test(videoUrl)) {
            errors.videoUrl = "Має бути коректним посиланням";
            valid = false;
        }

        if (liveUrl && !/^https?:\/\/.+/.test(liveUrl)) {
            errors.liveUrl = "Має бути коректним посиланням";
            valid = false;
        }

        if (description && description.length > 2000) {
            errors.description = "Максимум 2000 символів";
            valid = false;
        }

        return valid;
    }

    function handleSubmit() {
        if (validate()) {
            console.log("Valid submission", { githubUrl, videoUrl, liveUrl, description });
        }
    }
</script>

<div class = "p-6 max-w-3xl w-full mx-auto mt-10">
    <div class = "flex flex-col items-center gap-1 sm:justify-between sm:flex-row mb-5">
        <h1 class = "font-bold text-[1.75rem]">Здати роботу</h1>
        <h3 class = "text-[0.8rem]">Часу залишилося: <b class ="ml-5">{timeLeft}</b></h3>
    </div>

    <form on:submit|preventDefault={handleSubmit} class="w-full">
        <Card class = "flex flex-col gap-5 w-full mb-4">
            <InputField bind:value={githubUrl} error={errors.githubUrl} src_icon="/icons/github.svg" header="GitHub Repository *" placeholder="https://github.com/..." />
            <InputField bind:value={videoUrl} error={errors.videoUrl} src_icon="/icons/camera.svg" header="Video Demo *" placeholder="https://youtube.com/..." />
            <InputField bind:value={liveUrl} error={errors.liveUrl} src_icon="/icons/globe.svg" header="Live Demo URL" placeholder="https://..." />
            <TextArea bind:value={description} error={errors.description} src_icon="/icons/docs.svg" header="Description" placeholder="What you've built, how to run it..." />
        </Card>

        <Submit title="Здати роботу" class = "w-full"/>
    </form>
</div>
