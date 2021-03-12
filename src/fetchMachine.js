import { Machine, assign } from "xstate";

export const fetchMachine = Machine({
  id: "fetch",
  initial: "idle",
  context: {
    data: undefined,
    error: undefined,
  },
  states: {
    idle: {
      on: { FETCH: "loading" },
    },
    loading: {
      invoke: {
        src: "fetchData",
        onDone: {
          target: "success",
          actions: assign({
            data: (_, event) => event.data,
          }),
        },
        onError: {
          target: "failure",
          actions: assign({
            error: (_, event) => event.data,
          }),
        },
      },
    },
    success: {
      entry: "notifySuccess",
      type: "final",
    },
    failure: {
      on: {
        RETRY: "loading",
      },
    },
  },
});
