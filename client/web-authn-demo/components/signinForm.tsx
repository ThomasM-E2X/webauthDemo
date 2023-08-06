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
      let challengeResponse = await fetch(
        "http://localhost:8080/generate_challenge",
        {
          method: "Get",
          headers: {
            "Access-Control-Allow-Methods": "*",
          },
        }
      );

      let { challenge, challenge_id } = await challengeResponse.json();

      let userId = new Uint8Array(16);

      window.crypto.getRandomValues(userId);

      const creds = (await window.navigator.credentials.get({
        publicKey: {
          challenge: new Uint8Array(challenge),
          allowCredentials: [],
          rpId: "localhost",
        },
      })) as unknown as Credential & { response: Record<string, any> };
      const textDecoder = new TextDecoder();

      const clientDataJson = JSON.parse(
        textDecoder.decode(creds.response.clientDataJSON)
      );

      const signature = textDecoder.decode(creds.response.signature);

      const authenticatorData = textDecoder.decode(
        creds.response.authenticatorData
      );

      const userHandle = textDecoder.decode(creds.response.userHandle);

      let verifyResponse = await fetch(
        `http://localhost:8080/verify_public_key/${challenge_id}`,
        {
          method: "POST",
          headers: {
            "Access-Control-Allow-Methods": "*",
            "Content-Type": "application/json",
          },
          body: JSON.stringify({
            clientDataJson,
            signature,
            authenticatorData,
            userHandle,
          }),
        }
      ); //send key to server for

      if (verifyResponse.status !== 200) {
        console.log("Server failed to authenticate passKey");
        return;
      }

      // authContext?.setIsAuth(true);
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
