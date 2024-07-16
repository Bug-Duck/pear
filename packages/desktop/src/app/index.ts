import { BrowserWindow, app, Menu } from 'electron'
import { resolve } from 'path'
import { distPath, registerAppProtocol } from './protocol'
import { registerRemote } from './remote'

const handleReady = async () => {

  const window = new BrowserWindow({
    width: 800,
    height: 600,
    minHeight: 600,
    minWidth: 800,
    frame: true,
    titleBarStyle: 'hidden',
  titleBarOverlay: {
    color: '#121212ff',
    symbolColor: '#f0f0f0'
  },
    webPreferences: {
      devTools: !app.isPackaged,
      preload: resolve(distPath, 'app', 'preload.cjs'),
    },
  })

  Menu.setApplicationMenu(null)


  window.loadURL(!app.isPackaged ? 'http://localhost:5173/' : 'app://./')

  if (!app.isPackaged) window.webContents.openDevTools()

  app.on('window-all-closed', () => app.quit())
}

const bootstrap = async () => {
  if (!app.requestSingleInstanceLock()) app.quit()

  registerAppProtocol()
  registerRemote()

  await app.whenReady()

  await handleReady()
}

bootstrap()
