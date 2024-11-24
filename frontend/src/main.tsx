// import React from 'react'
import ReactDOM from "react-dom/client";
import { BrowserRouter, Routes, Route } from "react-router";
import App from "./App.tsx";
import Testbed from "./pages/Testbed.tsx";
import "./index.css";

ReactDOM.createRoot(document.getElementById("root")!).render(
  // <React.StrictMode>
  <BrowserRouter>
    <Routes>
      <Route index element={<App />} />
      <Route path="testbed" element={<Testbed />} />
    </Routes>
  </BrowserRouter>
  // </React.StrictMode>,
);
