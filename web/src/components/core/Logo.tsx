import { Cuboid } from "lucide-react";

type LogoProps = {
    hasText?: boolean;
};

export function Logo(props: LogoProps) {
    return (
        <div className="flex flex-row items-center justify-center gap-0 relative w-20 h-20 ">
            <div className="w-12 h-12 bg-linear-to-br blur-[0.2px] from-neutral-800 absolute top-0 right-0 to-neutral-700 rounded-full border border-neutral-900 opacity-30"></div>
            <div className="w-12 h-12 bg-linear-to-br blur-[0.2px] from-neutral-900 absolute top-5 right-3 to-neutral-800 rounded-full border border-neutral-950 opacity-50"></div>
        </div>
    );
}
