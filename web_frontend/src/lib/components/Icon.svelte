<script lang="ts">
    import IconNext from "./icon/IconNext.svelte";
    import IconPause from "./icon/IconPause.svelte";
    import IconPlay from "./icon/IconPlay.svelte";

    type Props = {
        readonly icon: "next" | "pause" | "play";
    };

    const { icon }: Props = $props();
</script>

{#if icon === "next"}
    <IconNext />
{/if}
{#if icon === "pause"}
    <IconPause />
{/if}
{#if icon === "play"}
    <IconPlay />
{/if}
