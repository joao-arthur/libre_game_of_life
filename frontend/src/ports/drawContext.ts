import { squareType } from "../core/square";

export type drawContextType = {
    readonly clear: (
        dimension: squareType,
    ) => void;
    readonly drawSquare: (
        dimension: squareType,
        color: string,
    ) => void;
};
