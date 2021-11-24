process.env.TZ = "UTC";

module.exports = {
  collectCoverageFrom: [
    "src/**/*.{ts,tsx}",
    "!src/index.tsx",
    "!src/App.tsx",
    "!src/setupTests.ts",
  ],
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
      tsconfig: "tsconfig.json",
    },
  },
  moduleFileExtensions: ["json", "ts", "tsx", "js", "jsx"],
  setupFilesAfterEnv: ["./jest.setup.ts"],
  testEnvironment: require.resolve(`jest-environment-node`),
  testMatch: ["**/**/test/**/*.(spec|test).ts?(x)"],
  transform: {
    "^.+\\.tsx?$": require.resolve("ts-jest"),
  },
  transformIgnorePatterns: ["/.pnp.cjs$"],
  verbose: true,
};
