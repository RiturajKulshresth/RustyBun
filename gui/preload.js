const { ipcRenderer } = require("electron");

window.addEventListener("DOMContentLoaded", () => {
    ipcRenderer.send("hide-menu");
    ipcRenderer.on("zoom-in", () => {
        // Implement zoom in functionality
    });

    ipcRenderer.on("zoom-out", () => {
        // Implement zoom out functionality
    });

    ipcRenderer.on("actual-size", () => {
        // Implement actual size functionality
    });
});
