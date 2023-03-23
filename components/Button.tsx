import { VNode } from "preact";

type props = {
    readonly label: string;
    readonly onClick: () => void;
};

export function Button({ label, onClick }: props): VNode {
    return (
        <button className="bg-gray-500 my-1" onClick={onClick}>
            {label}
        </button>
    );
}
