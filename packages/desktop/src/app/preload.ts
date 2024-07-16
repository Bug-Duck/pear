import { contextBridge, ipcRenderer } from 'electron'

contextBridge.exposeInMainWorld('__pearApp', {
  windowControl: (op: 'minimize' | 'maximize' | 'close'): Promise<void> => ipcRenderer.invoke('window-control', op),
  getWindowState: (): Promise<'minimized' | 'maximized' | 'none'> => ipcRenderer.invoke('window-state')
})