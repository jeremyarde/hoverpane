import React from "react";
import "../styles/global.css";
import {
  IconMinimalist,
  IconPin,
  IconRefresh,
  IconCode,
  IconCheck,
  HOVERPANE_DOWNLOAD_URL,
} from "./constants";
import wallpaperGaus from "../assets/wallpaper-gaus.png";
import { handleDownload } from "../utils";

const LandingPage: React.FC = () => {
  return (
    <div className="min-h-screen bg-slate-50">
      {/* Hero Section */}
      <header className="overflow-hidden relative px-4 py-24 text-center bg-gradient-to-br from-indigo-600 to-indigo-400 md:py-36">
        {/* <img
          src={wallpaperGaus.src}
          alt="wallpaper"
          className="absolute top-0 left-0 w-full h-full"
        /> */}
        <div className="gradient-background">
          <img
            src={wallpaperGaus.src}
            alt="wallpaper"
            className="absolute top-0 left-0 w-full h-full"
          />
        </div>
        {/* <div className="absolute top-0 left-0 w-64 h-48 bg-indigo-200 rounded-xl opacity-30 -translate-x-1/3 -translate-y-1/3"></div>
        <div className="absolute right-0 bottom-0 w-80 h-64 bg-indigo-200 rounded-xl opacity-30 translate-x-1/4 translate-y-1/4"></div> */}
        <div className="relative z-10">
          <h1 className="mb-4 text-4xl font-bold text-white md:text-6xl">
            HoverPane
          </h1>
          <p className="mx-auto mb-8 max-w-2xl text-lg text-slate-100 md:text-xl">
            Transform any website into a sleek desktop widget in seconds. Keep
            your favorite content always visible and easily accessible.
          </p>
          <button
            onClick={handleDownload}
            className="inline-block px-8 py-3 font-bold text-indigo-600 bg-white rounded-lg transition-all hover:scale-105 hover:bg-slate-100"
          >
            Get Started
          </button>
          <div className="overflow-hidden mx-auto mt-12 max-w-4xl rounded-xl shadow-2xl bg-slate-800">
            <video
              className="object-cover w-full aspect-video"
              autoPlay
              loop
              muted
              playsInline
            >
              <source src="/example1.webm" type="video/webm" />
              Your browser does not support the video tag.
            </video>
          </div>
        </div>
      </header>

      {/* Features Section */}
      <section id="features" className="px-4 py-20 bg-slate-50">
        <div className="mx-auto max-w-8xl">
          <h2 className="mb-12 text-3xl font-bold text-center text-slate-800">
            Why use HoverPane?
          </h2>
          <div className="grid gap-8 md:grid-cols-2 md:gap-10">
            <div className="p-6 text-center bg-white rounded-xl border shadow-sm transition-shadow border-slate-200 hover:shadow-md">
              <div className="flex justify-center items-center mx-auto mb-4 w-12 h-12 bg-indigo-100 rounded-full">
                <IconMinimalist className="w-8 h-8 text-indigo-300" />
              </div>
              <h3 className="mb-2 text-xl font-semibold text-slate-700">
                Clean & Focused
              </h3>
              <p className="text-slate-500">
                Experience your content without distractions. No ads, no
                toolbars, just what matters to you.
              </p>
            </div>
            <div className="p-6 text-center bg-white rounded-xl border shadow-sm transition-shadow border-slate-200 hover:shadow-md">
              <div className="flex justify-center items-center mx-auto mb-4 w-12 h-12 bg-sky-100 rounded-full">
                <IconPin className="w-8 h-8 text-sky-300" />
              </div>
              <h3 className="mb-2 text-xl font-semibold text-slate-700">
                Always Visible
              </h3>
              <p className="text-slate-500">
                Keep important information at your fingertips. Your widgets stay
                on top of other windows, exactly where you need them.
              </p>
            </div>
            <div className="p-6 text-center bg-white rounded-xl border shadow-sm transition-shadow border-slate-200 hover:shadow-md">
              <div className="flex justify-center items-center mx-auto mb-4 w-12 h-12 bg-amber-100 rounded-full">
                <IconRefresh className="w-8 h-8 text-amber-300" />
              </div>
              <h3 className="mb-2 text-xl font-semibold text-slate-700">
                Stay Updated
              </h3>
              <p className="text-slate-500">
                Never miss an update. Set custom refresh intervals to keep your
                information current.
              </p>
            </div>
            <div className="p-6 text-center bg-white rounded-xl border shadow-sm transition-shadow border-slate-200 hover:shadow-md">
              <div className="flex justify-center items-center mx-auto mb-4 w-12 h-12 bg-emerald-100 rounded-full">
                <IconCode className="w-8 h-8 text-emerald-500" />
              </div>
              <h3 className="mb-2 text-xl font-semibold text-slate-700">
                Customizable
              </h3>
              <p className="text-slate-500">
                Create your perfect widget. Use any website or build your own
                with HTML, CSS, and JavaScript.
              </p>
            </div>
          </div>
        </div>
      </section>

      {/* How It Works Section */}
      <section id="how-it-works" className="px-4 py-20 bg-white">
        <div className="mx-auto max-w-8xl">
          <h2 className="mb-12 text-3xl font-bold text-center text-slate-800">
            Simple Setup
          </h2>
          <div className="grid gap-8 mx-auto max-w-4xl md:grid-cols-3 md:gap-12">
            <div className="text-center">
              <div className="flex justify-center items-center mx-auto mb-4 w-12 h-12 text-xl font-bold text-white bg-indigo-600 rounded-full shadow-md">
                1
              </div>
              <h3 className="mb-2 text-lg font-semibold text-slate-700">
                Choose Your Source
              </h3>
              <p className="text-slate-500">
                Enter a website URL or paste your custom HTML code.
              </p>
            </div>
            <div className="text-center">
              <div className="flex justify-center items-center mx-auto mb-4 w-12 h-12 text-xl font-bold text-white bg-indigo-600 rounded-full shadow-md">
                2
              </div>
              <h3 className="mb-2 text-lg font-semibold text-slate-700">
                Customize
              </h3>
              <p className="text-slate-500">
                Set refresh rates, adjust size, and configure other settings to
                match your needs.
              </p>
            </div>
            <div className="text-center">
              <div className="flex justify-center items-center mx-auto mb-4 w-12 h-12 text-xl font-bold text-white bg-indigo-600 rounded-full shadow-md">
                3
              </div>
              <h3 className="mb-2 text-lg font-semibold text-slate-700">
                Enjoy
              </h3>
              <p className="text-slate-500">
                Your widget is ready! Place it anywhere on your desktop and
                start using it right away.
              </p>
            </div>
          </div>
        </div>
      </section>

      {/* Use Cases Section */}
      <section id="use-cases" className="px-4 py-20 bg-slate-50">
        <div className="mx-auto max-w-8xl">
          <h2 className="mb-12 text-3xl font-bold text-center text-slate-800">
            Endless Possibilities
          </h2>
          <div className="grid gap-6 md:grid-cols-2">
            <div className="flex gap-4 items-start p-6 bg-white rounded-xl border shadow-sm border-slate-200">
              <div className="flex justify-center items-center w-10 h-10 text-2xl bg-indigo-100 rounded-full shrink-0">
                📈
              </div>
              <div>
                <h3 className="mb-1 text-lg font-semibold text-slate-700">
                  Market Watch
                </h3>
                <p className="text-slate-500">
                  Monitor stocks, crypto, or market trends in real-time.
                </p>
              </div>
            </div>
            <div className="flex gap-4 items-start p-6 bg-white rounded-xl border shadow-sm border-slate-200">
              <div className="flex justify-center items-center w-10 h-10 text-2xl bg-indigo-100 rounded-full shrink-0">
                🌤️
              </div>
              <div>
                <h3 className="mb-1 text-lg font-semibold text-slate-700">
                  Weather Updates
                </h3>
                <p className="text-slate-500">
                  Keep an eye on current conditions and forecasts.
                </p>
              </div>
            </div>
            <div className="flex gap-4 items-start p-6 bg-white rounded-xl border shadow-sm border-slate-200">
              <div className="flex justify-center items-center w-10 h-10 text-2xl bg-indigo-100 rounded-full shrink-0">
                📅
              </div>
              <div>
                <h3 className="mb-1 text-lg font-semibold text-slate-700">
                  Calendar View
                </h3>
                <p className="text-slate-500">
                  Access your schedule without opening your browser.
                </p>
              </div>
            </div>
            <div className="flex gap-4 items-start p-6 bg-white rounded-xl border shadow-sm border-slate-200">
              <div className="flex justify-center items-center w-10 h-10 text-2xl bg-indigo-100 rounded-full shrink-0">
                ⚡
              </div>
              <div>
                <h3 className="mb-1 text-lg font-semibold text-slate-700">
                  Quick Tools
                </h3>
                <p className="text-slate-500">
                  Build custom utilities, API dashboards, or system monitors.
                </p>
              </div>
            </div>
            <div className="flex gap-4 items-start p-6 bg-white rounded-xl border shadow-sm border-slate-200">
              <div className="flex justify-center items-center w-10 h-10 text-2xl bg-indigo-100 rounded-full shrink-0">
                🤖
              </div>
              <div>
                <h3 className="mb-1 text-lg font-semibold text-slate-700">
                  AI Assistant
                </h3>
                <p className="text-slate-500">
                  Keep your favorite AI chat always accessible on your desktop.
                </p>
              </div>
            </div>
          </div>
        </div>
      </section>

      <div className="px-4 py-20 bg-white">
        <div className="mx-auto max-w-8xl">
          <h2 className="mb-12 text-3xl font-bold text-center text-slate-800">
            Customize your desktop with interactive widgets
          </h2>
          <div className="overflow-hidden mx-auto max-w-4xl rounded-xl shadow-2xl bg-slate-800">
            <video
              className="object-cover w-full"
              autoPlay
              loop
              muted
              playsInline
            >
              <source src="/interactive-widget.webm" type="video/webm" />
              Your browser does not support the video tag.
            </video>
          </div>
        </div>
      </div>

      {/* Pricing Section */}
      <section id="pricing" className="px-4 py-20 bg-white">
        <div className="mx-auto max-w-8xl">
          <h2 className="mb-4 text-3xl font-bold text-center text-slate-800">
            Simple Pricing
          </h2>
          <p className="mx-auto mb-12 max-w-2xl text-center text-slate-600">
            Start free, upgrade when you're ready. Early access supporters get
            special benefits.
          </p>
          <div className="p-6 mx-auto max-w-md bg-white rounded-xl border-2 border-indigo-500 shadow-lg">
            <h3 className="mb-2 text-xl font-semibold text-indigo-600">
              Early Access
            </h3>
            <p className="mb-4 text-3xl font-bold">
              $10{" "}
              <span className="text-lg font-normal text-slate-500">
                / one-time
              </span>
            </p>
            <ul className="mb-8 space-y-4">
              <li className="flex items-center text-slate-600">
                <IconCheck className="mr-2 w-5 h-5 text-emerald-500" />
                <span>All widget features unlocked</span>
              </li>
              <li className="flex items-center text-slate-600">
                <IconCheck className="mr-2 w-5 h-5 text-emerald-500" />
                <span>Priority support & feature requests</span>
              </li>
              <li className="flex items-center text-slate-600">
                <IconCheck className="mr-2 w-5 h-5 text-emerald-500" />
                <span>Free updates during early access</span>
              </li>
            </ul>
            <button
              onClick={handleDownload}
              className="block px-8 py-3 font-bold text-center text-white bg-indigo-600 rounded-lg transition-all hover:bg-indigo-700 hover:scale-105"
            >
              Download for free
            </button>
          </div>
        </div>
      </section>

      {/* Footer */}
      <footer className="px-4 py-16 text-center bg-slate-50">
        <div className="mx-auto max-w-8xl">
          <h2 className="mb-4 text-3xl font-bold text-slate-900">
            Ready to Get Started?
          </h2>
          <p className="mx-auto mb-8 max-w-2xl text-slate-600">
            Transform your desktop experience today. Try HoverPane for free.
          </p>
          <button
            onClick={handleDownload}
            className="inline-block px-8 py-3 font-bold text-white bg-indigo-600 rounded-lg transition-all hover:bg-indigo-700 hover:scale-105"
          >
            Download for macOS
          </button>
          <p className="mt-8 text-sm text-slate-400">
            Made by{" "}
            <a
              href="https://jeremyarde.com"
              target="_blank"
              rel="noopener noreferrer"
              className="text-slate-800 hover:text-slate-100"
            >
              Jeremy
            </a>
            <span className="ml-2">jere.arde@gmail.com</span>
          </p>
          <p className="mt-2 text-xs text-slate-500">
            © {new Date().getFullYear()} HoverPane. All rights reserved.
          </p>
        </div>
      </footer>
    </div>
  );
};

export default LandingPage;
