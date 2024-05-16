// main.js
const { app, BrowserWindow, ipcMain, Menu } = require("electron");
const path = require("path");
const { spawn } = require('child_process');
const fs = require("fs");

let menu = null;
let mainWindow;

function createWindow() {
    mainWindow = new BrowserWindow({
        width: 400,
        height: 600,
        // icon: path.join(__dirname, './assets/icons/png/1024x1024.png'),
        webPreferences: {
            nodeIntegration: true,
            contextIsolation: false,
            preload: path.join(__dirname, "preload.js"),
        },
    });

    mainWindow.loadFile("index.html");

    // Open DevTools
    // mainWindow.webContents.openDevTools();

    // console.log("Spawning clip_cpy process...");
    // // Set up clip_cpy process
    // const clip_cpy = spawn('/opt/gui/clip_cpy');

    // // Listen for any errors
    // clip_cpy.on('error', (err) => {
    //     console.error('Failed to start clip_cpy:', err);
    //     console.log("Cat")
    //     // You may want to handle this error more gracefully, like showing an error dialog
    // });

    // // Listen for clip_cpy output
    // clip_cpy.stdout.on('data', (data) => {
    //     console.log(`clip_cpy output: ${data}`);
    //     console.log("Cat")

    //     // Handle the output as needed
    // });

    // Read clipboard.json and send its data to the renderer process
    // const clipboardFilePath = path.join(__dirname, "clipboard.json");
    const clipboardFilePath = "/home/ritu/Documents/RustyBun/clipboard.json";
    fs.readFile(clipboardFilePath, "utf-8", (err, data) => {
        if (err) {
            console.error("Error reading clipboard.json:", err);
            mainWindow.webContents.send("clipboard-data", { error: err.message });
        } else {
            mainWindow.webContents.send("clipboard-data", { data });
        }
    });

    fs.watchFile(clipboardFilePath, (curr, prev) => {
        fs.readFile(clipboardFilePath, "utf-8", (err, data) => {
            if (err) {
                console.error("Error reading clipboard.json:", err);
            } else {
                mainWindow.webContents.send("clipboard-data", { data });
            }
        });
    });
}

app.whenReady().then(() => {
    createWindow();

    // Other initialization code can go here

    // Optionally, handle any remaining setup after window creation
    app.on("activate", () => {
        if (BrowserWindow.getAllWindows().length === 0) {
            createWindow();
        }
    });

    ipcMain.on("show-menu", () => {
        if (!menu) {
            const menuTemplate = [
                {
                    label: "View",
                    submenu: [
                        {
                            label: "Zoom In",
                            accelerator: "CmdOrCtrl+Plus",
                            click: () => mainWindow.webContents.send("zoom-in"),
                        },
                        {
                            label: "Zoom Out",
                            accelerator: "CmdOrCtrl+-",
                            click: () => mainWindow.webContents.send("zoom-out"),
                        },
                        {
                            label: "Actual Size",
                            accelerator: "CmdOrCtrl+0",
                            click: () => mainWindow.webContents.send("actual-size"),
                        },
                    ],
                },
            ];
            menu = Menu.buildFromTemplate(menuTemplate);
        }

        mainWindow.setMenu(menu);
    });

    ipcMain.on("hide-menu", () => {
        if (menu) {
            mainWindow.setMenu(null);
            menu = null;
        }
    });

    // Add other event listeners and setup code here
});

app.on("window-all-closed", () => {
    if (process.platform !== "darwin") {
        app.quit();
    }
});
