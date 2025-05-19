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
        <div
          style={{
            position: "fixed",
            top: 0,
            left: 0,
            right: 0,
            height: "28px",
            background: "rgba(255, 255, 255, 0.8)",
            backdropFilter: "blur(10px)",
            display: "flex",
            alignItems: "center",
            padding: "0 10px",
            fontSize: "13px",
            color: "#333",
            borderBottom: "1px solid rgba(0, 0, 0, 0.1)",
            zIndex: 1000,
          }}
        >
          <div style={{ display: "flex", alignItems: "center", gap: "15px" }}>
            <img
              src={trayIcon}
              alt="tray icon"
              style={{ width: "16px", height: "16px" }}
            />
            <a
              href="/"
              style={{
                fontWeight: "bold",
                textDecoration: "none",
                color: "#333",
              }}
            >
              HoverPane
            </a>
            <button
              onClick={() => {
                const element = document.getElementById("features");
                element?.scrollIntoView({ behavior: "smooth" });
              }}
              style={{
                textDecoration: "none",
                color: "#333",
                background: "none",
                border: "none",
                cursor: "pointer",
                padding: 0,
                font: "inherit",
              }}
            >
              Features
            </button>
            <button
              onClick={() => {
                const element = document.getElementById("how-it-works");
                element?.scrollIntoView({ behavior: "smooth" });
              }}
              style={{
                textDecoration: "none",
                color: "#333",
                background: "none",
                border: "none",
                cursor: "pointer",
                padding: 0,
                font: "inherit",
              }}
            >
              How It Works
            </button>
            <button
              onClick={() => {
                const element = document.getElementById("use-cases");
                element?.scrollIntoView({ behavior: "smooth" });
              }}
              style={{
                textDecoration: "none",
                color: "#333",
                background: "none",
                border: "none",
                cursor: "pointer",
                padding: 0,
                font: "inherit",
              }}
            >
              Ideas
            </button>
            <button
              onClick={() => {
                const element = document.getElementById("pricing");
                element?.scrollIntoView({ behavior: "smooth" });
              }}
              style={{
                textDecoration: "none",
                color: "#333",
                background: "none",
                border: "none",
                cursor: "pointer",
                padding: 0,
                font: "inherit",
              }}
            >
              Pricing
            </button>
          </div>
          <div
            style={{
              marginLeft: "auto",
              display: "flex",
              alignItems: "center",
              gap: "15px",
            }}
          >
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
