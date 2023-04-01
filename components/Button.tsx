import { VNode } from "preact";
import { Icon } from "./Icon.tsx";

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
            <Icon icon={icon} />
            {label}
        </button>
    );
}
