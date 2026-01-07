type Token = {
    access_token: string;
    refresh_token: string;
};

export type AuthResponse = {
    message: string;
    token: Token;
};
