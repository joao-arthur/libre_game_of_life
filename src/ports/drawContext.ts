import { squareType } from "../core/square.ts";

export type drawContextType = {
    readonly drawSquare: (
        dimension: squareType,
        color: string,
    ) => void;
};
