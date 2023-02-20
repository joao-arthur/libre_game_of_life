import { stateType } from "../../cell/mod.ts";
import { modelType } from "../model.ts";

export function fromString(stringValue: string[]): modelType {
    const width = stringValue[0].length;
    const height = stringValue.length;

    const value = stringValue.map((line) =>
        line.split("").map((character: string): stateType => {
            switch (character) {
                case "⬛":
                    return stateType.DEAD;
                case "⬜":
                    return stateType.ALIVE;
                default:
                    return undefined!;
            }
        })
    );

    return {
        width,
        height,
        value,
        iteration: 0,
    };
}
