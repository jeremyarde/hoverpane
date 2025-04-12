import React from "react";
import "./styles.css";

// Placeholder Icons (replace with actual icons or component library)
// const IconBolt = () => (
//   <svg
//     className="w-8 h-8 text-indigo-600"
//     fill="none"
//     stroke="currentColor"
//     viewBox="0 0 24 24"
//     xmlns="http://www.w3.org/2000/svg"
//   >
//     <path
//       strokeLinecap="round"
//       strokeLinejoin="round"
//       strokeWidth="2"
//       d="M13 10V3L4 14h7v7l9-11h-7z"
//     ></path>
//   </svg>
// );
const paidEarlyAccessLink =
  "https://buy.polar.sh/polar_cl_CInamf8ulWDbVRoLyjWYWI5M8Vkzq2KTjLPDw2wUzQZ";

const IconCode = () => (
  <svg
    className="w-8 h-8 text-teal-400"
    fill="none"
    stroke="currentColor"
    viewBox="0 0 24 24"
    xmlns="http://www.w3.org/2000/svg"
  >
    <path
      strokeLinecap="round"
      strokeLinejoin="round"
      strokeWidth="2"
      d="M8 9l3 3-3 3m5 0h3M5 20h14a2 2 0 002-2V6a2 2 0 00-2-2H5a2 2 0 00-2 2v12a2 2 0 002 2z"
    />
  </svg>
);
// const IconCog = () => (
//   <svg
//     className="w-8 h-8 text-rose-600"
//     fill="none"
//     stroke="currentColor"
//     viewBox="0 0 24 24"
//     xmlns="http://www.w3.org/2000/svg"
//   >
//     <path
//       strokeLinecap="round"
//       strokeLinejoin="round"
//       strokeWidth="2"
//       d="M10.325 4.317c.426-1.756 2.924-1.756 3.35 0a1.724 1.724 0 002.573 1.066c1.543-.94 3.31.826 2.37 2.37a1.724 1.724 0 001.065 2.572c1.756.426 1.756 2.924 0 3.35a1.724 1.724 0 00-1.066 2.573c.94 1.543-.826 3.31-2.37 2.37a1.724 1.724 0 00-2.572 1.065c-.426 1.756-2.924 1.756-3.35 0a1.724 1.724 0 00-2.573-1.066c-1.543.94-3.31-.826-2.37-2.37a1.724 1.724 0 00-1.065-2.572c-1.756-.426-1.756-2.924 0-3.35a1.724 1.724 0 001.066-2.573c-.94-1.543.826-3.31 2.37-2.37.996.608 2.296.07 2.572-1.065z"
//     ></path>
//     <path
//       strokeLinecap="round"
//       strokeLinejoin="round"
//       strokeWidth="2"
//       d="M15 12a3 3 0 11-6 0 3 3 0 016 0z"
//     ></path>
//   </svg>
// );
// const IconEye = () => (
//   <svg
//     className="w-8 h-8 text-sky-600"
//     fill="none"
//     stroke="currentColor"
//     viewBox="0 0 24 24"
//     xmlns="http://www.w3.org/2000/svg"
//   >
//     <path
//       strokeLinecap="round"
//       strokeLinejoin="round"
//       strokeWidth={2}
//       d="M15 12a3 3 0 11-6 0 3 3 0 016 0z"
//     />
//     <path
//       strokeLinecap="round"
//       strokeLinejoin="round"
//       strokeWidth={2}
//       d="M2.458 12C3.732 7.943 7.523 5 12 5c4.478 0 8.268 2.943 9.542 7-1.274 4.057-5.064 7-9.542 7-4.477 0-8.268-2.943-9.542-7z"
//     />
//   </svg>
// );
const IconRefresh = () => (
  <svg
    className="w-8 h-8 text-amber-300"
    fill="none"
    stroke="currentColor"
    viewBox="0 0 24 24"
    xmlns="http://www.w3.org/2000/svg"
  >
    <path
      strokeLinecap="round"
      strokeLinejoin="round"
      strokeWidth={2}
      d="M4 4v5h.582m15.356 2A8.001 8.001 0 004.582 9m0 0H9m11 11v-5h-.581m0 0a8.003 8.003 0 01-15.357-2m15.357 2H15"
    />
  </svg>
);
const IconCheck = () => (
  <svg
    className="w-5 h-5 text-teal-400 mr-2 flex-shrink-0"
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

// New Icons
const IconMinimalist = () => (
  <svg
    className="w-8 h-8 text-indigo-300"
    fill="none"
    stroke="currentColor"
    viewBox="0 0 24 24"
    xmlns="http://www.w3.org/2000/svg"
  >
    <path
      strokeLinecap="round"
      strokeLinejoin="round"
      strokeWidth="2"
      d="M4 6a2 2 0 012-2h2a2 2 0 012 2v2a2 2 0 01-2 2H6a2 2 0 01-2-2V6zM14 6a2 2 0 012-2h2a2 2 0 012 2v2a2 2 0 01-2 2h-2a2 2 0 01-2-2V6zM4 16a2 2 0 012-2h2a2 2 0 012 2v2a2 2 0 01-2 2H6a2 2 0 01-2-2v-2zM14 16a2 2 0 012-2h2a2 2 0 012 2v2a2 2 0 01-2 2h-2a2 2 0 01-2-2v-2z"
    />
  </svg>
);

const IconPin = () => (
  <svg
    className="w-8 h-8 text-sky-300"
    fill="none"
    stroke="currentColor"
    viewBox="0 0 24 24"
    xmlns="http://www.w3.org/2000/svg"
  >
    <path
      strokeLinecap="round"
      strokeLinejoin="round"
      strokeWidth="2"
      d="M5 5a2 2 0 012-2h10a2 2 0 012 2v16l-7-3.5L5 21V5z"
    />
  </svg>
);

const LandingPage: React.FC = () => {
  return (
    <div>
      {/* Navbar */}
      <nav className="navbar">
        <div className="container navbar-content">
          <span className="logo">HoverPane</span>
          <div className="nav-links">
            <a href="#features" className="nav-link">
              Features
            </a>
            <a href="#how-it-works" className="nav-link">
              How It Works
            </a>
            <a href="#use-cases" className="nav-link">
              Ideas
            </a>
            <a href="#pricing" className="nav-link">
              Pricing
            </a>
            <a href={paidEarlyAccessLink} className="download-button">
              Download
            </a>
          </div>
        </div>
      </nav>

      {/* Hero Section */}
      <header className="hero">
        <div className="hero-shape hero-shape-1"></div>
        <div className="hero-shape hero-shape-2"></div>
        <div className="hero-content">
          <h1 className="hero-title">HoverPane</h1>
          <p className="hero-subtitle">
            Transform any website into a sleek desktop widget in seconds. Keep
            your favorite content always visible and easily accessible.
          </p>
          <a href={paidEarlyAccessLink} className="hero-button">
            Get Started
          </a>
          <div className="demo-video-container">
            <video className="demo-video" autoPlay loop muted playsInline>
              <source src="/tools-demo.webm" type="video/webm" />
              Your browser does not support the video tag.
            </video>
          </div>
        </div>
      </header>

      {/* Features Section */}
      <section id="features" className="features">
        <div className="container">
          <h2 className="section-title">Why Choose HoverPane</h2>
          <div className="features-grid">
            <div className="feature-card">
              <div className="feature-icon minimalist">
                <IconMinimalist />
              </div>
              <h3 className="feature-title">Clean & Focused</h3>
              <p className="feature-description">
                Experience your content without distractions. No ads, no
                toolbars, just what matters to you.
              </p>
            </div>
            <div className="feature-card">
              <div className="feature-icon pin">
                <IconPin />
              </div>
              <h3 className="feature-title">Always Visible</h3>
              <p className="feature-description">
                Keep important information at your fingertips. Your widgets stay
                on top of other windows, exactly where you need them.
              </p>
            </div>
            <div className="feature-card">
              <div className="feature-icon refresh">
                <IconRefresh />
              </div>
              <h3 className="feature-title">Stay Updated</h3>
              <p className="feature-description">
                Never miss an update. Set custom refresh intervals to keep your
                information current.
              </p>
            </div>
            <div className="feature-card">
              <div className="feature-icon code">
                <IconCode />
              </div>
              <h3 className="feature-title">Customizable</h3>
              <p className="feature-description">
                Create your perfect widget. Use any website or build your own
                with HTML, CSS, and JavaScript.
              </p>
            </div>
          </div>
        </div>
      </section>

      {/* How It Works Section */}
      <section id="how-it-works" className="how-it-works">
        <div className="container">
          <h2 className="section-title">Simple Setup</h2>
          <div className="steps-grid">
            <div className="step">
              <div className="step-number">1</div>
              <h3 className="step-title">Choose Your Source</h3>
              <p className="step-description">
                Enter a website URL or paste your custom HTML code.
              </p>
            </div>
            <div className="step">
              <div className="step-number">2</div>
              <h3 className="step-title">Customize</h3>
              <p className="step-description">
                Set refresh rates, adjust size, and configure other settings to
                match your needs.
              </p>
            </div>
            <div className="step">
              <div className="step-number">3</div>
              <h3 className="step-title">Enjoy</h3>
              <p className="step-description">
                Your widget is ready! Place it anywhere on your desktop and
                start using it right away.
              </p>
            </div>
          </div>
        </div>
      </section>

      {/* Use Cases Section */}
      <section id="use-cases" className="use-cases">
        <div className="container">
          <h2 className="section-title">Endless Possibilities</h2>

          <div className="use-cases-grid">
            <div className="use-case-card">
              <div className="use-case-icon">
                <span>📈</span>
              </div>
              <div className="use-case-content">
                <h3 className="use-case-title">Market Watch</h3>
                <p className="use-case-description">
                  Monitor stocks, crypto, or market trends in real-time.
                </p>
              </div>
            </div>
            <div className="use-case-card">
              <div className="use-case-icon">
                <span>🌤️</span>
              </div>
              <div className="use-case-content">
                <h3 className="use-case-title">Weather Updates</h3>
                <p className="use-case-description">
                  Keep an eye on current conditions and forecasts.
                </p>
              </div>
            </div>
            <div className="use-case-card">
              <div className="use-case-icon">
                <span>📅</span>
              </div>
              <div className="use-case-content">
                <h3 className="use-case-title">Calendar View</h3>
                <p className="use-case-description">
                  Access your schedule without opening your browser.
                </p>
              </div>
            </div>
            <div className="use-case-card">
              <div className="use-case-icon">
                <span>⚡</span>
              </div>
              <div className="use-case-content">
                <h3 className="use-case-title">Quick Tools</h3>
                <p className="use-case-description">
                  Build custom utilities, API dashboards, or system monitors.
                </p>
              </div>
            </div>
            <div className="use-case-card">
              <div className="use-case-icon">
                <span>🤖</span>
              </div>
              <div className="use-case-content">
                <h3 className="use-case-title">AI Assistant</h3>
                <p className="use-case-description">
                  Keep your favorite AI chat always accessible on your desktop.
                </p>
              </div>
            </div>
          </div>
        </div>
      </section>
      <div className="container">
        <h2 className="section-title">Customize your desktop</h2>
        <div className="demo-video-container">
          <video className="demo-video" autoPlay loop muted playsInline>
            <source src="/hoverpane-demo.webm" type="video/webm" />
            Your browser does not support the video tag.
          </video>
        </div>
      </div>

      {/* Pricing Section */}
      <section id="pricing" className="pricing">
        <div className="container">
          <h2 className="section-title">Simple Pricing</h2>
          <p className="pricing-description">
            Start free, upgrade when you're ready. Early access supporters get
            special benefits.
          </p>
          <div className="pricing-card">
            <h3 className="pricing-title">Early Access</h3>
            <p className="pricing-price">
              $10 <span className="pricing-period">/ one-time</span>
            </p>
            <p className="pricing-description">
              Join our early community and help shape the future of HoverPane.
            </p>
            <ul className="pricing-features">
              <li className="pricing-feature">
                <IconCheck />
                <span>Create up to 2 widgets</span>
              </li>
              <li className="pricing-feature">
                <IconCheck />
                <span>All widget features unlocked</span>
              </li>
              <li className="pricing-feature">
                <IconCheck />
                <span>Priority support & feature requests</span>
              </li>
              <li className="pricing-feature">
                <IconCheck />
                <span>Free updates during early access</span>
              </li>
            </ul>
            <a href={paidEarlyAccessLink} className="hero-button">
              Join Early Access
            </a>
          </div>
        </div>
      </section>

      {/* Footer */}
      <footer className="footer">
        <div className="container">
          <h2 className="footer-title">Ready to Get Started?</h2>
          <p className="footer-description">
            Transform your desktop experience today. Try HoverPane for free.
          </p>
          <a href={paidEarlyAccessLink} className="footer-button">
            Download for macOS
          </a>
          <p className="made-by">
            Made by{" "}
            <a
              href="https://jeremyarde.com"
              target="_blank"
              rel="noopener noreferrer"
              className="made-by-link"
            >
              Jeremy
            </a>
            <p className="made-by-email">jere.arde@gmail.com</p>
          </p>
          <p className="copyright">
            © {new Date().getFullYear()} HoverPane. All rights reserved.
          </p>
        </div>
      </footer>
    </div>
  );
};

export default LandingPage;
