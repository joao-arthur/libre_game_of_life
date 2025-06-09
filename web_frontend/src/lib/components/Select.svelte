<script lang="ts">
    type Option = {
        readonly value: string;
        readonly label: string;
    };

    type Group = {
        readonly label: string;
        readonly value: string;
        readonly options: Option[];
    };

    type Props = {
        readonly id: string;
        readonly groups: readonly Group[];
        readonly value: string;
        readonly onChange: (newValue: string) => void;
    };

    const { id, groups, value, onChange }: Props = $props();

    function handleOnChange(newValue: string): void {
        onChange(newValue);
    }
</script>

<style>
    select {
        border-radius: 0.25rem;
        padding-left: 0.5rem;
        padding-right: 0.5rem;
        padding-top: 0.25rem;
        padding-bottom: 0.25rem;
        border: none;
        font-size: 1rem;
    }
</style>

<select
    {value}
    onchange={(e) => handleOnChange(e.currentTarget.value)}
    {id}
    name={id}
>
    <option value={undefined}></option>
    {#each groups as group}
        <optgroup label={group.label}>
            {#each group.options as option}
                <option value={option.value}>
                    {option.label}
                </option>
            {/each}
        </optgroup>
    {/each}
</select>
