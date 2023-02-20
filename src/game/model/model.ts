import { stateType } from "../cell/mod.ts";

export type modelType = {
    readonly width: number;
    readonly height: number;
    readonly value: readonly (readonly stateType[])[];
    readonly iteration: number;
};
