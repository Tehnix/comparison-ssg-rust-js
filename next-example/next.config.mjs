/** @type {import('next').NextConfig} */
const nextConfig = {
  output: "export", // <-- This line here
  // Generate index.html files for each route in the output.
  trailingSlash: true, // <-- This line here
  // Optional: Enable experimental React Compiler support.
  experimental: {
    reactCompiler: true, // <-- This line here
  },
};

export default nextConfig;
