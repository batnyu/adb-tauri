import { useMachine } from "@xstate/react";
import { fetchMachine } from "./fetchMachine";
import { promisified } from "tauri/api/tauri";

function Apks() {
  const [state, send] = useMachine(fetchMachine, {
    // actions: {
    //   notifySuccess: (ctx) => onResolve(ctx.data),
    // },
    services: {
      fetchData: (_, e) =>
        promisified({
          cmd: "listApks",
        }),
    },
  });

  switch (state.value) {
    case "idle":
      return (
        <button onClick={() => send("FETCH", { query: "something" })}>
          Search for something
        </button>
      );
    case "loading":
      return <div>Searching...</div>;
    case "success":
      return <div>Success! Data: {state.context.data}</div>;
    case "failure":
      return (
        <>
          <p>{state.context.error.message}</p>
          <button onClick={() => send("RETRY")}>Retry</button>
        </>
      );
    default:
      return null;
  }
}

export default Apks;
