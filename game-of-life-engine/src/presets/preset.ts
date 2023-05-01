import { Model } from "../model/mod.js";

export type Preset = {
    readonly name: string;
    readonly id: string;
    readonly discover: {
        readonly name: string;
        readonly year: number;
    };
    readonly model: Model;
};
