import { Mic, MicOff, Video, VideoOff, Monitor, PhoneOff, Maximize2 } from "lucide-react";
import { useState } from "react";

const participants = [
  { id: 1, name: "You", avatar: "😎", speaking: false, muted: false, videoOn: true },
  { id: 2, name: "Alex Rivera", avatar: "🧑‍💻", speaking: true, muted: false, videoOn: true },
  { id: 3, name: "Maya Chen", avatar: "👩‍🎨", speaking: false, muted: true, videoOn: false },
  { id: 4, name: "Jordan Taylor", avatar: "🎮", speaking: false, muted: false, videoOn: true },
  { id: 5, name: "Sam Wilson", avatar: "🎵", speaking: false, muted: false, videoOn: true },
  { id: 6, name: "Riley Brooks", avatar: "📸", speaking: false, muted: true, videoOn: false },
];

interface VoiceCallViewProps {
  onEnd: () => void;
}

const VoiceCallView = ({ onEnd }: VoiceCallViewProps) => {
  const [muted, setMuted] = useState(false);
  const [videoOn, setVideoOn] = useState(true);

  return (
    <div className="flex-1 bg-background rounded-3xl m-2 ml-0 float-shadow flex flex-col overflow-hidden">
      {/* Header */}
      <div className="flex items-center justify-between px-5 py-3">
        <div>
          <h3 className="text-sm font-bold text-foreground">Gaming Room</h3>
          <p className="text-[11px] text-muted-foreground">{participants.length} participants • Voice Channel</p>
        </div>
        <button className="w-8 h-8 rounded-full hover:bg-secondary flex items-center justify-center text-muted-foreground transition-colors">
          <Maximize2 size={16} />
        </button>
      </div>

      {/* Video Grid */}
      <div className="flex-1 px-4 pb-2 overflow-y-auto scrollbar-hide">
        <div className="grid grid-cols-3 gap-3 h-full auto-rows-fr">
          {participants.map((p) => (
            <div
              key={p.id}
              className={`relative rounded-3xl overflow-hidden flex items-center justify-center min-h-[160px] ${
                p.videoOn
                  ? "bg-surface-elevated"
                  : "bg-secondary"
              } ${
                p.speaking ? "ring-2 ring-primary animate-pulse-glow" : ""
              } transition-all duration-300`}
            >
              {p.videoOn ? (
                <div className="w-full h-full bg-gradient-to-br from-surface-elevated to-secondary flex items-center justify-center">
                  <span className="text-5xl">{p.avatar}</span>
                </div>
              ) : (
                <div className="flex flex-col items-center gap-2">
                  <div className="w-16 h-16 rounded-full bg-surface-elevated flex items-center justify-center text-3xl">
                    {p.avatar}
                  </div>
                </div>
              )}
              {/* Name tag */}
              <div className="absolute bottom-3 left-3 right-3 flex items-center justify-between">
                <span className="text-xs font-semibold text-foreground bg-background/60 backdrop-blur-sm px-3 py-1 rounded-full">
                  {p.name}
                  {p.muted && <MicOff size={10} className="inline ml-1 text-destructive" />}
                </span>
              </div>
            </div>
          ))}
        </div>
      </div>

      {/* Controls */}
      <div className="flex items-center justify-center gap-3 py-4 px-4">
        <div className="flex items-center gap-2 bg-secondary rounded-full px-4 py-2 float-shadow-sm">
          <button
            onClick={() => setMuted(!muted)}
            className={`w-11 h-11 rounded-full flex items-center justify-center transition-all ${
              muted
                ? "bg-destructive/20 text-destructive"
                : "bg-surface-elevated text-foreground hover:bg-surface-hover"
            }`}
          >
            {muted ? <MicOff size={20} /> : <Mic size={20} />}
          </button>
          <button
            onClick={() => setVideoOn(!videoOn)}
            className={`w-11 h-11 rounded-full flex items-center justify-center transition-all ${
              !videoOn
                ? "bg-destructive/20 text-destructive"
                : "bg-surface-elevated text-foreground hover:bg-surface-hover"
            }`}
          >
            {videoOn ? <Video size={20} /> : <VideoOff size={20} />}
          </button>
          <button className="w-11 h-11 rounded-full bg-surface-elevated text-foreground hover:bg-surface-hover flex items-center justify-center transition-all">
            <Monitor size={20} />
          </button>
          <div className="w-[2px] h-6 bg-border rounded-full mx-1" />
          <button
            onClick={onEnd}
            className="w-11 h-11 rounded-full bg-destructive text-destructive-foreground hover:opacity-90 flex items-center justify-center transition-all"
          >
            <PhoneOff size={20} />
          </button>
        </div>
      </div>
    </div>
  );
};

export default VoiceCallView;
