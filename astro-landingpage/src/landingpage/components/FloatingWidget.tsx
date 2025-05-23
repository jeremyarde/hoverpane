import React, { useState, useRef, useEffect } from "react";

interface FloatingWidgetProps {
  initialPosition: { x: number; y: number };
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
  initialPosition,
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
  const [position, setPosition] = useState<{ x: number; y: number }>(
    initialPosition ?? { x: 100, y: 100 }
  );
  const [isDragging, setIsDragging] = useState(false);
  const widgetRef = useRef<HTMLDivElement>(null);
  const startPos = useRef<{ x: number; y: number }>({ x: 0, y: 0 });
  const startMousePos = useRef<{ x: number; y: number }>({ x: 0, y: 0 });
  const lastScrollY = useRef(window.scrollY);

  useEffect(() => {
    const handleScroll = () => {
      const currentScrollY = window.scrollY;
      const isScrollingDown = currentScrollY > lastScrollY.current;
      lastScrollY.current = currentScrollY;

      if (widgetRef.current) {
        const rect = widgetRef.current.getBoundingClientRect();
        // Check if widget is visible in viewport and we've scrolled past 30% of the viewport
        const isVisible = rect.top < window.innerHeight && rect.bottom > 0;
        const isAtRightEdge =
          Math.abs(position.x - (window.innerWidth - 20)) < 1;
        const hasScrolledEnough = currentScrollY > window.innerHeight * 0.3;

        if (
          isScrollingDown &&
          isVisible &&
          !isAtRightEdge &&
          hasScrolledEnough
        ) {
          // Move to right edge, keeping 20px visible
          const visiblePart = 40;
          setPosition((prev) => ({
            ...prev,
            x: window.innerWidth - visiblePart,
          }));
        } else if (!isScrollingDown && currentScrollY === 0) {
          // Restore original position when at the top
          setPosition(initialPosition);
        }
      }
    };

    window.addEventListener("scroll", handleScroll);
    return () => window.removeEventListener("scroll", handleScroll);
  }, [initialPosition, position.x, title]);

  // Add a resize handler to update positions when window is resized
  useEffect(() => {
    const handleResize = () => {
      if (widgetRef.current) {
        const rect = widgetRef.current.getBoundingClientRect();
        const isVisible = rect.top < window.innerHeight && rect.bottom > 0;
        const hasScrolledEnough = window.scrollY > window.innerHeight * 0.3;

        if (isVisible && hasScrolledEnough) {
          const visiblePart = 20;
          setPosition((prev) => ({
            ...prev,
            x: window.innerWidth - visiblePart,
          }));
        } else {
          setPosition(initialPosition);
        }
      }
    };

    window.addEventListener("resize", handleResize);
    return () => window.removeEventListener("resize", handleResize);
  }, [initialPosition, title]);

  const handleMouseDown = (e: React.MouseEvent) => {
    e.preventDefault();
    if (widgetRef.current) {
      // Store the current position as the starting point
      startPos.current = { ...position };
      startMousePos.current = { x: e.clientX, y: e.clientY };
      setIsDragging(true);

      // Initialize the CSS custom properties with current position
      widgetRef.current.style.setProperty("--translate-x", `${position.x}px`);
      widgetRef.current.style.setProperty("--translate-y", `${position.y}px`);
    }
  };

  useEffect(() => {
    if (!isDragging) return;

    const handleMouseMove = (e: MouseEvent) => {
      e.preventDefault();
      if (widgetRef.current) {
        const dx = e.clientX - startMousePos.current.x;
        const dy = e.clientY - startMousePos.current.y;
        const newX = startPos.current.x + dx;
        const newY = startPos.current.y + dy;

        // Update CSS custom properties
        widgetRef.current.style.setProperty("--translate-x", `${newX}px`);
        widgetRef.current.style.setProperty("--translate-y", `${newY}px`);
      }
    };

    const handleMouseUp = (e: MouseEvent) => {
      if (widgetRef.current) {
        const dx = e.clientX - startMousePos.current.x;
        const dy = e.clientY - startMousePos.current.y;
        setPosition({
          x: startPos.current.x + dx,
          y: startPos.current.y + dy,
        });
      }
      setIsDragging(false);
    };

    window.addEventListener("mousemove", handleMouseMove);
    window.addEventListener("mouseup", handleMouseUp);
    return () => {
      window.removeEventListener("mousemove", handleMouseMove);
      window.removeEventListener("mouseup", handleMouseUp);
    };
  }, [isDragging]);

  // Minimize: move to far right, just barely visible
  const handleMinimize = () => {
    const visiblePart = 20;
    setPosition((prev) => ({ ...prev, x: window.innerWidth - visiblePart }));
    onMinimize?.();
  };

  // Maximize: restore to initial position
  const handleMaximize = () => {
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
        transform: isDragging
          ? "translate3d(var(--translate-x), var(--translate-y), 0)"
          : `translate3d(${position.x}px, ${position.y}px, 0)`,
        willChange: "transform",
        touchAction: "none",
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
