"use client";

import React, { useState } from "react";

type Props = {
  children: React.ReactNode;
};

export const AuthContext = React.createContext<{
  isAuth: boolean;
  setIsAuth: React.Dispatch<React.SetStateAction<boolean>>;
} | null>(null);

function AuthContextWrapper({ children }: Props) {
  const [isAuth, setIsAuth] = useState(false);

  return (
    <AuthContext.Provider value={{ isAuth, setIsAuth }}>
      {children}
    </AuthContext.Provider>
  );
}

export default AuthContextWrapper;
