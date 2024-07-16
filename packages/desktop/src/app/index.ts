import { BrowserWindow, app, Menu, nativeTheme } from 'electron'
import { resolve } from 'path'
import { distPath, registerAppProtocol } from './protocol'
import { registerRemote } from './remote'

const handleReady = async () => {
  const window = new BrowserWindow({
    width: 800,
    height: 600,
    minHeight: 600,
    minWidth: 800,
    frame: false,
    titleBarStyle: 'hidden',
    titleBarOverlay: {
      color: '#121212',
      symbolColor: '#f0f0f0',
    },
    show: false,
    backgroundColor: '#121212',
    webPreferences: {
      devTools: !app.isPackaged,
      preload: resolve(distPath, 'app', 'preload.cjs'),
    },
  })

  Menu.setApplicationMenu(null)

  window.loadURL(!app.isPackaged ? 'http://localhost:5173/' : 'app://./')

  window.once('ready-to-show', () => window.show())

  if (!app.isPackaged) window.webContents.openDevTools()


  app.on('window-all-closed', () => app.quit())
}

const bootstrap = async () => {
  if (!app.requestSingleInstanceLock()) app.quit()

  registerAppProtocol()
  registerRemote()

  await app.whenReady()

  nativeTheme.themeSource = 'dark'

  await handleReady()
}

bootstrap()
