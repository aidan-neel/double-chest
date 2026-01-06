import { Logo } from "@/components/core/Logo";
import { Button } from "@/components/ui/button";
import { Input } from "@/components/ui/input";
import { Label } from "@/components/ui/label";
import { useState } from "react";
import { Link } from "react-router-dom";
import { Spinner } from "@/components/ui/spinner";
import { motion } from "framer-motion";

export default function Register() {
    const [registering, setRegistering] = useState(false);

    return (
        <div className="h-screen w-screen flex-col gap-4 flex items-center justify-start pt-48">
            <Logo hasText={false} />
            <header className="flex flex-col items-center mb-6">
                <h1 className="font-medium text-xl">Register for an account</h1>
                <p className="text-muted-foreground mt-1">
                    Storage made simple, secure, and accessible.
                </p>
            </header>
            <div className="grid w-full max-w-sm items-center gap-3">
                <Label htmlFor="email">Email</Label>
                <Input type="email" id="email" placeholder="Email" />
            </div>
            <div className="grid w-full max-w-sm items-center gap-3 mt-1">
                <Label htmlFor="password">Password</Label>
                <Input type="password" id="password" placeholder="Password" />
            </div>
            <Button
                onClick={() => {
                    setRegistering(true);
                    setTimeout(() => {
                        setRegistering(false);
                    }, 3000);
                }}
                disabled={registering}
                className="w-full max-w-sm mt-1 transition-all duration-300"
            >
                {registering ? "Registering..." : "Register"}
                <motion.div
                    initial={{
                        opacity: 0,
                        marginRight: 4,
                        scale: 0.8,
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
                                  scale: 0.8,
                              }
                    }
                    transition={{ duration: 0.3, ease: [0.33, 1, 0.68, 1] }}
                >
                    <Spinner
                        className={`${registering ? "inline-block" : "hidden"}`}
                    />
                </motion.div>
            </Button>
            <Link to="/login" className="text-muted-foreground text-sm">
                Already have an account? Log in
            </Link>
        </div>
    );
}
