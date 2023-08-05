"use client";

import { useContext } from "react";
import { AuthContext } from "./context/authContext";
import Link from "next/link";

export default function Home() {
  const authContext = useContext(AuthContext);

  if (!authContext || !authContext.isAuth) {
    return (
      <main className="flex min-h-screen flex-col items-center p-24">
        <h1>You are not logged in</h1>
        <div className="flex flex-col space-y-5 pt-5">
          <Link href="/create">
            <button className="bg-purple-500 w-full rounded-md py-5 hover:bg-purple-900 hover:text-white">
              Create new user
            </button>
          </Link>
          <Link href="/login">
            <button className="bg-purple-500 w-full rounded-md py-5 hover:bg-purple-900 hover:text-white">
              Login
            </button>
          </Link>
        </div>
      </main>
    );
  }

  return (
    <main className="flex min-h-screen flex-col items-center justify-between p-24">
      <h1>You are logged in, welcome!</h1>
    </main>
  );
}
