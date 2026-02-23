import { Search } from "lucide-react";

const conversations = [
  { id: 1, name: "Alex Rivera", avatar: "🧑‍💻", lastMsg: "Check out this new feature!", time: "2m", unread: 3, online: true },
  { id: 2, name: "Maya Chen", avatar: "👩‍🎨", lastMsg: "The design looks amazing", time: "15m", unread: 1, online: true },
  { id: 3, name: "Jordan Taylor", avatar: "🎮", lastMsg: "GG! Next round?", time: "1h", unread: 0, online: false },
  { id: 4, name: "Sam Wilson", avatar: "🎵", lastMsg: "Listen to this track", time: "3h", unread: 0, online: true },
  { id: 5, name: "Riley Brooks", avatar: "📸", lastMsg: "Sent you the photos", time: "5h", unread: 0, online: false },
  { id: 6, name: "Casey Morgan", avatar: "🚀", lastMsg: "Ready for launch!", time: "1d", unread: 0, online: false },
];

interface DMSidebarProps {
  activeChat: number;
  onChatSelect: (id: number) => void;
}

const DMSidebar = ({ activeChat, onChatSelect }: DMSidebarProps) => {
  return (
    <div className="w-60 bg-card rounded-3xl m-2 ml-0 float-shadow flex flex-col overflow-hidden">
      <div className="p-4 pb-2">
        <h2 className="text-foreground font-bold text-base mb-3">Messages</h2>
        <div className="relative">
          <Search size={16} className="absolute left-3 top-1/2 -translate-y-1/2 text-muted-foreground" />
          <input
            type="text"
            placeholder="Search..."
            className="w-full bg-secondary rounded-full py-2 pl-9 pr-3 text-sm text-foreground placeholder:text-muted-foreground outline-none focus:ring-2 focus:ring-primary/40 transition-all"
          />
        </div>
      </div>

      <div className="flex-1 overflow-y-auto scrollbar-hide px-2 pb-2">
        {conversations.map((c) => (
          <button
            key={c.id}
            onClick={() => onChatSelect(c.id)}
            className={`w-full flex items-center gap-3 px-3 py-2.5 rounded-2xl transition-all duration-150 ${
              activeChat === c.id ? "bg-secondary" : "hover:bg-secondary/50"
            }`}
          >
            <div className="relative flex-shrink-0">
              <div className="w-10 h-10 rounded-full bg-surface-elevated flex items-center justify-center text-lg">
                {c.avatar}
              </div>
              {c.online && (
                <div className="absolute -bottom-0.5 -right-0.5 w-3.5 h-3.5 bg-online rounded-full border-2 border-card" />
              )}
            </div>
            <div className="flex-1 min-w-0 text-left">
              <div className="flex items-center justify-between">
                <span className="text-sm font-semibold text-foreground truncate">{c.name}</span>
                <span className="text-[11px] text-muted-foreground flex-shrink-0">{c.time}</span>
              </div>
              <p className="text-xs text-muted-foreground truncate">{c.lastMsg}</p>
            </div>
            {c.unread > 0 && (
              <div className="w-5 h-5 rounded-full bg-unread flex items-center justify-center flex-shrink-0">
                <span className="text-[10px] font-bold text-primary-foreground">{c.unread}</span>
              </div>
            )}
          </button>
        ))}
      </div>
    </div>
  );
};

export default DMSidebar;
