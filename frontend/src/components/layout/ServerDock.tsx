import { MessageCircle, Compass, Plus } from "lucide-react";

interface ServerDockProps {
  activeView: string;
  onViewChange: (view: string) => void;
}

const servers = [
  { id: "gaming", emoji: "🎮", color: "from-blue-500 to-cyan-400" },
  { id: "music", emoji: "🎵", color: "from-pink-500 to-rose-400" },
  { id: "dev", emoji: "💻", color: "from-emerald-500 to-teal-400" },
  { id: "art", emoji: "🎨", color: "from-orange-500 to-amber-400" },
];

const ServerDock = ({ activeView, onViewChange }: ServerDockProps) => {
  return (
    <div className="flex flex-col items-center w-[72px] py-3 gap-2 bg-surface rounded-3xl m-2 float-shadow scrollbar-hide overflow-y-auto">
      {/* DM Button */}
      <button
        onClick={() => onViewChange("dms")}
        className={`relative w-12 h-12 rounded-2xl flex items-center justify-center transition-all duration-200 ${
          activeView === "dms"
            ? "bg-primary rounded-[16px] glow-accent-sm"
            : "bg-surface-elevated hover:bg-surface-hover hover:rounded-[16px]"
        }`}
      >
        {activeView === "dms" && (
          <div className="absolute -left-[18px] w-1 h-8 bg-primary rounded-r-full" />
        )}
        <MessageCircle size={22} className={activeView === "dms" ? "text-primary-foreground" : "text-foreground"} />
      </button>

      <div className="w-8 h-[2px] bg-border rounded-full mx-auto" />

      {/* Server Icons */}
      {servers.map((server) => (
        <button
          key={server.id}
          onClick={() => onViewChange(server.id)}
          className={`relative w-12 h-12 rounded-2xl flex items-center justify-center text-lg transition-all duration-200 ${
            activeView === server.id
              ? `bg-gradient-to-br ${server.color} rounded-[16px] glow-accent-sm`
              : "bg-surface-elevated hover:bg-surface-hover hover:rounded-[16px]"
          }`}
        >
          {activeView === server.id && (
            <div className="absolute -left-[18px] w-1 h-8 bg-primary rounded-r-full" />
          )}
          <span>{server.emoji}</span>
        </button>
      ))}

      <div className="w-8 h-[2px] bg-border rounded-full mx-auto" />

      {/* Add Server */}
      <button
        onClick={() => {}}
        className="w-12 h-12 rounded-2xl flex items-center justify-center bg-surface-elevated hover:bg-online/20 hover:text-online transition-all duration-200 text-online/70"
      >
        <Plus size={22} />
      </button>

      {/* Discovery */}
      <button
        onClick={() => onViewChange("discovery")}
        className={`w-12 h-12 rounded-2xl flex items-center justify-center transition-all duration-200 ${
          activeView === "discovery"
            ? "bg-primary rounded-[16px] glow-accent-sm"
            : "bg-surface-elevated hover:bg-surface-hover hover:rounded-[16px]"
        }`}
      >
        {activeView === "discovery" && (
          <div className="absolute -left-[18px] w-1 h-8 bg-primary rounded-r-full" />
        )}
        <Compass size={22} className={activeView === "discovery" ? "text-primary-foreground" : "text-foreground"} />
      </button>
    </div>
  );
};

export default ServerDock;
