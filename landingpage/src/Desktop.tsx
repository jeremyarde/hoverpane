import { MeshGradient } from "@paper-design/shaders-react";
import React, { useState } from "react";
import FloatingWidget from "./components/FloatingWidget";

const googleImage = "/google.png";
const chatGpt = "/gpt.png";
import trayIcon from "/tray-icon.png";

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
        x: window.innerWidth * 0.1, // 10% from left
        y: window.innerHeight * 0.2, // 20% from top
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
        x: window.innerWidth * 0.6, // 60% from left
        y: window.innerHeight * 0.1, // 10% from top
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
        x: window.innerWidth * 0.1, // 10% from left
        y: window.innerHeight * 0.6, // 60% from top
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
        x: window.innerWidth * 0.6, // 60% from left
        y: window.innerHeight * 0.6, // 60% from top
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
      iframeSrc: "https://hoverpane.com#features",
      title: "HoverPane",
      ReactNode: <>{/* <img src={stock}></img> */}</>,
      size: {
        width: 500,
        height: 400,
      },
      position: {
        x: window.innerWidth / 2 - 200, // Center horizontally (half of width)
        y: window.innerHeight / 2 - 150, // Center vertically (half of height)
      },
    },
  ]);

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
            <a
              href="#features"
              style={{ textDecoration: "none", color: "#333" }}
            >
              Features
            </a>
            <a
              href="#how-it-works"
              style={{ textDecoration: "none", color: "#333" }}
            >
              How It Works
            </a>
            <a
              href="#use-cases"
              style={{ textDecoration: "none", color: "#333" }}
            >
              Ideas
            </a>
            <a
              href="#pricing"
              style={{ textDecoration: "none", color: "#333" }}
            >
              Pricing
            </a>
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
          >
            {widget.ReactNode}
          </FloatingWidget>
        ))}

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
