<script>
    import "../../app.css"
    import { page } from '$app/stores';
</script>
<div class = "min-h-screen w-full bg-[#F9FFE5] flex flex-col items-center justify-center ">{#key $page.url.pathname}<slot></slot>{/key}</div>

