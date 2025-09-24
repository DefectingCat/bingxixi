const darkMode =
  (!("theme" in localStorage) &&
    window.matchMedia("(prefers-color-scheme: dark)").matches) ||
  localStorage.theme === "dark";

document.body.setAttribute("arco-theme", darkMode ? "dark" : "light");
document.documentElement.setAttribute(
  "arco-theme",
  darkMode ? "dark" : "light",
);
