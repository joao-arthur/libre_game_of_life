import { VNode } from "preact";
import { Next } from "./Icons/Next.tsx";
import { Pause } from "./Icons/Pause.tsx";
import { Play } from "./Icons/Play.tsx";

type props = {
    readonly icon: "next" | "pause" | "play";
};

export function Icon({ icon }: props): VNode {
    switch (icon) {
        case "next":
            return <Next />;
        case "pause":
            return <Pause />;
        case "play":
            return <Play />;
    }
}
