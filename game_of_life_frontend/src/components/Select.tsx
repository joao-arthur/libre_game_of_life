import type { ReactElement } from "react";

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

export function Select({ id, groups, value, onChange }: Props): ReactElement {
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
                <optgroup label={group.label} key={group.value}>
                    {group.options.map((option) => (
                        <option
                            value={option.value}
                            key={option.value}
                        >
                            {option.label}
                        </option>
                    ))}
                </optgroup>
            ))}
        </select>
    );
}
