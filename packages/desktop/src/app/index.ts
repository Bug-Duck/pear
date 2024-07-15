import { BrowserWindow, app } from 'electron'
import { resolve } from 'path'
import { distPath, registerAppProtocol } from './protocol'
import { registerRemote } from './remote'

const handleReady = async () => {
  const window = new BrowserWindow({
    width: 800,
    height: 600,
    webPreferences: {
      devTools: !app.isPackaged,
      preload: resolve(distPath, 'app', 'preload.cjs'),
    },
  })


  window.loadURL(!app.isPackaged ? 'http://localhost:5173/' : 'app://./')

  if (!app.isPackaged) window.webContents.openDevTools()

  app.on('window-all-closed', () => app.quit())
}

const bootstrap = async () => {
  registerAppProtocol()
  registerRemote()

  await app.whenReady()

  await handleReady()
}

bootstrap()
