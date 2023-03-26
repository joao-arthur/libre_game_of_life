export function absoluteToRelative(
    absoluteValue: number,
    unitSize: number,
): number {
    return Math.trunc(absoluteValue / unitSize);
}
