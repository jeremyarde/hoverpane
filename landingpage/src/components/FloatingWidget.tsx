import React, { useState, useRef, useEffect } from "react";

interface FloatingWidgetProps {
  initialPosition?: { x: number; y: number };
  iframeSrc?: string;
  dragAreaHeight?: number;
  width?: number | string;
  height?: number | string;
  title?: string;
  children?: React.ReactNode;
  onClose?: () => void;
  onMinimize?: () => void;
  onMaximize?: () => void;
}

const WindowButton = ({
  color,
  onClick,
}: {
  color: string;
  onClick?: () => void;
}) => (
  <button
    type="button"
    onClick={(e) => {
      e.stopPropagation();
      onClick?.();
    }}
    className="w-3 h-3 rounded-full border border-gray-200 mr-1.5 focus:outline-none"
    style={{ backgroundColor: color }}
    tabIndex={-1}
  />
);

const FloatingWidget: React.FC<FloatingWidgetProps> = ({
  initialPosition = { x: 100, y: 100 },
  iframeSrc,
  dragAreaHeight = 30,
  width = 300,
  height = "auto",
  title,
  children,
  onClose,
  onMinimize,
  onMaximize,
}) => {
  const [position, setPosition] = useState(initialPosition);
  const [isDragging, setIsDragging] = useState(false);
  const [dragOffset, setDragOffset] = useState({ x: 0, y: 0 });
  const widgetRef = useRef<HTMLDivElement>(null);

  const handleMouseDown = (e: React.MouseEvent) => {
    e.preventDefault();
    if (widgetRef.current) {
      const rect = widgetRef.current.getBoundingClientRect();
      setDragOffset({
        x: e.clientX - rect.left,
        y: e.clientY - rect.top,
      });
      setIsDragging(true);
    }
  };

  const handleMouseMove = (e: MouseEvent) => {
    if (isDragging) {
      e.preventDefault();
      setPosition({
        x: e.clientX - dragOffset.x,
        y: e.clientY - dragOffset.y,
      });
    }
  };

  const handleMouseUp = () => setIsDragging(false);

  useEffect(() => {
    if (isDragging) {
      window.addEventListener("mousemove", handleMouseMove);
      window.addEventListener("mouseup", handleMouseUp);
    }
    return () => {
      window.removeEventListener("mousemove", handleMouseMove);
      window.removeEventListener("mouseup", handleMouseUp);
    };
  }, [isDragging, dragOffset]);

  // Convert dragAreaHeight to px for Tailwind spacing
  const dragBarH = typeof dragAreaHeight === "number" ? dragAreaHeight : 30;
  const widgetW = typeof width === "number" ? `${width}px` : width;
  const widgetH = typeof height === "number" ? `${height}px` : height;

  return (
    <div
      ref={widgetRef}
      className="fixed z-[1000] rounded-lg select-none overflow-hidden shadow-lg bg-white"
      style={{
        left: position.x,
        top: position.y,
        width: widgetW,
        height: widgetH,
      }}
    >
      <div className="flex flex-col h-full">
        <div
          className={`flex items-center bg-gray-100 border-b border-gray-200 rounded-t-lg px-2 ${
            isDragging ? "cursor-grabbing" : "cursor-grab"
          }`}
          style={{ height: dragBarH }}
          onMouseDown={handleMouseDown}
        >
          <div className="flex items-center">
            <WindowButton color="#ff5f57" onClick={onClose} />
            <WindowButton color="#febc2e" onClick={onMinimize} />
            <WindowButton color="#28c840" onClick={onMaximize} />
          </div>
          <div className="flex items-center justify-center flex-1">
            <div className="text-sm font-medium p-2 mr-[60px]">{title}</div>
          </div>
        </div>
        {iframeSrc ? (
          <iframe
            src={iframeSrc}
            className="flex-1 w-full border-0 bg-transparent block"
            title="Floating Widget Iframe"
          />
        ) : (
          <div className="flex-1 w-full overflow-auto bg-white">{children}</div>
        )}
      </div>
    </div>
  );
};

export default FloatingWidget;
