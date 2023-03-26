import { VNode } from "preact";
import { Next } from "./Icons/Next.tsx";
import { Pause } from "./Icons/Pause.tsx";
import { Play } from "./Icons/Play.tsx";

type props = {
    readonly label: string;
    readonly onClick: () => void;
    readonly icon: "next" | "pause" | "play";
};

export function Button({ label, onClick, icon }: props): VNode {
    return (
        <button
            className="bg-indigo-500 text-white shadow my-2 rounded flex justify-center gap-1"
            onClick={onClick}
        >
            {icon === "next" ? <Next /> : null}
            {icon === "pause" ? <Pause /> : null}
            {icon === "play" ? <Play /> : null}
            {label}
        </button>
    );
}
