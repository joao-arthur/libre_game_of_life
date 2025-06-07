<script lang="ts">
    import Icon from "./Icon.svelte";

    type Props = {
        readonly label: string;
        readonly onClick: () => void;
        readonly icon: "next" | "pause" | "play";
    };

    const { label, onClick, icon }: Props = $props();
</script>

<style>
    button {
        background-color: rgb(99 102 241);
        color: white;
        margin-top: 0.5rem;
        margin-bottom: 0.5rem;
        display: flex;
        justify-content: center;
        gap: 0.25rem;
    }
</style>

<button onclick={onClick}>
    <Icon {icon} /> {label}
</button>
