import { VNode } from "preact";

type optionType = {
    readonly value: string;
    readonly label: string;
};

type groupType = {
    readonly label: string;
    readonly options: optionType[];
};

type props = {
    readonly id: string;
    readonly groups: readonly groupType[];
    readonly value: string;
    readonly onChange: (newValue: string) => void;
};

export function Select(
    { id, groups, value, onChange }: props,
): VNode {
    function handleOnChange(newValue: string): void {
        onChange(newValue);
    }

    return (
        <select
            value={value}
            onChange={(e) => handleOnChange(e.currentTarget.value)}
            id={id}
            name={id}
        >
            {groups.map((group) => (
                <optgroup label={group.label}>
                    {group.options.map((option) => (
                        <option value={option.value}>
                            {option.label}
                        </option>
                    ))}
                </optgroup>
            ))}
        </select>
    );
}
