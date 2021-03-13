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
          Load apks
        </button>
      );
    case "loading":
      return <div>Loading...</div>;
    case "success":
      return (
        <div>
          <button onClick={() => send("RETRY")}>Retry</button>
          List of apks: (time: {state.context.data.elapsed})
          <ul>
            {state.context.data.apks.map((apk) => (
              <li key={apk.package}>
                {apk.package} - {apk.version_name} - {apk.version_code}
              </li>
            ))}
          </ul>
        </div>
      );
    case "failure":
      return (
        <>
          <p>{state.context.error}</p>
          <button onClick={() => send("RETRY")}>Retry</button>
        </>
      );
    default:
      return null;
  }
}

export default Apks;
