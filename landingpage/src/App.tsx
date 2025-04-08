/* eslint-disable @typescript-eslint/no-unused-vars */

import React from "react";
import { MeshGradient } from "@paper-design/shaders-react";

// Placeholder Icons (replace with actual icons or component library)
const IconBolt = () => (
  <svg
    className="w-8 h-8 text-indigo-600"
    fill="none"
    stroke="currentColor"
    viewBox="0 0 24 24"
    xmlns="http://www.w3.org/2000/svg"
  >
    <path
      strokeLinecap="round"
      strokeLinejoin="round"
      strokeWidth="2"
      d="M13 10V3L4 14h7v7l9-11h-7z"
    ></path>
  </svg>
);
const IconCode = () => (
  <svg
    className="w-8 h-8 text-emerald-600"
    fill="none"
    stroke="currentColor"
    viewBox="0 0 24 24"
    xmlns="http://www.w3.org/2000/svg"
  >
    <path
      strokeLinecap="round"
      strokeLinejoin="round"
      strokeWidth="2"
      d="M10 20l4-16m4 4l4 4-4 4M6 16l-4-4 4-4"
    ></path>
  </svg>
);
const IconCog = () => (
  <svg
    className="w-8 h-8 text-rose-600"
    fill="none"
    stroke="currentColor"
    viewBox="0 0 24 24"
    xmlns="http://www.w3.org/2000/svg"
  >
    <path
      strokeLinecap="round"
      strokeLinejoin="round"
      strokeWidth="2"
      d="M10.325 4.317c.426-1.756 2.924-1.756 3.35 0a1.724 1.724 0 002.573 1.066c1.543-.94 3.31.826 2.37 2.37a1.724 1.724 0 001.065 2.572c1.756.426 1.756 2.924 0 3.35a1.724 1.724 0 00-1.066 2.573c.94 1.543-.826 3.31-2.37 2.37a1.724 1.724 0 00-2.572 1.065c-.426 1.756-2.924 1.756-3.35 0a1.724 1.724 0 00-2.573-1.066c-1.543.94-3.31-.826-2.37-2.37a1.724 1.724 0 00-1.065-2.572c-1.756-.426-1.756-2.924 0-3.35a1.724 1.724 0 001.066-2.573c-.94-1.543.826-3.31 2.37-2.37.996.608 2.296.07 2.572-1.065z"
    ></path>
    <path
      strokeLinecap="round"
      strokeLinejoin="round"
      strokeWidth="2"
      d="M15 12a3 3 0 11-6 0 3 3 0 016 0z"
    ></path>
  </svg>
);
const IconEye = () => (
  <svg
    className="w-8 h-8 text-sky-600"
    fill="none"
    stroke="currentColor"
    viewBox="0 0 24 24"
    xmlns="http://www.w3.org/2000/svg"
  >
    <path
      strokeLinecap="round"
      strokeLinejoin="round"
      strokeWidth={2}
      d="M15 12a3 3 0 11-6 0 3 3 0 016 0z"
    />
    <path
      strokeLinecap="round"
      strokeLinejoin="round"
      strokeWidth={2}
      d="M2.458 12C3.732 7.943 7.523 5 12 5c4.478 0 8.268 2.943 9.542 7-1.274 4.057-5.064 7-9.542 7-4.477 0-8.268-2.943-9.542-7z"
    />
  </svg>
);
const IconRefresh = () => (
  <svg
    className="w-8 h-8 text-amber-600"
    fill="none"
    stroke="currentColor"
    viewBox="0 0 24 24"
    xmlns="http://www.w3.org/2000/svg"
  >
    <path
      strokeLinecap="round"
      strokeLinejoin="round"
      strokeWidth={2}
      d="M4 4v5h5M20 20v-5h-5M4 4l5 5M20 20l-5-5M4 4h16v16H4z"
    />
  </svg>
);
const IconCheck = () => (
  <svg
    className="w-5 h-5 text-emerald-500 mr-2 flex-shrink-0"
    fill="none"
    stroke="currentColor"
    viewBox="0 0 24 24"
    xmlns="http://www.w3.org/2000/svg"
  >
    <path
      strokeLinecap="round"
      strokeLinejoin="round"
      strokeWidth={2}
      d="M5 13l4 4L19 7"
    />
  </svg>
);

const App: React.FC = () => {
  return (
    <div className="bg-slate-50 text-slate-800 font-sans">
      {/* Navbar */}
      <nav className="sticky top-0 z-50 bg-white/95 backdrop-blur-sm shadow-sm border-b border-slate-200/75">
        <div className="max-w-6xl mx-auto px-4">
          <div className="flex justify-between items-center h-16">
            <span className="text-2xl font-bold text-indigo-600">
              Widget Maker
            </span>
            <div className="hidden md:flex space-x-6 items-center">
              <a
                href="#features"
                className="text-slate-600 hover:text-indigo-600 transition duration-300"
              >
                Features
              </a>
              <a
                href="#how-it-works"
                className="text-slate-600 hover:text-indigo-600 transition duration-300"
              >
                How It Works
              </a>
              <a
                href="#use-cases"
                className="text-slate-600 hover:text-indigo-600 transition duration-300"
              >
                Ideas
              </a>
              <a
                href="#pricing"
                className="text-slate-600 hover:text-indigo-600 transition duration-300"
              >
                Pricing
              </a>
              <a
                href="#download"
                className="bg-indigo-600 hover:bg-indigo-700 text-white font-semibold py-2 px-4 rounded-lg transition duration-300 focus:outline-none focus:ring-2 focus:ring-indigo-500 focus:ring-offset-2"
              >
                Download
              </a>
            </div>
            {/* TODO: Add mobile menu button here */}
          </div>
        </div>
      </nav>
      {/* Hero Section */}
      <header className="relative bg-indigo-100 py-24 md:py-36 text-center px-4 overflow-hidden">
        {/* Absolutely positioned background shapes - No Circles or Rotations */}
        <div className="absolute top-0 left-0 -translate-x-1/3 -translate-y-1/3 w-64 h-48 bg-indigo-200 rounded-xl opacity-30"></div>{" "}
        {/* Rectangle */}
        <div className="absolute bottom-0 right-0 translate-x-1/4 translate-y-1/4 w-80 h-64 bg-sky-200 rounded-xl opacity-20"></div>{" "}
        {/* Rectangle */}
        <div className="absolute top-1/4 right-10 w-20 h-20 bg-indigo-300 rounded-lg opacity-40"></div>{" "}
        {/* Square */}
        <div className="absolute bottom-10 left-10 w-32 h-32 bg-sky-300 rounded-xl opacity-30"></div>{" "}
        {/* Square */}
        {/* Added Shapes */}
        <div className="absolute top-1/2 left-1/4 -translate-y-1/2 w-24 h-48 bg-indigo-200 rounded-lg opacity-20"></div>{" "}
        {/* Tall Rectangle */}
        <div className="absolute bottom-1/4 right-1/3 w-40 h-40 bg-sky-200 rounded-xl opacity-30"></div>{" "}
        {/* Square */}
        <div className="absolute top-10 right-1/3 w-16 h-16 bg-indigo-300 rounded-md opacity-35"></div>{" "}
        {/* Small Square */}
        {/* Content container with higher z-index */}
        <div className="relative z-10">
          <h1 className="text-4xl md:text-6xl font-bold text-slate-900 mb-4">
            Widget Maker
          </h1>
          <p className="text-lg md:text-xl text-slate-700 max-w-2xl mx-auto mb-8 leading-relaxed">
            Turn websites or your own code into handy desktop widgets. Get the
            info you need, right when you need it, without touching your
            browser.
          </p>
          <a
            href="#download"
            className="inline-block bg-indigo-600 hover:bg-indigo-700 text-white font-bold py-3 px-8 rounded-lg transition duration-300 ease-in-out transform hover:scale-105 focus:outline-none focus:ring-2 focus:ring-indigo-500 focus:ring-offset-2"
          >
            Download
          </a>
        </div>
      </header>

      {/* Detailed Features Section */}
      <section id="features" className="py-20 md:py-28 bg-slate-50 px-4">
        <div className="max-w-6xl mx-auto">
          <h2 className="text-3xl font-bold text-center text-slate-800 mb-12 md:mb-16">
            Everything You Need
          </h2>
          <div className="grid md:grid-cols-2 lg:grid-cols-3 gap-8 md:gap-10">
            {/* Feature 1 */}
            <div className="feature-item text-center p-6 bg-white border border-slate-200 rounded-xl shadow-sm hover:shadow-md transition-shadow">
              <div className="bg-indigo-100 rounded-full p-4 inline-block mb-4">
                <IconBolt />
              </div>
              <h3 className="text-xl font-semibold text-slate-700 mb-2">
                Live Web Content
              </h3>
              <p className="text-slate-500 leading-relaxed">
                Snag just the important bits (prices, weather, news) from any
                website. See updates live on your desktop, ditch the tab
                clutter.
              </p>
            </div>
            {/* Feature 2 */}
            <div className="feature-item text-center p-6 bg-white border border-slate-200 rounded-xl shadow-sm hover:shadow-md transition-shadow">
              <div className="bg-emerald-100 rounded-full p-4 inline-block mb-4">
                <IconCode />
              </div>
              <h3 className="text-xl font-semibold text-slate-700 mb-2">
                Custom HTML Widgets
              </h3>
              <p className="text-slate-500 leading-relaxed">
                Know HTML, CSS, and JavaScript? Build whatever you need –
                dashboards, quick tools, data displays, and more.
              </p>
            </div>
            {/* Feature 3 */}
            <div className="feature-item text-center p-6 bg-white border border-slate-200 rounded-xl shadow-sm hover:shadow-md transition-shadow">
              <div className="bg-rose-100 rounded-full p-4 inline-block mb-4">
                <IconCog />
              </div>
              <h3 className="text-xl font-semibold text-slate-700 mb-2">
                Control the Details
              </h3>
              <p className="text-slate-500 leading-relaxed">
                Set auto-refresh timers so data stays fresh. Use CSS selectors
                to pinpoint exactly what you want from web widgets.
              </p>
            </div>
            {/* Feature 4 */}
            <div className="feature-item text-center p-6 bg-white border border-slate-200 rounded-xl shadow-sm hover:shadow-md transition-shadow">
              <div className="bg-sky-100 rounded-full p-4 inline-block mb-4">
                <IconEye />
              </div>
              <h3 className="text-xl font-semibold text-slate-700 mb-2">
                Look and Feel
              </h3>
              <p className="text-slate-500 leading-relaxed">
                Make widgets see-through to blend in. Choose if they float on
                top, stay put, or hide behind other windows.
              </p>
            </div>
            {/* Feature 5 */}
            <div className="feature-item text-center p-6 bg-white border border-slate-200 rounded-xl shadow-sm hover:shadow-md transition-shadow">
              <div className="bg-amber-100 rounded-full p-4 inline-block mb-4">
                <IconRefresh />
              </div>
              <h3 className="text-xl font-semibold text-slate-700 mb-2">
                Lightweight & Fast
              </h3>
              <p className="text-slate-500 leading-relaxed">
                Built efficiently with Rust. Runs light on resources, unlike
                heavier browser-based tools.
              </p>
            </div>
          </div>
        </div>
      </section>
      {/* How It Works Section */}
      <section id="how-it-works" className="py-20 md:py-28 bg-white px-4">
        <div className="max-w-4xl mx-auto text-center">
          <h2 className="text-3xl font-bold text-slate-800 mb-12 md:mb-16">
            Easy Setup
          </h2>
          <div className="grid md:grid-cols-3 gap-8 md:gap-12 relative">
            {/* Dashed line connector (visual only) */}
            <div className="hidden md:block absolute top-1/2 left-0 w-full h-px border-t-2 border-dashed border-indigo-200 -translate-y-1/2"></div>

            {/* Step 1 */}
            <div className="relative z-10">
              <div className="bg-indigo-600 text-white rounded-full w-12 h-12 flex items-center justify-center text-xl font-bold mx-auto mb-4 shadow-lg">
                1
              </div>
              <h3 className="text-lg font-semibold text-slate-700 mb-2">
                Pick Your Source
              </h3>
              <p className="text-slate-500 leading-relaxed">
                Give it a website address or your own HTML code.
              </p>
            </div>
            {/* Step 2 */}
            <div className="relative z-10">
              <div className="bg-indigo-600 text-white rounded-full w-12 h-12 flex items-center justify-center text-xl font-bold mx-auto mb-4 shadow-lg">
                2
              </div>
              <h3 className="text-lg font-semibold text-slate-700 mb-2">
                Tweak (If You Want)
              </h3>
              <p className="text-slate-500 leading-relaxed">
                Set how often it refreshes or target specific web content.
              </p>
            </div>
            {/* Step 3 */}
            <div className="relative z-10">
              <div className="bg-indigo-600 text-white rounded-full w-12 h-12 flex items-center justify-center text-xl font-bold mx-auto mb-4 shadow-lg">
                3
              </div>
              <h3 className="text-lg font-semibold text-slate-700 mb-2">
                Launch It
              </h3>
              <p className="text-slate-500 leading-relaxed">
                That's it! Your widget is ready on your desktop.
              </p>
            </div>
          </div>
        </div>
      </section>
      {/* Use Cases Section */}
      <section id="use-cases" className="py-20 md:py-28 bg-slate-50 px-4">
        <div className="max-w-6xl mx-auto">
          <h2 className="text-3xl font-bold text-center text-slate-800 mb-12 md:mb-16">
            What Will You Build?
          </h2>
          <div className="grid md:grid-cols-2 gap-8">
            {/* Use Case 1 */}
            <div className="bg-white rounded-xl p-6 flex items-start space-x-4 border border-slate-100 shadow-sm">
              {/* Icon placeholder */}
              <div className="mt-1 flex-shrink-0 w-10 h-10 bg-indigo-100 rounded-full flex items-center justify-center">
                <span className="text-indigo-600 text-xl">📈</span>
              </div>
              <div>
                <h3 className="text-lg font-semibold text-slate-700 mb-1">
                  Finance
                </h3>
                <p className="text-slate-500 leading-relaxed">
                  Keep an eye on stocks, crypto, or exchange rates without
                  needing a full browser window.
                </p>
              </div>
            </div>
            {/* Use Case 2 */}
            <div className="bg-white rounded-xl p-6 flex items-start space-x-4 border border-slate-100 shadow-sm">
              <div className="mt-1 flex-shrink-0 w-10 h-10 bg-emerald-100 rounded-full flex items-center justify-center">
                <span className="text-emerald-600 text-xl">📰</span>
              </div>
              <div>
                <h3 className="text-lg font-semibold text-slate-700 mb-1">
                  News
                </h3>
                <p className="text-slate-500 leading-relaxed">
                  See the latest headlines from your favorite sources, updated
                  automatically.
                </p>
              </div>
            </div>
            {/* Use Case 3 */}
            <div className="bg-white rounded-xl p-6 flex items-start space-x-4 border border-slate-100 shadow-sm">
              <div className="mt-1 flex-shrink-0 w-10 h-10 bg-amber-100 rounded-full flex items-center justify-center">
                <span className="text-amber-600 text-xl">☁️</span>
              </div>
              <div>
                <h3 className="text-lg font-semibold text-slate-700 mb-1">
                  Weather
                </h3>
                <p className="text-slate-500 leading-relaxed">
                  Get the current conditions or forecast right on your desktop.
                </p>
              </div>
            </div>
            {/* Use Case 4 */}
            <div className="bg-white rounded-xl p-6 flex items-start space-x-4 border border-slate-100 shadow-sm">
              <div className="mt-1 flex-shrink-0 w-10 h-10 bg-rose-100 rounded-full flex items-center justify-center">
                <span className="text-rose-600 text-xl">🛠️</span>
              </div>
              <div>
                <h3 className="text-lg font-semibold text-slate-700 mb-1">
                  Custom Tools
                </h3>
                <p className="text-slate-500 leading-relaxed">
                  Build quick tools, display data from APIs, monitor system
                  stats – the possibilities are open.
                </p>
              </div>
            </div>
          </div>
        </div>
      </section>
      {/* Pricing Section */}
      <section id="pricing" className="py-20 md:py-28 bg-white px-4">
        <div className="max-w-4xl mx-auto text-center">
          <h2 className="text-3xl font-bold text-slate-800 mb-4">Pricing</h2>
          <p className="text-slate-600 mb-12 md:mb-16 max-w-xl mx-auto leading-relaxed">
            Simple and straightforward. Try it free, then upgrade if you need
            more.
          </p>

          <div className="grid md:grid-cols-2 gap-8 max-w-2xl mx-auto">
            {/* Free Tier */}
            <div className="border border-slate-200 rounded-xl p-6 md:p-8 flex flex-col">
              <h3 className="text-xl font-semibold mb-2">Free</h3>
              <p className="text-4xl font-bold mb-4">$0</p>
              <p className="text-slate-500 mb-6 flex-grow">
                Perfect for personal use.
              </p>
              <ul className="space-y-2 text-left mb-8 text-slate-600">
                <li className="flex items-center">
                  <IconCheck /> Up to 3 active widgets
                </li>
                <li className="flex items-center">
                  <IconCheck /> Basic modifiers (Refresh)
                </li>
                <li className="flex items-center">
                  <IconCheck /> Community support
                </li>
              </ul>
              <a
                href="#download"
                className="mt-auto w-full block text-center bg-white hover:bg-slate-100 border border-slate-300 text-slate-800 font-semibold py-2 px-4 rounded-lg transition duration-300 focus:outline-none focus:ring-2 focus:ring-indigo-500 focus:ring-offset-1"
              >
                Download Free
              </a>
            </div>

            {/* Pro Tier */}
            <div className="border-2 border-indigo-500 rounded-xl p-6 md:p-8 flex flex-col relative bg-white shadow-lg">
              <span className="absolute top-0 left-1/2 -translate-x-1/2 -translate-y-1/2 bg-indigo-500 text-white text-xs font-bold px-3 py-1 rounded-full">
                Most Popular
              </span>
              <h3 className="text-xl font-semibold mb-2 text-indigo-600">
                Pro
              </h3>
              <p className="text-4xl font-bold mb-4">
                $19{" "}
                <span className="text-lg font-normal text-slate-500">
                  / one-time
                </span>
              </p>
              <p className="text-slate-500 mb-6 flex-grow">
                For those who want it all.
              </p>
              <ul className="space-y-2 text-left mb-8 text-slate-600">
                <li className="flex items-center">
                  <IconCheck /> Unlimited widgets
                </li>
                <li className="flex items-center">
                  <IconCheck /> All modifiers (Scrape, etc.)
                </li>
                <li className="flex items-center">
                  <IconCheck /> Advanced options
                </li>
                <li className="flex items-center">
                  <IconCheck /> Priority support
                </li>
                <li className="flex items-center">
                  <IconCheck /> Lifetime updates
                </li>
              </ul>
              <a
                href="#"
                className="mt-auto w-full block text-center bg-indigo-600 hover:bg-indigo-700 text-white font-semibold py-2 px-4 rounded-lg transition duration-300 focus:outline-none focus:ring-2 focus:ring-indigo-500 focus:ring-offset-2"
              >
                Go Pro (Placeholder)
              </a>
            </div>
          </div>
        </div>
      </section>
      {/* Download Section (Footer) */}
      <footer
        id="download"
        className="bg-slate-800 text-slate-300 py-16 md:py-20 text-center px-4"
      >
        <h2 className="text-3xl font-bold text-white mb-4">
          Try Widget Maker Now
        </h2>
        <p className="text-lg text-slate-400 max-w-xl mx-auto mb-8 leading-relaxed">
          It's free to get started. See how you can bring useful info right to
          your desktop.
        </p>
        <a
          href="#"
          className="inline-block bg-indigo-600 hover:bg-indigo-700 text-white font-bold py-3 px-8 rounded-lg transition duration-300 ease-in-out transform hover:scale-105 focus:outline-none focus:ring-2 focus:ring-indigo-500 focus:ring-offset-2 ring-offset-slate-800"
        >
          Download for macOS (Placeholder)
        </a>
        {/* Add links for other OS if applicable */}
        <p className="text-xs text-slate-500 mt-12">
          © {new Date().getFullYear()} Widget Maker. All rights reserved.
        </p>
      </footer>
    </div>
  );
};

export default App;
