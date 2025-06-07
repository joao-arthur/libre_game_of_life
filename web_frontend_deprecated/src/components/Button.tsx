import type { ReactElement } from "react";
import { Icon } from "./Icon";

type Props = {
    readonly label: string;
    readonly onClick: () => void;
    readonly icon: "next" | "pause" | "play";
};

export function Button({ label, onClick, icon }: Props): ReactElement {
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
