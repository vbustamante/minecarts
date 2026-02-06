import type { ContainerEvent } from "./types";

export type EventHandler = (event: ContainerEvent) => void;

export function connectWebSocket(onEvent: EventHandler): WebSocket {
  const protocol = window.location.protocol === "https:" ? "wss:" : "ws:";
  const ws = new WebSocket(`${protocol}//${window.location.host}/api/ws`);

  ws.addEventListener("message", (msg) => {
    const event: ContainerEvent = JSON.parse(msg.data);
    onEvent(event);
  });

  return ws;
}
