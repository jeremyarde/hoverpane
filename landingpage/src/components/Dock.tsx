import React, { useState, useEffect } from "react";

interface DockItem {
  icon: React.ReactNode;
  label: string;
  onClick?: () => void;
}

interface DockProps {
  items: DockItem[];
}

const Dock: React.FC<DockProps> = ({ items }) => {
  const [isVisible, setIsVisible] = useState(true);
  const lastScrollY = React.useRef(window.scrollY);

  useEffect(() => {
    const handleScroll = () => {
      const currentScrollY = window.scrollY;
      console.log(currentScrollY);
      const isScrollingDown = currentScrollY > lastScrollY.current;
      lastScrollY.current = currentScrollY;

      // Show dock when at the top or scrolling up
      if (currentScrollY <= 40 || !isScrollingDown) {
        setIsVisible(true);
      } else {
        setIsVisible(false);
      }
    };

    window.addEventListener("scroll", handleScroll);
    return () => window.removeEventListener("scroll", handleScroll);
  }, []);

  return (
    <div
      className={`fixed bottom-5 left-1/2 -translate-x-1/2 flex gap-2 p-3 px-4 bg-gray-200/80 backdrop-blur-md shadow-xl rounded-2xl shadow-lg z-50 transition-transform duration-300 ${
        isVisible ? "translate-y-0" : "translate-y-32"
      }`}
    >
      {items.map((item, index) => (
        <div
          key={index}
          onClick={item.onClick}
          className="w-12 h-12 flex flex-col items-center cursor-pointer transition-transform duration-200 ease-in-out hover:scale-105"
        >
          <div className="w-10 h-10 flex items-center justify-center rounded-lg bg-white shadow hover:shadow-lg transition-all duration-200">
            {item.icon}
          </div>
          <span className="text-[10px] text-gray-700 mt-1 text-center">
            {item.label}
          </span>
        </div>
      ))}
    </div>
  );
};

export default Dock;
