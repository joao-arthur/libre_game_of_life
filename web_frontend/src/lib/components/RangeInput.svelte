<script lang="ts">
    type Props = {
        readonly id: string;
        readonly min: number;
        readonly max: number;
        readonly step: number;
        readonly value: number;
        readonly onChange: (newValue: number) => void;
    };

    const { id, min, max, step, value, onChange }: Props = $props();

    function handleOnChange(newValue: number): void {
        if (newValue > max) return;
        if (newValue < min) return;
        onChange(newValue);
    }
</script>

<style>
    input {
        accent-color: #6366f1;
    }
</style>

<input
    type="range"
    {id}
    name={id}
    min={String(min)}
    max={String(max)}
    step={String(step)}
    value={String(value)}
    oninput={(e) => {
        let newValue = Number(e.currentTarget.value);
        if (newValue != value) {
            handleOnChange(newValue);
        }
    }}
/>
