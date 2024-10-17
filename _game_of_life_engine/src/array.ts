export type ArrPos = {
    readonly row: number;
    readonly col: number;
};

export function arrPosFrom(row: number, col: number): ArrPos {
    return { row, col };
}
