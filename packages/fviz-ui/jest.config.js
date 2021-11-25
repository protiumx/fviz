process.env.TZ = "UTC";

module.exports = {
  collectCoverageFrom: [
    "src/**/*.{ts,tsx}",
    "!src/index.tsx",
    "!src/App.tsx",
    "!src/api/axios.ts",
  ],
  coverageDirectory: "coverage",
  coverageReporters: process.env.GITHUB_ACTIONS
    ? ["lcovonly", "text"]
    : ["html", "lcov", "text"],
  coverageThreshold: {
    global: {
      branches: 30,
      functions: 30,
      lines: 30,
      statements: 30,
    },
  },
  globals: {
    "ts-jest": {
      babel: true,
      tsconfig: "tsconfig.json",
    },
  },
  moduleNameMapper: {
    "^$/(.*)$": "<rootDir>/src/$1",
  },
  moduleFileExtensions: ["json", "ts", "tsx", "js", "jsx"],
  setupFilesAfterEnv: ["./jest.setup.js"],
  testEnvironment: "node",
  testMatch: ["**/*.(spec|test).ts?(x)"],
  transform: {
    "^.+\\.jsx?$": "babel-jest",
    "^.+\\.tsx?$": "ts-jest",
  },
  transformIgnorePatterns: ["/.pnp.cjs$", "/node_modules/", "/dist"],
  verbose: true,
};
