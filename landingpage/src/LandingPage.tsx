import React from "react";
import "./styles.css";
import {
  paidEarlyAccessLink,
  IconMinimalist,
  IconPin,
  IconRefresh,
  IconCode,
  IconCheck,
} from "./constants";

const LandingPage: React.FC = () => {
  return (
    <div className="min-h-screen bg-slate-50">
      {/* Hero Section */}
      <header className="relative overflow-hidden bg-gradient-to-br from-indigo-600 to-indigo-400 py-24 px-4 text-center md:py-36">
        <div className="absolute top-0 left-0 h-48 w-64 -translate-x-1/3 -translate-y-1/3 rounded-xl bg-indigo-200 opacity-30"></div>
        <div className="absolute bottom-0 right-0 h-64 w-80 translate-x-1/4 translate-y-1/4 rounded-xl bg-indigo-200 opacity-30"></div>
        <div className="relative z-10">
          <h1 className="mb-4 text-4xl font-bold text-white md:text-6xl">
            HoverPane
          </h1>
          <p className="mx-auto mb-8 max-w-2xl text-lg text-slate-100 md:text-xl">
            Transform any website into a sleek desktop widget in seconds. Keep
            your favorite content always visible and easily accessible.
          </p>
          <a
            href={paidEarlyAccessLink}
            className="inline-block rounded-lg bg-white px-8 py-3 font-bold text-indigo-600 transition-all hover:scale-105 hover:bg-slate-100"
          >
            Get Started
          </a>
          <div className="mx-auto mt-12 max-w-4xl overflow-hidden rounded-xl bg-slate-800 shadow-2xl">
            <video
              className="aspect-video w-full object-cover"
              autoPlay
              loop
              muted
              playsInline
            >
              <source src="/tools-demo.webm" type="video/webm" />
              Your browser does not support the video tag.
            </video>
          </div>
        </div>
      </header>

      {/* Features Section */}
      <section id="features" className="bg-slate-50 py-20 px-4">
        <div className="mx-auto max-w-8xl">
          <h2 className="mb-12 text-center text-3xl font-bold text-slate-800">
            Why use HoverPane?
          </h2>
          <div className="grid gap-8 md:grid-cols-2 md:gap-10">
            <div className="rounded-xl border border-slate-200 bg-white p-6 text-center shadow-sm transition-shadow hover:shadow-md">
              <div className="mx-auto mb-4 flex h-12 w-12 items-center justify-center rounded-full bg-indigo-100">
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
            <div className="rounded-xl border border-slate-200 bg-white p-6 text-center shadow-sm transition-shadow hover:shadow-md">
              <div className="mx-auto mb-4 flex h-12 w-12 items-center justify-center rounded-full bg-sky-100">
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
            <div className="rounded-xl border border-slate-200 bg-white p-6 text-center shadow-sm transition-shadow hover:shadow-md">
              <div className="mx-auto mb-4 flex h-12 w-12 items-center justify-center rounded-full bg-amber-100">
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
            <div className="rounded-xl border border-slate-200 bg-white p-6 text-center shadow-sm transition-shadow hover:shadow-md">
              <div className="mx-auto mb-4 flex h-12 w-12 items-center justify-center rounded-full bg-emerald-100">
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
      <section id="how-it-works" className="bg-white py-20 px-4">
        <div className="mx-auto max-w-8xl">
          <h2 className="mb-12 text-center text-3xl font-bold text-slate-800">
            Simple Setup
          </h2>
          <div className="mx-auto grid max-w-4xl gap-8 md:grid-cols-3 md:gap-12">
            <div className="text-center">
              <div className="mx-auto mb-4 flex h-12 w-12 items-center justify-center rounded-full bg-indigo-600 text-xl font-bold text-white shadow-md">
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
              <div className="mx-auto mb-4 flex h-12 w-12 items-center justify-center rounded-full bg-indigo-600 text-xl font-bold text-white shadow-md">
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
              <div className="mx-auto mb-4 flex h-12 w-12 items-center justify-center rounded-full bg-indigo-600 text-xl font-bold text-white shadow-md">
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
      <section id="use-cases" className="bg-slate-50 py-20 px-4">
        <div className="mx-auto max-w-8xl">
          <h2 className="mb-12 text-center text-3xl font-bold text-slate-800">
            Endless Possibilities
          </h2>
          <div className="grid gap-6 md:grid-cols-2">
            <div className="flex items-start gap-4 rounded-xl border border-slate-200 bg-white p-6 shadow-sm">
              <div className="flex h-10 w-10 shrink-0 items-center justify-center rounded-full bg-indigo-100 text-2xl">
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
            <div className="flex items-start gap-4 rounded-xl border border-slate-200 bg-white p-6 shadow-sm">
              <div className="flex h-10 w-10 shrink-0 items-center justify-center rounded-full bg-indigo-100 text-2xl">
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
            <div className="flex items-start gap-4 rounded-xl border border-slate-200 bg-white p-6 shadow-sm">
              <div className="flex h-10 w-10 shrink-0 items-center justify-center rounded-full bg-indigo-100 text-2xl">
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
            <div className="flex items-start gap-4 rounded-xl border border-slate-200 bg-white p-6 shadow-sm">
              <div className="flex h-10 w-10 shrink-0 items-center justify-center rounded-full bg-indigo-100 text-2xl">
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
            <div className="flex items-start gap-4 rounded-xl border border-slate-200 bg-white p-6 shadow-sm">
              <div className="flex h-10 w-10 shrink-0 items-center justify-center rounded-full bg-indigo-100 text-2xl">
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

      <div className="bg-white py-20 px-4">
        <div className="mx-auto max-w-8xl">
          <h2 className="mb-12 text-center text-3xl font-bold text-slate-800">
            Customize your desktop
          </h2>
          <div className="mx-auto max-w-4xl overflow-hidden rounded-xl bg-slate-800 shadow-2xl">
            <video
              className="aspect-video w-full object-cover"
              autoPlay
              loop
              muted
              playsInline
            >
              <source src="/hoverpane-demo.webm" type="video/webm" />
              Your browser does not support the video tag.
            </video>
          </div>
        </div>
      </div>

      {/* Pricing Section */}
      <section id="pricing" className="bg-white py-20 px-4">
        <div className="mx-auto max-w-8xl">
          <h2 className="mb-4 text-center text-3xl font-bold text-slate-800">
            Simple Pricing
          </h2>
          <p className="mx-auto mb-12 max-w-2xl text-center text-slate-600">
            Start free, upgrade when you're ready. Early access supporters get
            special benefits.
          </p>
          <div className="mx-auto max-w-md rounded-xl border-2 border-indigo-500 bg-white p-6 shadow-lg">
            <h3 className="mb-2 text-xl font-semibold text-indigo-600">
              Early Access
            </h3>
            <p className="mb-4 text-3xl font-bold">
              $10{" "}
              <span className="text-lg font-normal text-slate-500">
                / one-time
              </span>
            </p>
            <p className="mb-6 text-slate-600">
              Join our early community and help shape the future of HoverPane.
            </p>
            <ul className="mb-8 space-y-4">
              <li className="flex items-center text-slate-600">
                <IconCheck className="mr-2 h-5 w-5 text-emerald-500" />
                <span>Create up to 2 widgets</span>
              </li>
              <li className="flex items-center text-slate-600">
                <IconCheck className="mr-2 h-5 w-5 text-emerald-500" />
                <span>All widget features unlocked</span>
              </li>
              <li className="flex items-center text-slate-600">
                <IconCheck className="mr-2 h-5 w-5 text-emerald-500" />
                <span>Priority support & feature requests</span>
              </li>
              <li className="flex items-center text-slate-600">
                <IconCheck className="mr-2 h-5 w-5 text-emerald-500" />
                <span>Free updates during early access</span>
              </li>
            </ul>
            <a
              href={paidEarlyAccessLink}
              className="block rounded-lg bg-indigo-600 px-8 py-3 text-center font-bold text-white transition-all hover:bg-indigo-700 hover:scale-105"
            >
              Join Early Access
            </a>
          </div>
        </div>
      </section>

      {/* Footer */}
      <footer className="bg-slate-50 py-16 px-4 text-center">
        <div className="mx-auto max-w-8xl">
          <h2 className="mb-4 text-3xl font-bold text-slate-900">
            Ready to Get Started?
          </h2>
          <p className="mx-auto mb-8 max-w-2xl text-slate-600">
            Transform your desktop experience today. Try HoverPane for free.
          </p>
          <a
            href={paidEarlyAccessLink}
            className="inline-block rounded-lg bg-indigo-600 px-8 py-3 font-bold text-white transition-all hover:bg-indigo-700 hover:scale-105"
          >
            Download for macOS
          </a>
          <p className="mt-8 text-sm text-slate-400">
            Made by{" "}
            <a
              href="https://jeremyarde.com"
              target="_blank"
              rel="noopener noreferrer"
              className="text-slate-200 hover:text-slate-100"
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
