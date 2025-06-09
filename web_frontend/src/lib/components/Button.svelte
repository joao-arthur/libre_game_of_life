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
        display: flex;
        gap: 0.25rem;
        border: none;
        cursor: pointer;
        width: 100%;
        justify-content: center;
        align-items: center;
        font-size: 1rem;
        border-radius: 5px;
    }
</style>

<button onclick={onClick}>
    <Icon {icon} /> {label}
</button>
