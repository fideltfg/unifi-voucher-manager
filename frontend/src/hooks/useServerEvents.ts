import { useEffect, useRef } from "react";

export function useServerEvents() {
  const eventSourceRef = useRef<EventSource | null>(null);

  useEffect(() => {
    console.log("Setting up SSE connection...");

    eventSourceRef.current = new EventSource("/api/events");

    eventSourceRef.current.onopen = () => {
      console.log("SSE connection opened");
    };

    eventSourceRef.current.onmessage = (event) => {
      try {
        const data = JSON.parse(event.data);
        switch (data.type) {
          case "connected":
            console.log(`SSE connected with clientId: ${data.clientId}`);
            break;
          case "vouchersUpdated":
            window.dispatchEvent(new CustomEvent("vouchersUpdated"));
            break;
          default:
            break;
        }
      } catch (error) {
        console.error("Error parsing SSE data:", error);
      }
    };

    eventSourceRef.current.onerror = (error) => {
      console.error("SSE connection error:", error);
    };

    return () => {
      eventSourceRef.current?.close();
    };
  }, []);

  return eventSourceRef.current;
}
