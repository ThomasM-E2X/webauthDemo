"use client";

import React, { FormEvent, useEffect, useState } from "react";

interface FormElements extends HTMLFormControlsCollection {
  firstName: HTMLInputElement;
  lastName: HTMLInputElement;
}
interface CreateForm extends HTMLFormElement {
  readonly elements: FormElements;
}

type Props = {};

function SigninForm({}: Props) {
  useEffect(() => {
    const requestChallenge = async () => {
      let challengeBytes = await fetch(
        "http://localhost:8080/generate_challenge",
        {
          method: "Get",
          headers: {
            "Access-Control-Allow-Methods": "*",
          },
        }
      );

      const bytes = await challengeBytes.arrayBuffer();
      let userId = new Uint8Array(16);

      window.crypto.getRandomValues(userId);

      const creds = await window.navigator.credentials.get({
        publicKey: {
          challenge: bytes,
          allowCredentials: [],
          rpId: "localhost",
        },
      });

      const json = JSON.parse(
        new TextDecoder().decode(creds.response.clientDataJSON)
      );

      let verifyResponse = await fetch(
        "http://localhost:8080/verify_public_key",
        {
          method: "POST",
          headers: {
            "Access-Control-Allow-Methods": "*",
            "Content-Type": "application/json",
          },
          body: JSON.stringify(creds),
        }
      ); //send key to server for

      const res2 = verifyResponse;
    };

    requestChallenge();
  }, []);

  async function handleSubmit(e: FormEvent<CreateForm>) {
    e.preventDefault();
    console.log("erererer");
  }

  return (
    <form onSubmit={handleSubmit} className="flex flex-col space-y-5">
      <div>
        <label htmlFor="firstName">FirstName</label>
        <input
          autoComplete="firstName webauthn"
          className="pl-5 py-5"
          type="text"
          name="firstName"
          placeholder="John"
        />
      </div>
      <div>
        <label htmlFor="lastName">LastName</label>
        <input
          autoComplete="lastName webauthn"
          className="pl-5 py-5"
          type="text"
          name="lastName"
          placeholder="Doe"
        />
      </div>
      <div>
        <button
          type="submit"
          className="bg-purple-500 w-full rounded-md py-5 hover:bg-purple-900 hover:text-white"
        >
          Login
        </button>
      </div>
    </form>
  );
}

export default SigninForm;
