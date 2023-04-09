export function absoluteToRelative(
    value: number,
    unitSize: number,
): number {
    return Math.trunc(value / unitSize);
}
