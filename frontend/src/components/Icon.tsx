import type { ReactElement } from "react";
import { Next } from "./Icons/Next";
import { Pause } from "./Icons/Pause";
import { Play } from "./Icons/Play";

type props = {
    readonly icon: "next" | "pause" | "play";
};

export function Icon({ icon }: props): ReactElement {
    switch (icon) {
        case "next":
            return <Next />;
        case "pause":
            return <Pause />;
        case "play":
            return <Play />;
    }
}
