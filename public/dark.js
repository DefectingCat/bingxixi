const darkThemeMq = window.matchMedia("(prefers-color-scheme: dark)");

const darkMode =
  (!("theme" in localStorage) && darkThemeMq.matches) ||
  localStorage.theme === "dark";

document.body.setAttribute("arco-theme", darkMode ? "dark" : "light");
document.documentElement.setAttribute(
  "arco-theme",
  darkMode ? "dark" : "light",
);
