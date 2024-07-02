"use client";

import { useState } from "react";

export default function SubPage() {
  const [counter, setCounter] = useState(0);
  return (
    // Reused styling.
    <div className="font-sans grid grid-rows-[20px_1fr_20px] items-center justify-items-center min-h-screen p-8 pb-20 gap-16 sm:p-20">
      <main className="flex flex-col gap-8 row-start-2 items-center sm:items-start">
        {/* Our content */}
        <h1>SubPage</h1>
        <p>This is a subpage with interactivity: {counter}</p>
        <button onClick={() => setCounter((prev) => prev + 1)}>
          Increment
        </button>
      </main>
    </div>
  );
}
