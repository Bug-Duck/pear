import { app, type WebFrameMain, ipcMain, BrowserWindow } from 'electron'
import { showDialog } from './utils'

export const validateSender = (frame: WebFrameMain) => {
  if (app.isPackaged && new URL(frame.url).protocol != 'app:') {
    throw new Error('Invalid sender')
  }
}

export const registerRemote = () => {
  ipcMain.handle('show-dialog', (event, message: string) => {
    validateSender(event.senderFrame)
    return showDialog(message)
  })

  ipcMain.handle('window-control', (event, op: 'minimize' | 'maximize' | 'close') => {
    validateSender(event.senderFrame)
    const window = BrowserWindow.fromWebContents(event.sender)
    if (!window) throw new TypeError('sender is not a browser window')

    if (op == 'minimize') window.minimize()
    else if (op == 'maximize') window.maximize()
    else if (op == 'close') window.close()
  })

  ipcMain.handle('window-state', (event) => {
    validateSender(event.senderFrame)
    const window = BrowserWindow.fromWebContents(event.sender)
    if (!window) throw new TypeError('sender is not a browser window')

    return window.isMaximized() ? 'maximized' : window.isMinimized() ? 'minimized' : 'none'
  })
}