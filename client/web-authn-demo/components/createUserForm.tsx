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

function CreateUserForm({}: Props) {
  const [canUseAuthn, setCanUseAuthn] = useState(false);
  const [error, seterror] = useState<string | undefined>();

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

    // if (canUseAuthn) {

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

    let credential = await navigator.credentials.create({
      publicKey: {
        challenge: bytes,
        rp: {
          id: "localhost",
          name: "localHost",
        },
        user: {
          id: userId,
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

    const savedPublicKey = await fetch(
      "http://localhost:8080/save_public_key",
      {
        method: "post",
        headers: {
          "Access-Control-Allow-Methods": "*",
          "Content-Type": "application/json",
        },
        body: JSON.stringify(credential),
        // body: JSON.stringify({
        //   publicKey: btoa(String.fromCharCode.apply(null, publicKey)),
        // }),
      }
    );

    console.log(savedPublicKey);

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
          Create User
        </button>
      </div>
    </form>
  );
}

export default CreateUserForm;
