import { squareType } from "../core/square.ts";

export type drawContextType = {
    readonly clear: (
        dimension: squareType,
    ) => void;
    readonly drawSquare: (
        dimension: squareType,
        color: string,
    ) => void;
};
