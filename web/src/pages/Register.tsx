import { Logo } from "@/components/core/Logo";
import { Button } from "@/components/ui/button";
import { Input } from "@/components/ui/input";
import { Label } from "@/components/ui/label";
import { useMemo, useState } from "react";
import { Link } from "react-router-dom";
import { Spinner } from "@/components/ui/spinner";
import { motion } from "framer-motion";
import { API_URL } from "@/lib/utils";
import type { AuthResponse } from "@/responses/auth";

export default function Register() {
    const [registering, setRegistering] = useState(false);
    const [error, setError] = useState<string | null>(null);

    const [email, setEmail] = useState("");
    const [password, setPassword] = useState("");
    const canContinue = useMemo(() => {
        const emailRegex = /^[^\s@]+@[^\s@]+\.[^\s@]+$/;
        return (
            email.length > 0 && password.length >= 8 && emailRegex.test(email)
        );
    }, [email, password]);

    const handleSubmit = async (e: React.FormEvent<HTMLFormElement>) => {
        e.preventDefault();

        setRegistering(true);

        try {
            const res = await fetch(`${API_URL}/auth/register`, {
                method: "POST",
                headers: {
                    "Content-Type": "application/json",
                },
                body: await JSON.stringify({
                    email: email,
                    display_name: email.split("@")[0],
                    password: password,
                }),
            });

            const data = (await res.json()) as AuthResponse;
        } catch (error: unknown) {
            if (error instanceof Error) {
                setError(error.message);
                setTimeout(() => {
                    setError(null);
                }, 2500);
            }
        } finally {
            setRegistering(false);
        }
    };

    return (
        <div
            className={`h-screen bg-background w-screen flex-col gap-4 flex items-center justify-start pt-48 ${
                error ? "" : ""
            }`}
        >
            <form
                onSubmit={handleSubmit}
                className={`flex flex-col relative p-4 py-12 pb-16 border rounded-lg transition-all duration-200 items-center gap-4 w-[450px] px-4 ${
                    error
                        ? "head-shake bg-red-50 border border-red-100"
                        : "border-transparent"
                }`}
            >
                {error && (
                    <p className="absolute bottom-[-1rem] bg-red-100 text-red-400 px-4 text-sm font-medium p-2 rounded-lg">
                        {error}
                    </p>
                )}
                <Logo hasText={false} />
                <header className="flex flex-col items-center mb-6">
                    <h1 className="font-medium text-xl">
                        Register for an account
                    </h1>
                    <p className="text-muted-foreground mt-1">
                        Storage made simple, secure, and accessible.
                    </p>
                </header>
                <div className="grid w-full max-w-sm items-center gap-3">
                    <Label
                        className={
                            registering
                                ? "opacity-70 hover:cursor-not-allowed pointer-events-none"
                                : ""
                        }
                        htmlFor="email"
                    >
                        Email
                    </Label>
                    <Input
                        onChange={(e) => setEmail(e.target.value)}
                        disabled={registering}
                        type="email"
                        id="email"
                        placeholder="Email"
                    />
                </div>
                <div className="grid w-full max-w-sm items-center gap-3 mt-1">
                    <Label
                        className={
                            registering
                                ? "opacity-70 hover:cursor-not-allowed pointer-events-none"
                                : ""
                        }
                        htmlFor="password"
                    >
                        Password
                    </Label>
                    <Input
                        onChange={(e) => setPassword(e.target.value)}
                        disabled={registering}
                        type="password"
                        id="password"
                        placeholder="Password"
                    />
                </div>
                <Button
                    type="submit"
                    disabled={registering || !canContinue}
                    className="w-full max-w-sm mt-1 transition-all duration-300"
                >
                    {registering ? "Registering..." : "Register"}
                    <motion.div
                        initial={{
                            opacity: 0,
                            marginRight: 4,
                            scale: 0,
                        }}
                        animate={
                            registering
                                ? {
                                      opacity: 1,
                                      marginRight: 12,
                                      scale: 1,
                                  }
                                : {
                                      opacity: 0,
                                      marginRight: 4,
                                      scale: 0,
                                  }
                        }
                        transition={{ duration: 0.3, ease: [0.33, 1, 0.68, 1] }}
                    >
                        <Spinner
                            className={`${
                                registering ? "inline-block" : "hidden"
                            }`}
                        />
                    </motion.div>
                </Button>
                <Link to="/login" className="text-muted-foreground text-sm">
                    Already have an account? Log in
                </Link>
            </form>
        </div>
    );
}
