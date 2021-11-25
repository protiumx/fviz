process.env.TZ = "UTC";

module.exports = {
  collectCoverageFrom: ["src/**/*.{ts,tsx}", "!src/index.tsx", "!src/App.tsx"],
  coverageDirectory: "coverage",
  coverageReporters: process.env.GITHUB_ACTIONS
    ? ["lcovonly", "text"]
    : ["html", "lcov", "text"],
  coverageThreshold: {
    global: {
      branches: 100,
      functions: 100,
      lines: 100,
      statements: 100,
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
