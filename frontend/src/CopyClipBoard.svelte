<script>
    import { onMount } from 'svelte';

    export let text;

    let textarea;

    onMount(() => {
        textarea.select();
        document.execCommand('copy');
    });
</script>

<textarea bind:value={text} bind:this={textarea}></textarea>

