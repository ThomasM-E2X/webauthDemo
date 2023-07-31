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
  const [canUseAuthn, setCanUseAuthn] = useState(false);

  useEffect(() => {
    if (
      window.PublicKeyCredential &&
      PublicKeyCredential.isUserVerifyingPlatformAuthenticatorAvailable &&
      PublicKeyCredential.isConditionalMediationAvailable
    ) {
      Promise.all([
        PublicKeyCredential.isUserVerifyingPlatformAuthenticatorAvailable(),
        PublicKeyCredential.isConditionalMediationAvailable(),
      ]).then((results) => {
        if (results.every((r) => r === true)) {
          setCanUseAuthn(true);
        }
      });
    }
  }, []);

  async function handleSubmit(e: FormEvent<CreateForm>) {
    e.preventDefault();
    const { firstName, lastName } = e.currentTarget.elements;

    console.log(canUseAuthn);

    let challengeBytes = await fetch("localhost:8080/generate_challenge", {
      method: "Get",
    });

    console.log("bytes", challengeBytes);

    // if (canUseAuthn) {
    //generateChallenge
    let credential = await navigator.credentials.create({
      publicKey: {
        challenge: new Uint8Array([]),
        rp: {
          id: "4a4f-82-30-114-57.ngrok-free.app",
          name: "localHost",
        },
        user: {
          id: new Uint8Array([79, 252, 83, 72, 214, 7, 89, 26]),
          name: `${firstName.value}${lastName.value}`,
          displayName: `${firstName.value} ${lastName.value} `,
        },
        pubKeyCredParams: [{ type: "public-key", alg: -7 }],
        authenticatorSelection: {
          authenticatorAttachment: "platform",
          requireResidentKey: true,
        },
      },
    });

    //get users consent

    console.log(credential);
    //post public key to user
    // }
  }

  return (
    <form onSubmit={handleSubmit} className="flex flex-col space-y-5">
      <div>
        <label htmlFor="firstName">FirstName</label>
        <input
          className="pl-5 py-5"
          type="text"
          name="firstName"
          placeholder="John"
        />
      </div>
      <div>
        <label htmlFor="lastName">LastName</label>
        <input
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
          Signin
        </button>
      </div>
    </form>
  );
}

export default SigninForm;
