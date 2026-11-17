import { Channel, invoke } from "@tauri-apps/api/core";
import { useSyncExternalStore } from "react";

let connected: boolean = false;

const connectedSubscribe = (callback: () => void) => {
  const channel = new Channel((message: boolean) => {
    connected = message;
    callback();
  });

  invoke('stream_connected', { channel });

  return () => {};
};

const getConnected = (): boolean => connected;

export const useConnected = () => {
  const hook = useSyncExternalStore(connectedSubscribe, getConnected);
  return hook;
};
