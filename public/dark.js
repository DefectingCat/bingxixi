const darkThemeMq = window.matchMedia("(prefers-color-scheme: dark)");

const darkMode =
  (!("theme" in localStorage) && darkThemeMq.matches) ||
  localStorage.theme === "dark";

document.body.setAttribute("theme-mode", darkMode ? "dark" : "light");
document.documentElement.setAttribute(
  "theme-mode",
  darkMode ? "dark" : "light",
);
