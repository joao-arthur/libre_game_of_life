import { stateType } from "../../cell/mod.js";
import { modelType } from "../model.js";

export function fromString(stringValue: string[]): modelType {
    const value = stringValue.map((line) =>
        line.split("").map((character: string): stateType | undefined=> {
            switch (character) {
                case "⬛":
                    return stateType.DEAD;
                case "⬜":
                    return stateType.ALIVE;
                default:
                    return undefined;
            }
        }).filter((maybeState) => maybeState !== undefined)
            .map((state) => state as stateType),
    );

    return {
        size: value[0].length,
        value,
        iteration: 0,
    };
}
