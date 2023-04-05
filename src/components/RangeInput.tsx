import { VNode } from "preact";

type props = {
    readonly id: string;
    readonly min: number;
    readonly max: number;
    readonly step: number;
    readonly value: number;
    readonly onChange: (newValue: number) => void;
};

export function RangeInput(
    { id, min, max, step, value, onChange }: props,
): VNode {
    function handleOnChange(newValue: number): void {
        if (newValue > max) return;
        if (newValue < min) return;
        onChange(newValue);
    }

    return (
        <input
            type="range"
            id={id}
            name={id}
            min={String(min)}
            max={String(max)}
            step={String(step)}
            value={String(value)}
            onInput={(e) => {
                handleOnChange(Number(e.currentTarget.value));
            }}
        />
    );
}
