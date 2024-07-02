"use client";

import { useState, useEffect } from "react";

export default function AnotherPage() {
  const [windowHeight, setWindowHeight] = useState<number | undefined>(
    undefined
  );
  // Ensure that the window object is available before calling browser APIs.
  useEffect(() => {
    setWindowHeight(window.innerHeight);
  }, []);

  return (
    // Reused styling.
    <div className="font-sans grid grid-rows-[20px_1fr_20px] items-center justify-items-center min-h-screen p-8 pb-20 gap-16 sm:p-20">
      <main className="flex flex-col gap-8 row-start-2 items-center sm:items-start">
        {/* Our content */}
        <h1>Another Page</h1>
        <p>
          This is a another page calling browser APIs
          {windowHeight ? `: ${windowHeight}` : ""}
        </p>
      </main>
    </div>
  );
}
