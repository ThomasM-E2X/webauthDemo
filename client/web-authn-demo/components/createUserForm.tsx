"use client";

import { AuthContext } from "@/app/context/authContext";
import React, { FormEvent, useContext, useEffect, useState } from "react";

import { useRouter } from "next/navigation";

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
  const router = useRouter();

  const authContext = useContext(AuthContext);

  useEffect(() => {
    if (authContext && authContext.isAuth) {
      router.push("/");
    }
  }, [authContext, router]);

  async function handleSubmit(e: FormEvent<CreateForm>) {
    e.preventDefault();
    const { firstName, lastName } = e.currentTarget.elements;

    let challengeResponse = await fetch(
      "http://localhost:8080/generate_challenge",
      {
        method: "Get",
        headers: {
          "Access-Control-Allow-Methods": "*",
        },
      }
    );

    const { challenge_id, challenge } = await challengeResponse.json();

    let userId = new Uint8Array(16);

    window.crypto.getRandomValues(userId);

    let credential = (await navigator.credentials.create({
      publicKey: {
        challenge: new Uint8Array(challenge),
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
    })) as Credential & { response: Record<string, any> };

    const clientDataJson = JSON.parse(
      new TextDecoder().decode(credential?.response?.clientDataJSON)
    );

    const publicKey = new TextDecoder("utf-8").decode(
      credential.response.getPublicKey()
    );

    const savedPublicKey = await fetch(
      `http://localhost:8080/save_public_key/${challenge_id}`,
      {
        method: "post",
        headers: {
          "Access-Control-Allow-Methods": "*",
          "Content-Type": "application/json",
        },
        body: JSON.stringify({
          credentialId: credential?.id,
          clientDataJson,
          publicKey,
          userId: new TextDecoder().decode(userId),
        }),
      }
    );

    if (savedPublicKey.status !== 200) {
      seterror("Server failed to authenticate passKey");
      return;
    }

    authContext?.setIsAuth(true);
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
