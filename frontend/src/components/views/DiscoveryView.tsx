import { Users, TrendingUp } from "lucide-react";
import banner1 from "@/assets/banner-1.jpg";
import banner2 from "@/assets/banner-2.jpg";
import banner3 from "@/assets/banner-3.jpg";
import banner4 from "@/assets/banner-4.jpg";
import banner5 from "@/assets/banner-5.jpg";
import banner6 from "@/assets/banner-6.jpg";

const communities = [
  { id: 1, name: "Gaming Legends", desc: "The ultimate gaming community", members: "24.5K", banner: banner1, icon: "🎮", category: "Gaming" },
  { id: 2, name: "Creative Studio", desc: "Design, art, and creativity hub", members: "18.2K", banner: banner2, icon: "🎨", category: "Art" },
  { id: 3, name: "Dev Central", desc: "Code, build, and ship together", members: "31.7K", banner: banner3, icon: "💻", category: "Technology" },
  { id: 4, name: "Nature Vibes", desc: "Explore the natural world", members: "12.1K", banner: banner4, icon: "🌿", category: "Nature" },
  { id: 5, name: "Music Lounge", desc: "Discover and share music", members: "42.3K", banner: banner5, icon: "🎵", category: "Music" },
  { id: 6, name: "Film Club", desc: "Movies, shows, and cinema", members: "9.8K", banner: banner6, icon: "🎬", category: "Entertainment" },
];

const DiscoveryView = () => {
  return (
    <div className="flex-1 bg-card rounded-3xl m-2 ml-0 float-shadow flex flex-col overflow-hidden">
      {/* Header */}
      <div className="px-6 pt-6 pb-4">
        <div className="flex items-center gap-3 mb-4">
          <TrendingUp size={22} className="text-primary" />
          <h2 className="text-xl font-bold text-foreground">Discover Communities</h2>
        </div>
        <div className="flex gap-2">
          {["All", "Gaming", "Music", "Art", "Technology"].map((cat, i) => (
            <button
              key={cat}
              className={`px-4 py-1.5 rounded-full text-xs font-semibold transition-all ${
                i === 0
                  ? "bg-primary text-primary-foreground glow-accent-sm"
                  : "bg-secondary text-secondary-foreground hover:bg-surface-hover"
              }`}
            >
              {cat}
            </button>
          ))}
        </div>
      </div>

      {/* Grid */}
      <div className="flex-1 overflow-y-auto scrollbar-hide px-6 pb-6">
        <div className="grid grid-cols-2 xl:grid-cols-3 gap-4">
          {communities.map((c) => (
            <div
              key={c.id}
              className="bg-secondary rounded-3xl overflow-hidden hover:scale-[1.02] transition-transform duration-200 float-shadow-sm group cursor-pointer"
            >
              <div className="relative h-28 overflow-hidden">
                <img
                  src={c.banner}
                  alt={c.name}
                  className="w-full h-full object-cover group-hover:scale-105 transition-transform duration-300"
                />
                <div className="absolute inset-0 bg-gradient-to-t from-secondary/80 to-transparent" />
              </div>
              <div className="px-4 pb-4 -mt-6 relative">
                <div className="w-12 h-12 rounded-2xl bg-surface-elevated flex items-center justify-center text-2xl border-4 border-secondary mb-2">
                  {c.icon}
                </div>
                <h3 className="text-sm font-bold text-foreground">{c.name}</h3>
                <p className="text-xs text-muted-foreground mb-3">{c.desc}</p>
                <div className="flex items-center justify-between">
                  <span className="text-[11px] text-muted-foreground flex items-center gap-1">
                    <Users size={12} /> {c.members} members
                  </span>
                  <button className="px-4 py-1.5 rounded-full bg-primary text-primary-foreground text-xs font-bold hover:opacity-90 transition-opacity glow-accent-sm">
                    Join
                  </button>
                </div>
              </div>
            </div>
          ))}
        </div>
      </div>
    </div>
  );
};

export default DiscoveryView;
