type coordinateType = {
    readonly x: number;
    readonly y: number;
};

export function serializeCoordinate(
    { x, y }: coordinateType,
): string {
    return `(x: ${x}, y: ${y})`;
}
