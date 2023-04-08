import { stateType } from "../cell/mod.js";

export type modelType = {
    readonly value: Map<string, stateType>;
    readonly iteration: number;
    readonly position: {
        readonly x: number;
        readonly y: number;
    };
    readonly size: number;
};
