import { VNode } from "preact";

type props = {
    readonly id: string;
    readonly min: number;
    readonly max: number;
    readonly step: number;
    readonly value: number;
    readonly setValue: (newValue: number) => void;
};

export function RangeInput(
    { id, min, max, step, value, setValue }: props,
): VNode {
    function handleSetValue(newValue: number): void {
        if (newValue > max) return;
        if (newValue < min) return;
        setValue(newValue);
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
            onChange={(e) => {
                handleSetValue(Number(e.currentTarget.value));
            }}
        />
    );
}
