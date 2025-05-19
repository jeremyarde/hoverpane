import React from "react";

interface DockItem {
  icon: React.ReactNode;
  label: string;
  onClick?: () => void;
}

interface DockProps {
  items: DockItem[];
}

const Dock: React.FC<DockProps> = ({ items }) => {
  return (
    <div className="fixed bottom-5 left-1/2 -translate-x-1/2 flex gap-2 p-4 px-4 bg-white/80 backdrop-blur-md rounded-2xl shadow-lg z-50">
      {items.map((item, index) => (
        <div
          key={index}
          onClick={item.onClick}
          className="w-12 h-12 flex flex-col items-center cursor-pointer transition-transform duration-200 ease-in-out hover:scale-120"
        >
          <div className="w-10 h-10">{item.icon}</div>
          <span className="text-[10px] text-gray-700 mt-1 text-center">
            {item.label}
          </span>
        </div>
      ))}
    </div>
  );
};

export default Dock;
