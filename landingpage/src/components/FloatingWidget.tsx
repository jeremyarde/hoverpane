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
  const livePosition = useRef(initialPosition);

  // Update transform directly
  const updateWidgetTransform = (x: number, y: number) => {
    if (widgetRef.current) {
      widgetRef.current.style.transform = `translate3d(${x}px, ${y}px, 0)`;
    }
  };

  useEffect(() => {
    // Set initial transform
    updateWidgetTransform(position.x, position.y);
  }, []);

  useEffect(() => {
    // Keep transform in sync if position changes from outside
    updateWidgetTransform(position.x, position.y);
    livePosition.current = position;
  }, [position]);

  useEffect(() => {
    const handleScroll = () => {
      if (window.scrollY > 0) {
        // Move to right edge, keeping 20px visible
        const visiblePart = 20;
        const y = livePosition.current.y;
        const x = window.innerWidth - visiblePart;
        updateWidgetTransform(x, y);
        livePosition.current = { x, y };
        setPosition({ x, y });
      } else {
        // Restore original position
        updateWidgetTransform(initialPosition.x, initialPosition.y);
        livePosition.current = initialPosition;
        setPosition(initialPosition);
      }
    };
    window.addEventListener("scroll", handleScroll);
    return () => window.removeEventListener("scroll", handleScroll);
  }, [initialPosition]);

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

  useEffect(() => {
    if (!isDragging) return;
    const handleMouseMove = (e: MouseEvent) => {
      e.preventDefault();
      const newX = e.clientX - dragOffset.x;
      const newY = e.clientY - dragOffset.y;
      updateWidgetTransform(newX, newY);
      livePosition.current = { x: newX, y: newY };
    };
    const handleMouseUp = () => {
      setIsDragging(false);
      // Commit the final position to React state
      setPosition(livePosition.current);
    };
    window.addEventListener("mousemove", handleMouseMove);
    window.addEventListener("mouseup", handleMouseUp);
    return () => {
      window.removeEventListener("mousemove", handleMouseMove);
      window.removeEventListener("mouseup", handleMouseUp);
    };
  }, [isDragging, dragOffset]);

  // Minimize: move to far right, just barely visible
  const handleMinimize = () => {
    const visiblePart = 20;
    const y = livePosition.current.y;
    const x = window.innerWidth - visiblePart;
    updateWidgetTransform(x, y);
    livePosition.current = { x, y };
    setPosition({ x, y });
    onMinimize?.();
  };

  // Maximize: restore to initial position
  const handleMaximize = () => {
    updateWidgetTransform(initialPosition.x, initialPosition.y);
    livePosition.current = initialPosition;
    setPosition(initialPosition);
    onMaximize?.();
  };

  // Convert dragAreaHeight to px for Tailwind spacing
  const dragBarH = typeof dragAreaHeight === "number" ? dragAreaHeight : 30;
  const widgetW = typeof width === "number" ? `${width}px` : width;
  const widgetH = typeof height === "number" ? `${height}px` : height;

  return (
    <div
      ref={widgetRef}
      className={`fixed z-[1000] rounded-lg select-none overflow-hidden shadow-lg bg-white ${
        !isDragging ? "transition-transform duration-300" : ""
      }`}
      style={{
        width: widgetW,
        height: widgetH,
        // left/top removed, handled by transform
      }}
    >
      <div className="flex flex-col h-full">
        <div
          className={`flex w-full items-center bg-gray-100 border-b border-gray-200 rounded-t-lg ${
            isDragging ? "cursor-grabbing" : "cursor-grab"
          }`}
          style={{ height: dragBarH }}
          onMouseDown={handleMouseDown}
        >
          <div className="flex items-center gap-0 pl-2">
            <WindowButton color="#ff5f57" onClick={onClose} />
            <WindowButton color="#febc2e" onClick={handleMinimize} />
            <WindowButton color="#28c840" onClick={handleMaximize} />
          </div>
          <div className="flex items-center justify-center flex-1">
            <div className="text-sm font-medium p-2 mr-[70px]">{title}</div>
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
