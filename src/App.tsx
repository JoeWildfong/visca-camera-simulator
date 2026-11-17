import { invoke } from "@tauri-apps/api/core";
import "./App.css";
import { useConnected } from "./useConnected";
import { RecentMessages } from "./RecentMessages";
import { addMessage } from "./useRecentIncomingMessages";

const sendData = (data: number[]) => {
  const str_message = "[send] " + data.map(num => num.toString(16).toUpperCase().padStart(2, "0")).join(" ");
  addMessage(str_message);
  invoke("send_data", {data: data});
}

function App() {
  const connected = useConnected();
  return (
    <main className={"container" + (connected ? " connected" : "")}>
      <section className="recentMessages">
        <RecentMessages />
      </section>
      <section className="socket">
        <h3>Socket 1</h3>
        <button onClick={() => sendData([0x90, 0x41, 0xff])}>Accept</button>
        <button onClick={() => sendData([0x90, 0x51, 0xff])}>Finish</button>
      </section>
      <section className="socket">
        <h3>Socket 2</h3>
        <button onClick={() => sendData([0x90, 0x42, 0xff])}>Accept</button>
        <button onClick={() => sendData([0x90, 0x52, 0xff])}>Finish</button>
      </section>
      <section className="query">
        <h3>Query Responses</h3>
        <button onClick={() => sendData([0x90, 0x50, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0xff])}>Pan/Tilt at 0/0</button>
        <button onClick={() => sendData([0x90, 0x50, 0x00, 0x00, 0x0f, 0x0f, 0x00, 0x00, 0x0f, 0x0f, 0xff])}>Pan/Tilt at 255/255</button>
        <button onClick={() => sendData([0x90, 0x50, 0x00, 0x00, 0x00, 0x00, 0xff])}>Zoom at 0</button>
        <button onClick={() => sendData([0x90, 0x50, 0x00, 0x00, 0x0f, 0x0f, 0xff])}>Zoom at 255</button>
      </section>
    </main>
  );
}

export default App;
