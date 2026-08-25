import React from "react";
import ReactDOM from "react-dom/client";

// Kendi barindirdigimiz Inter (Google Fonts CDN'i degil): uygulama cevrimdisi
// da acilmali ve CSP disari baglanti izni vermiyor.
// latin-ext alt kumesi Turkce icin sart: i, g, s harfleri orada.
import "@fontsource/inter/latin-400.css";
import "@fontsource/inter/latin-ext-400.css";
import "@fontsource/inter/latin-500.css";
import "@fontsource/inter/latin-ext-500.css";
import "@fontsource/inter/latin-600.css";
import "@fontsource/inter/latin-ext-600.css";
import "@fontsource/inter/latin-700.css";
import "@fontsource/inter/latin-ext-700.css";

import "./styles/tokens.css";
import "./styles/base.css";
import "./styles/app.css";

import App from "./App";

ReactDOM.createRoot(document.getElementById("root") as HTMLElement).render(
  <React.StrictMode>
    <App />
  </React.StrictMode>
);
