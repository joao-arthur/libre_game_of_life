import { VNode } from "preact";

type props = {
    label: string;
    onClick: () => void;
};

export function Button({ label, onClick }: props): VNode {
    return (
        <button className="bg-gray-500" onClick={onClick}>
            {label}
        </button>
    );
}
