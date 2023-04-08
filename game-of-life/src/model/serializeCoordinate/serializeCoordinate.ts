type coordinateType = {
    readonly x: number;
    readonly y: number;
};

export function serializeCoordinate({ x, y }: coordinateType) {
    return `(x: ${x}, y: ${y})` as const;
}
