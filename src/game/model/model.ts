import { stateType } from "../state.ts";

export type modelType = {
    readonly width: number;
    readonly height: number;
    readonly values: readonly (readonly stateType[])[];
};
