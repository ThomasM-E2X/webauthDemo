"use client";

import { AuthContext } from "@/app/context/authContext";
import { encode } from "js-base64";
import { useRouter } from "next/navigation";
import React, { FormEvent, useContext, useEffect, useState } from "react";

interface FormElements extends HTMLFormControlsCollection {
  firstName: HTMLInputElement;
  lastName: HTMLInputElement;
}
interface CreateForm extends HTMLFormElement {
  readonly elements: FormElements;
}

function convertASN1toRaw(signatureBuffer: ArrayBuffer) {
  // Convert signature from ASN.1 sequence to "raw" format
  const usignature = new Uint8Array(signatureBuffer);
  const rStart = usignature[4] === 0 ? 5 : 4;
  const rEnd = rStart + 32;
  const sStart = usignature[rEnd + 2] === 0 ? rEnd + 3 : rEnd + 2;
  const r = usignature.slice(rStart, rEnd);
  const s = usignature.slice(sStart);
  return new Uint8Array([...r, ...s]);
}

type Props = {};

function SigninForm({}: Props) {
  const [signInFailed, setsignInFailed] = useState<boolean | null>(null);
  const router = useRouter();
  const authContext = useContext(AuthContext);

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

      const creds = (await window.navigator.credentials.get({
        publicKey: {
          challenge: new Uint8Array(challenge),
          allowCredentials: [],
          rpId: "localhost",
        },
      })) as unknown as Credential & { response: Record<string, any> };
      const textDecoder = new TextDecoder("utf-8");

      const rawSig = convertASN1toRaw(creds.response.signature);

      const authenticatorData = textDecoder.decode(
        creds.response.authenticatorData
      );

      const clientDataJson = encode(creds.response.clientDataJSON);
      const signature = encode(textDecoder.decode(rawSig));

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
      );

      if (verifyResponse.status !== 200) {
        console.log("Server failed to authenticate passKey");
        setsignInFailed(true);
      } else {
      }

      authContext?.setIsAuth(true);
    };

    requestChallenge();
  }, []);

  useEffect(() => {
    if (authContext && authContext.isAuth) {
      router.push("/");
    }
  }, [authContext, router]);

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

      {signInFailed && <p>Error could not be authenticated</p>}
    </form>
  );
}

export default SigninForm;
