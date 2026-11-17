import { listen } from "@tauri-apps/api/event"
import { useSyncExternalStore } from "react"

type Message = {
    body: string;
    id: string;
};

let messageIdCounter = 0;
let messageBuffer: Message[] = [];
let listeners = new Set<() => void>();

const subscribe = (onChanged: () => void): (() => void) => {
    listeners.add(onChanged);
    return () => listeners.delete(onChanged);
};

const getSnapshot = (): Message[] => {
    return messageBuffer;
};

type ListenerState =
    | { state: "not-started" }
    | { state: "starting" }
    | { state: "running", "unsubscribe": () => void };

let listenerState: ListenerState = { state: "not-started" };

export const addMessage = (message: string) => {
    const messageId = String(++messageIdCounter);
    // make a new array so useSyncExternalStore doesn't ignore the update
    messageBuffer = [...messageBuffer, { id: messageId, body: message }];
    while (messageBuffer.length > 10) {
        messageBuffer.shift();
    }
    for (const notify of listeners) {
        notify();
    }
};

export const startListener = () => {
    if (listenerState.state !== "not-started") {
        return;
    }
    listenerState = { state: "starting" };
    listen<number[]>("incoming-msg", msg => {
        const str_message = "[recv] " + msg.payload.map(byte => byte.toString(16).toUpperCase()).join(" ");
        addMessage(str_message);
    }).then(unsubscribe => {
        if (listenerState.state === "starting") {
            listenerState = { state: "running", unsubscribe };
        } else {
            unsubscribe();
        }
    });
};

export const stopListener = () => {
    if (listenerState.state === "running") {
        listenerState.unsubscribe();
    }
    listenerState = { state: "not-started" };
};

export const useRecentIncomingMessages = (): Message[] => {
    startListener();
    return useSyncExternalStore(subscribe, getSnapshot);
};