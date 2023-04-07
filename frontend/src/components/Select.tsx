import type { ReactElement } from "react";

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
): ReactElement {
    function handleOnChange(newValue: string): void {
        onChange(newValue);
    }

    return (
        <select
            className="rounded px-2 py-1 my-1"
            value={value}
            onChange={(e) => handleOnChange(e.currentTarget.value)}
            id={id}
            name={id}
        >
            <option value={undefined}></option>
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
