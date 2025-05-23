// import { MeshGradient } from "@paper-design/shaders-react";
import React, { useState } from "react";
import FloatingWidget from "./components/FloatingWidget";
import Dock from "./components/Dock";
import {
  FeaturesIcon,
  HowItWorksIcon,
  DownloadIcon,
  ExampleIcon,
} from "./components/DockIcons";

const googleImage = "/google.png";
const chatGpt = "/gpt.png";
import trayIcon from "../assets/tray-icon.png";
import wallpaper from "../assets/wallpaper.png";
import wallpaperGaus from "../assets/wallpaper-gaus.png";
import { getDownloadUrl, handleDownload } from "../utils";
import { HOVERPANE_DOWNLOAD_URL } from "./constants";

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
          <header className="w-full h-full text-center bg-indigo-400">
            <div className="text-white hero-content">
              <h1 className="hero-title">HoverPane</h1>
              <p className="text-white hero-subtitle">
                Transform any website into a sleek desktop widget in seconds.
                Keep your favorite content always visible and easily accessible.
              </p>
              <button
                onClick={handleDownload}
                className="text-white hero-button"
              >
                Get Started
              </button>
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
      icon: <ExampleIcon />,
      label: "Example",
      onClick: () => {
        const element = document.getElementById("home");
        element?.scrollIntoView({ behavior: "smooth" });
      },
    },
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
      <section id="home">
        <div style={{ position: "relative", width: "100vw", height: "100vh" }}>
          <div className="flex fixed top-0 right-0 left-0 z-50 items-center px-6 h-16 text-base text-gray-800 border-b border-gray-200 backdrop-blur-md bg-white/80">
            <div className="flex gap-6 items-center">
              <img src={trayIcon.src} alt="tray icon" className="w-6 h-6" />
              <a href="/" className="font-bold text-gray-800 no-underline">
                HoverPane
              </a>
              <button
                onClick={() => {
                  const element = document.getElementById("features");
                  element?.scrollIntoView({ behavior: "smooth" });
                }}
                className="p-0 text-gray-800 no-underline bg-transparent border-none cursor-pointer font-inherit hover:text-gray-600"
              >
                Features
              </button>
              <button
                onClick={() => {
                  const element = document.getElementById("how-it-works");
                  element?.scrollIntoView({ behavior: "smooth" });
                }}
                className="p-0 text-gray-800 no-underline bg-transparent border-none cursor-pointer font-inherit hover:text-gray-600"
              >
                How It Works
              </button>
              <button
                onClick={() => {
                  const element = document.getElementById("use-cases");
                  element?.scrollIntoView({ behavior: "smooth" });
                }}
                className="p-0 text-gray-800 no-underline bg-transparent border-none cursor-pointer font-inherit hover:text-gray-600"
              >
                Ideas
              </button>
              <button
                onClick={() => {
                  const element = document.getElementById("pricing");
                  element?.scrollIntoView({ behavior: "smooth" });
                }}
                className="p-0 text-gray-800 no-underline bg-transparent border-none cursor-pointer font-inherit hover:text-gray-600"
              >
                Pricing
              </button>
            </div>
            <div className="flex gap-6 items-center ml-auto">
              {/* <span>{new Date().toLocaleTimeString()}</span>*/}
              <button
                onClick={handleDownload}
                className="p-2 text-gray-800 no-underline bg-transparent rounded-md border-none cursor-pointer font-inherit hover:text-gray-600 hover:bg-yellow-200"
              >
                Download
              </button>
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

          <div
            className=""
            style={{
              width: "100vw",
              height: "100vh",
              position: "absolute",
              zIndex: -1,
              // background: "blue",
              top: 0,
              left: 0,
            }}
          >
            <img
              src={wallpaper.src}
              alt="tray icon"
              className="w-full h-full"
            />
          </div>
          <Dock items={dockItems} />
        </div>
      </section>
    </>
  );
};

export default Desktop;
