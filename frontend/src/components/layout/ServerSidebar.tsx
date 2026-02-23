import { Hash, Volume2, ChevronDown, Settings } from "lucide-react";
import banner1 from "@/assets/banner-1.jpg";

const textChannels = [
  { id: "general", name: "general", unread: true },
  { id: "introductions", name: "introductions", unread: false },
  { id: "off-topic", name: "off-topic", unread: true },
  { id: "announcements", name: "announcements", unread: false },
];

const voiceChannels = [
  { id: "lounge", name: "Lounge", users: 3 },
  { id: "gaming", name: "Gaming Room", users: 7 },
  { id: "music", name: "Music Vibes", users: 2 },
];

interface ServerSidebarProps {
  activeChannel: string;
  onChannelSelect: (id: string) => void;
  onVoiceJoin: () => void;
}

const ServerSidebar = ({ activeChannel, onChannelSelect, onVoiceJoin }: ServerSidebarProps) => {
  return (
    <div className="w-60 bg-card rounded-3xl m-2 ml-0 float-shadow flex flex-col overflow-hidden">
      {/* Server Banner */}
      <div className="relative h-28 overflow-hidden rounded-t-3xl">
        <img src={banner1} alt="Server banner" className="w-full h-full object-cover" />
        <div className="absolute inset-0 bg-gradient-to-t from-card/90 to-transparent" />
        <div className="absolute bottom-2 left-4 right-4">
          <h2 className="text-foreground font-bold text-base flex items-center gap-1">
            Gaming Hub
            <ChevronDown size={14} />
          </h2>
        </div>
      </div>

      <div className="flex-1 overflow-y-auto scrollbar-hide px-2 py-2">
        {/* Text Channels */}
        <div className="mb-3">
          <h3 className="text-[11px] font-bold text-muted-foreground uppercase tracking-wider px-2 mb-1">
            Text Channels
          </h3>
          {textChannels.map((ch) => (
            <button
              key={ch.id}
              onClick={() => onChannelSelect(ch.id)}
              className={`w-full flex items-center gap-2 px-3 py-1.5 rounded-xl text-sm transition-all duration-150 ${
                activeChannel === ch.id
                  ? "bg-secondary text-foreground font-semibold"
                  : "text-muted-foreground hover:bg-secondary/50 hover:text-foreground"
              }`}
            >
              <Hash size={16} className="flex-shrink-0 opacity-60" />
              <span className="truncate">{ch.name}</span>
              {ch.unread && activeChannel !== ch.id && (
                <div className="w-2 h-2 rounded-full bg-primary ml-auto flex-shrink-0" />
              )}
            </button>
          ))}
        </div>

        {/* Voice Channels */}
        <div>
          <h3 className="text-[11px] font-bold text-muted-foreground uppercase tracking-wider px-2 mb-1">
            Voice Channels
          </h3>
          {voiceChannels.map((ch) => (
            <button
              key={ch.id}
              onClick={onVoiceJoin}
              className="w-full flex items-center gap-2 px-3 py-1.5 rounded-xl text-sm text-muted-foreground hover:bg-secondary/50 hover:text-foreground transition-all duration-150"
            >
              <Volume2 size={16} className="flex-shrink-0 opacity-60" />
              <span className="truncate">{ch.name}</span>
              <span className="text-[11px] text-muted-foreground/60 ml-auto">{ch.users}</span>
            </button>
          ))}
        </div>
      </div>

      {/* User Panel */}
      <div className="p-2 border-t border-border">
        <div className="flex items-center gap-2 px-2 py-1.5 rounded-xl hover:bg-secondary/50 transition-colors cursor-pointer">
          <div className="relative">
            <div className="w-8 h-8 rounded-full bg-surface-elevated flex items-center justify-center text-sm">
              😎
            </div>
            <div className="absolute -bottom-0.5 -right-0.5 w-3 h-3 bg-online rounded-full border-2 border-card" />
          </div>
          <div className="flex-1 min-w-0">
            <p className="text-xs font-semibold text-foreground truncate">You</p>
            <p className="text-[10px] text-muted-foreground">Online</p>
          </div>
          <Settings size={14} className="text-muted-foreground" />
        </div>
      </div>
    </div>
  );
};

export default ServerSidebar;
