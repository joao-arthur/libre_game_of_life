import { stateType } from "../cell/mod.ts";

export type modelType = {
    readonly size: number;
    readonly value: readonly (readonly stateType[])[];
    readonly iteration: number;
};
