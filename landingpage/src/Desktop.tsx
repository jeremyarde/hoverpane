import { MeshGradient } from "@paper-design/shaders-react";
import React, { useState } from "react";
import FloatingWidget from "./components/FloatingWidget";
import Dock from "./components/Dock";
import {
  FeaturesIcon,
  HowItWorksIcon,
  DownloadIcon,
} from "./components/DockIcons";

const googleImage = "/google.png";
const chatGpt = "/gpt.png";
import trayIcon from "/tray-icon.png";
import { paidEarlyAccessLink } from "./constants";

const Desktop: React.FC = () => {
  const [widgets] = useState<
    Array<{
      id: number;
      type: string;
      ReactNode: React.ReactNode;
      position: { x: number; y: number };
      title: string;
      iframeSrc?: string;
      size?: { width: number; height: number };
    }>
  >([
    {
      id: 1,
      type: "Widget 1",
      ReactNode: (
        <>
          <img src={googleImage}></img>
        </>
      ),
      position: {
        x: window.innerWidth * 0.1,
        y: 100,
      },
      title: "Google",
      size: {
        width: 300,
        height: 200,
      },
    },
    {
      id: 2,
      type: "Widget 2",
      ReactNode: (
        <>
          <img src={chatGpt}></img>
        </>
      ),
      position: {
        x: window.innerWidth * 0.6,
        y: 100,
      },
      title: "ChatGPT",
      size: {
        width: 300,
        height: 200,
      },
    },
    {
      id: 3,
      type: "Widget 3",
      iframeSrc: "/98_widget.html",
      position: {
        x: window.innerWidth * 0.1,
        y: 600,
      },
      title: "poke",
      ReactNode: <>{/* <img src={poke}></img> */}</>,
      size: {
        width: 300,
        height: 280,
      },
    },
    {
      id: 4,
      type: "Widget 4",
      iframeSrc: "/todo.html",
      position: {
        x: window.innerWidth * 0.75,
        y: 600,
      },
      title: "Todo",
      ReactNode: <>{/* <img src={todo}></img> */}</>,
      size: {
        width: 300,
        height: 250,
      },
    },
    {
      id: 5,
      type: "Widget 5",
      title: "HoverPane",
      ReactNode: (
        <>
          <header className="text-center bg-indigo-400 w-full h-full">
            <div className="hero-content text-white">
              <h1 className="hero-title">HoverPane</h1>
              <p className="hero-subtitle text-white">
                Transform any website into a sleek desktop widget in seconds.
                Keep your favorite content always visible and easily accessible.
              </p>
              <a href={paidEarlyAccessLink} className="hero-button text-white">
                Get Started
              </a>
              {/* <div className="demo-video-container">
                <video className="demo-video" autoPlay loop muted playsInline>
                  <source src="/tools-demo.webm" type="video/webm" />
                  Your browser does not support the video tag.
                </video>
              </div> */}
            </div>
          </header>
        </>
      ),
      size: {
        width: 500,
        height: 400,
      },
      position: {
        x: window.innerWidth / 2 - 200,
        y: 400,
      },
    },
  ]);

  const dockItems = [
    {
      icon: <FeaturesIcon />,
      label: "Features",
      onClick: () => {
        const element = document.getElementById("features");
        element?.scrollIntoView({ behavior: "smooth" });
      },
    },
    {
      icon: <HowItWorksIcon />,
      label: "How",
      onClick: () => {
        const element = document.getElementById("how-it-works");
        element?.scrollIntoView({ behavior: "smooth" });
      },
    },
    {
      icon: <DownloadIcon />,
      label: "Download",
      onClick: () => {
        const element = document.getElementById("pricing");
        element?.scrollIntoView({ behavior: "smooth" });
      },
    },
  ];

  return (
    <>
      <div style={{ position: "relative", width: "100vw", height: "100vh" }}>
        <div className="fixed top-0 left-0 right-0 h-16 bg-white/80 backdrop-blur-md flex items-center px-6 text-base text-gray-800 border-b border-gray-200 z-50">
          <div className="flex items-center gap-6">
            <img src={trayIcon} alt="tray icon" className="w-6 h-6" />
            <a href="/" className="font-bold no-underline text-gray-800">
              HoverPane
            </a>
            <button
              onClick={() => {
                const element = document.getElementById("features");
                element?.scrollIntoView({ behavior: "smooth" });
              }}
              className="no-underline text-gray-800 bg-transparent border-none cursor-pointer p-0 font-inherit hover:text-gray-600"
            >
              Features
            </button>
            <button
              onClick={() => {
                const element = document.getElementById("how-it-works");
                element?.scrollIntoView({ behavior: "smooth" });
              }}
              className="no-underline text-gray-800 bg-transparent border-none cursor-pointer p-0 font-inherit hover:text-gray-600"
            >
              How It Works
            </button>
            <button
              onClick={() => {
                const element = document.getElementById("use-cases");
                element?.scrollIntoView({ behavior: "smooth" });
              }}
              className="no-underline text-gray-800 bg-transparent border-none cursor-pointer p-0 font-inherit hover:text-gray-600"
            >
              Ideas
            </button>
            <button
              onClick={() => {
                const element = document.getElementById("pricing");
                element?.scrollIntoView({ behavior: "smooth" });
              }}
              className="no-underline text-gray-800 bg-transparent border-none cursor-pointer p-0 font-inherit hover:text-gray-600"
            >
              Pricing
            </button>
          </div>
          <div className="ml-auto flex items-center gap-6">
            <span>🔋</span>
            <span>📶</span>
            <span>🔊</span>
            <span>🔍</span>
            <span>{new Date().toLocaleTimeString()}</span>
          </div>
        </div>
        {/* Floating Widgets */}
        {widgets.map((widget) => (
          <FloatingWidget
            key={widget.id}
            initialPosition={{
              x: widget.position.x,
              y: widget.position.y,
            }}
            width={widget.size?.width}
            height={widget.size?.height}
            title={widget.title}
            iframeSrc={widget.iframeSrc}
            // scrollThreshold={widget.scrollThreshold ?? 200}
          >
            {widget.ReactNode}
          </FloatingWidget>
        ))}

        {/* Add the Dock component */}
        <Dock items={dockItems} />

        <div
          style={{
            width: "100%",
            height: "100vh",
            position: "relative",
            zIndex: -1,
          }}
        >
          <MeshGradient
            color1="#FFE4E1"
            color2="#E6E6FA"
            color3="#D8BFD8"
            color4="#DDA0DD"
            speed={0.0}
            style={{
              width: "100%",
              height: "100%",
              zIndex: -1,
            }}
          />
        </div>
      </div>
    </>
  );
};

export default Desktop;
