import { Phone, Video, Pin, Search, Smile, Paperclip, Send, Image as ImageIcon } from "lucide-react";

interface Message {
  id: number;
  sender: string;
  avatar: string;
  content: string;
  time: string;
  isMine: boolean;
}

const dmMessages: Message[] = [
  { id: 1, sender: "Alex Rivera", avatar: "🧑‍💻", content: "Hey! Have you seen the latest update?", time: "10:23 AM", isMine: false },
  { id: 2, sender: "You", avatar: "😎", content: "Not yet! What changed?", time: "10:24 AM", isMine: true },
  { id: 3, sender: "Alex Rivera", avatar: "🧑‍💻", content: "They completely redesigned the UI. It's so much cleaner now with these bubbly elements ✨", time: "10:25 AM", isMine: false },
  { id: 4, sender: "You", avatar: "😎", content: "That sounds amazing! I love the new direction they're going. The rounded corners everywhere give it such a friendly feel 🎉", time: "10:26 AM", isMine: true },
  { id: 5, sender: "Alex Rivera", avatar: "🧑‍💻", content: "Right? And the floating panels are gorgeous. Want to hop on a call and I'll show you?", time: "10:27 AM", isMine: false },
  { id: 6, sender: "You", avatar: "😎", content: "Sure, let me grab my headphones! 🎧", time: "10:28 AM", isMine: true },
];

const channelMessages: Message[] = [
  { id: 1, sender: "Maya Chen", avatar: "👩‍🎨", content: "Who's up for some gaming tonight? 🎮", time: "8:15 PM", isMine: false },
  { id: 2, sender: "Jordan Taylor", avatar: "🎮", content: "Count me in! What are we playing?", time: "8:16 PM", isMine: false },
  { id: 3, sender: "You", avatar: "😎", content: "I'm down! Let's do a few rounds of Valorant", time: "8:18 PM", isMine: true },
  { id: 4, sender: "Sam Wilson", avatar: "🎵", content: "I'll join the voice channel and DJ while you guys play 🎶", time: "8:20 PM", isMine: false },
  { id: 5, sender: "Maya Chen", avatar: "👩‍🎨", content: "Perfect! See everyone in the Gaming Room voice channel at 9!", time: "8:22 PM", isMine: false },
];

interface ChatAreaProps {
  type: "dm" | "channel";
  title: string;
  subtitle?: string;
}

const ChatArea = ({ type, title, subtitle }: ChatAreaProps) => {
  const messages = type === "dm" ? dmMessages : channelMessages;

  return (
    <div className="flex-1 bg-card rounded-3xl m-2 ml-0 float-shadow flex flex-col overflow-hidden">
      {/* Header */}
      <div className="flex items-center justify-between px-5 py-3 border-b border-border">
        <div className="flex items-center gap-3">
          <div className="w-9 h-9 rounded-full bg-surface-elevated flex items-center justify-center text-base">
            {type === "dm" ? "🧑‍💻" : "#"}
          </div>
          <div>
            <h3 className="text-sm font-bold text-foreground">{title}</h3>
            {subtitle && <p className="text-[11px] text-muted-foreground">{subtitle}</p>}
          </div>
        </div>
        <div className="flex items-center gap-1">
          {type === "dm" && (
            <>
              <button className="px-4 py-1.5 rounded-full bg-primary/10 text-primary text-xs font-semibold hover:bg-primary/20 transition-colors flex items-center gap-1.5">
                <Phone size={14} /> Voice
              </button>
              <button className="px-4 py-1.5 rounded-full bg-primary/10 text-primary text-xs font-semibold hover:bg-primary/20 transition-colors flex items-center gap-1.5">
                <Video size={14} /> Video
              </button>
            </>
          )}
          <button className="w-8 h-8 rounded-full hover:bg-secondary flex items-center justify-center text-muted-foreground transition-colors">
            <Pin size={16} />
          </button>
          <button className="w-8 h-8 rounded-full hover:bg-secondary flex items-center justify-center text-muted-foreground transition-colors">
            <Search size={16} />
          </button>
        </div>
      </div>

      {/* Messages */}
      <div className="flex-1 overflow-y-auto scrollbar-hide px-5 py-4 space-y-3">
        {messages.map((msg) => (
          <div
            key={msg.id}
            className={`flex items-end gap-2 bubble-in ${msg.isMine ? "flex-row-reverse" : ""}`}
          >
            {!msg.isMine && (
              <div className="w-8 h-8 rounded-full bg-surface-elevated flex items-center justify-center text-sm flex-shrink-0">
                {msg.avatar}
              </div>
            )}
            <div className={`max-w-[65%] ${msg.isMine ? "items-end" : "items-start"}`}>
              {!msg.isMine && (
                <span className="text-[11px] font-semibold text-muted-foreground ml-1 mb-0.5 block">{msg.sender}</span>
              )}
              <div
                className={`px-4 py-2.5 text-sm leading-relaxed ${
                  msg.isMine
                    ? "bg-primary text-primary-foreground rounded-3xl rounded-br-lg"
                    : "bg-secondary text-secondary-foreground rounded-3xl rounded-bl-lg"
                }`}
              >
                {msg.content}
              </div>
              <span className={`text-[10px] text-muted-foreground mt-0.5 block ${msg.isMine ? "text-right mr-1" : "ml-1"}`}>
                {msg.time}
              </span>
            </div>
          </div>
        ))}
      </div>

      {/* Input */}
      <div className="px-4 pb-4 pt-2">
        <div className="flex items-center gap-2 bg-secondary rounded-full px-4 py-2 float-shadow-sm">
          <button className="w-8 h-8 rounded-full hover:bg-surface-hover flex items-center justify-center text-muted-foreground transition-colors">
            <Paperclip size={18} />
          </button>
          <button className="w-8 h-8 rounded-full hover:bg-surface-hover flex items-center justify-center text-muted-foreground transition-colors">
            <ImageIcon size={18} />
          </button>
          <input
            type="text"
            placeholder={`Message ${title}...`}
            className="flex-1 bg-transparent text-sm text-foreground placeholder:text-muted-foreground outline-none"
          />
          <button className="w-8 h-8 rounded-full hover:bg-surface-hover flex items-center justify-center text-muted-foreground transition-colors">
            <Smile size={18} />
          </button>
          <button className="w-8 h-8 rounded-full bg-primary flex items-center justify-center text-primary-foreground hover:opacity-90 transition-opacity glow-accent-sm">
            <Send size={16} />
          </button>
        </div>
      </div>
    </div>
  );
};

export default ChatArea;
