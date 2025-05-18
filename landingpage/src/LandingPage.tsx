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
    <div className="bg-indigo-400">
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
          <h2 className="section-title">Why use HoverPane?</h2>
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
      <div className="container bg-white">
        <h2 className="section-title pt-12">Customize your desktop</h2>
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
            <label className="made-by-email">jere.arde@gmail.com</label>
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
