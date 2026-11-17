import { useRecentIncomingMessages } from "./useRecentIncomingMessages"
import "./RecentMessages.css"

export const RecentMessages = () => {
  const messages = useRecentIncomingMessages();
  return (
    <div className="messages">
      {messages.map(msg => <p key={msg.id}>{msg.body}</p>)}
    </div>
  );
}