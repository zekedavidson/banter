import { useState } from "react";
import ServerDock from "@/components/layout/ServerDock";
import DMSidebar from "@/components/layout/DMSidebar";
import ServerSidebar from "@/components/layout/ServerSidebar";
import ChatArea from "@/components/chat/ChatArea";
import DiscoveryView from "@/components/views/DiscoveryView";
import VoiceCallView from "@/components/views/VoiceCallView";

const Index = () => {
  const [activeView, setActiveView] = useState("dms");
  const [activeChat, setActiveChat] = useState(1);
  const [activeChannel, setActiveChannel] = useState("general");
  const [inVoiceCall, setInVoiceCall] = useState(false);

  const isServer = !["dms", "discovery"].includes(activeView);

  const renderSidebar = () => {
    if (activeView === "dms") {
      return <DMSidebar activeChat={activeChat} onChatSelect={setActiveChat} />;
    }
    if (activeView === "discovery") {
      return null;
    }
    return (
      <ServerSidebar
        activeChannel={activeChannel}
        onChannelSelect={setActiveChannel}
        onVoiceJoin={() => setInVoiceCall(true)}
      />
    );
  };

  const renderMain = () => {
    if (activeView === "discovery") {
      return <DiscoveryView />;
    }
    if (inVoiceCall) {
      return <VoiceCallView onEnd={() => setInVoiceCall(false)} />;
    }
    if (activeView === "dms") {
      return <ChatArea type="dm" title="Alex Rivera" subtitle="Online" />;
    }
    return <ChatArea type="channel" title={`# ${activeChannel}`} subtitle="Gaming Hub" />;
  };

  return (
    <div className="flex h-screen bg-background overflow-hidden p-0">
      <ServerDock activeView={activeView} onViewChange={(v) => { setActiveView(v); setInVoiceCall(false); }} />
      {renderSidebar()}
      {renderMain()}
    </div>
  );
};

export default Index;
